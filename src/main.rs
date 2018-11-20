use std::env;
use std::io;
use std::process;
use std::collections::HashMap;


fn main() {
    // hash map will hold all variables and their values
    let variable_map: &mut HashMap<String, String> = &mut HashMap::new();
    // program control - also gets command line arg
    let command_line_arg = get_command_line_args();

    if command_line_arg.trim() == "-1" {
        println!("{}{}", "Number of arguments passed: ",  command_line_arg.len()-1);
        println!("Too many command line arguments! Enter one argument or none. Exiting program...");
    }
    else if command_line_arg.trim() == "" {
        main_user_input_loop(variable_map);
    }
    else {

    }
}

/// this function processes the one-time evaluation when program is called with a command line arg
fn main_one_time_eval(command_line_arg: String) {

}

/// main user input while loop
fn main_user_input_loop(varib_map: &mut HashMap<String, String>) {
    let mut user_input = "".to_string();

    while user_input.trim() != "quit" {
        user_input = "".to_string();
        // wait for user input
        io::stdin().read_line(&mut user_input)
            .expect("failed to read input");

        // clean all white space from string and any trailing or heading escape chars
        let clean_white_space_str = remove_all_white_space_from_string(user_input.trim());
        // check explicit division by 0
        if clean_white_space_str.contains("/0") {
            println!("{}", "You divided by 0! Enter a valid expression..");
            continue;
        }
        // checks if incoming string starts with lowercase letter - may be a variable assignment
        if clean_white_space_str.chars().nth(0).unwrap().is_ascii_lowercase() &&
            is_a_variable_assignment(&clean_white_space_str) &&
            !string_has_comparison(clean_white_space_str.to_owned()) {

            handle_variable_assignment(clean_white_space_str.to_owned(), varib_map);
            println!("{}", "variable stored");
            // go to next iteration and wait for user input..
            continue;
        }
        // replace all variable declarations with appropriate values ** this is the clean string
        let variables_replaced_str = replace_variable_references_with_value_strings(
            clean_white_space_str.to_owned(), varib_map);
        // check explicit division by 0 after replacing variable values...
        if variables_replaced_str.contains("/0") {
            println!("{}", "You divided by 0! Enter a valid expression..");
            continue;
        }
        // check if a comparison...
        let is_comparison = string_has_comparison(variables_replaced_str.to_owned());
        // if input string is a comparison, evaluate comparison..
        if is_comparison {
            println!("{}", handle_comparison_expression(variables_replaced_str.to_owned()));
        }
        // not a variable assignment and not a comparison - simply evaluate expression and print answer
        else if clean_white_space_str.trim() != "quit" {
            println!("{}", evaluate_clean_expression(&variables_replaced_str))
        }
    }
}

/// this function checks if an incoming expression is an assignment (ie contains a single '=')
fn is_a_variable_assignment(test_string: &str) -> bool {
    if test_string.find('=') == None {
        return false;
    }
    // check if next char is an '=' indicating a comparison '=='
    else {
        let first_equals_index = test_string.find('=').unwrap();
        if test_string.chars().nth(first_equals_index + 1).unwrap() == '=' {
            return false;
        }
    }
    true
}

#[test]
fn test_is_a_variable_assignment() {
    assert_eq!(false, is_a_variable_assignment("sa4rw45fw4c5e"));
    assert_eq!(false, is_a_variable_assignment("123rq3d==aefe4c"));
    assert_eq!(true, is_a_variable_assignment("a=23"));
    assert_eq!(true, is_a_variable_assignment("var=(2345+45)*2<=2"));
    // assert_eq!(true, is_a_variable_assignment(sda<=))
}

/// this functions assumes all expressions are evaluated and a single value is present on right side of '='
fn handle_variable_assignment(incoming_assignment_expr: String, var_map: &mut HashMap<String, String>) {
    let exp_value_vec = incoming_assignment_expr.split("=");
    let var_val_vec: Vec<&str> = exp_value_vec.collect();
    if var_map.contains_key(var_val_vec[0]) {
        *var_map.get_mut(var_val_vec[0]).unwrap() = var_val_vec[1].to_owned();
    }
    else {
        var_map.insert(var_val_vec[0].to_string(), var_val_vec[1].to_string());
    }
}

#[test]
fn test_handle_variable_declaration() {
    let mut sample_hash_map: HashMap<String, String> = HashMap::new();
    sample_hash_map.insert("variable0".to_string(), "7".to_string());
    sample_hash_map.insert("variable1".to_string(), "10".to_string());

    let mut test_hash_map0: HashMap<String, String> = HashMap::new();
    test_hash_map0.insert("variable0".to_string(), "700".to_string());
    test_hash_map0.insert("variable1".to_string(), "10".to_string());

    let mut test_hash_map1: HashMap<String, String> = HashMap::new();
    test_hash_map1.insert("variable0".to_string(), "700".to_string());
    test_hash_map1.insert("variable1".to_string(), "10".to_string());
    test_hash_map1.insert("xyz".to_string(), "200".to_string());


    let incoming_str = "variable0=700".to_string();
    handle_variable_assignment(incoming_str, &mut sample_hash_map);
    assert_eq!(test_hash_map0, sample_hash_map);
    let incoming_str0 = "xyz=200".to_string();
    handle_variable_assignment(incoming_str0, &mut sample_hash_map);
    assert_eq!(test_hash_map1, sample_hash_map);
}

// ------------------------------------------------------------

/// determines amount of command line arguments passed to program call
fn get_command_line_args() -> String {
    let command_line_args: Vec<_> = env::args().collect();

    if command_line_args.len() == 1 {
        println!("no command line args where passed, run regular program");
        return "".to_string()
    }
    else if command_line_args.len() == 2 {
        println!("a command line argument was passed, evaluate, print answer, and exit program");

        // if argument == '--help' ---> print contents of help file

        // else...
        return command_line_args[1].to_owned();

    }
    // simply exit program after giving error message
    return "-1".to_string()
}

// --------------- Utility Functions ------------------------------------------

fn string_has_comparison(in_string: String) -> bool {
    if in_string.contains("==") || in_string.contains("<=") || in_string.contains(">=")
        || in_string.contains("<") || in_string.contains(">") {
        return true
    }
    false
}

#[test]
fn test_string_has_comparison() {
    let string0 = String::from("423424<fw34f3");
    assert_eq!(true, string_has_comparison(string0));
    let string1 = String::from("423424<=fw34f3");
    assert_eq!(true, string_has_comparison(string1));
    let string2 = String::from("4==f3");
    assert_eq!(true, string_has_comparison(string2));
    let string3 = String::from("4234>w34f3");
    assert_eq!(true, string_has_comparison(string3));
    let string4 = String::from("4234=>w34f3");
    assert_eq!(true, string_has_comparison(string4));
    let string5 = String::from("4234=w34f3");
    assert_eq!(false, string_has_comparison(string5));
    let string6 = String::from("4234w34f3");
    assert_eq!(false, string_has_comparison(string6));
}

/// changes incoming expression string into a char vector
fn expression_string_to_char_vector(expr_string: &str) -> Vec<char> {
    let char_vec: Vec<_> = expr_string.chars().collect();
    return char_vec
}

#[test]
fn test_expression_string_to_vector() {
    let s = String::from("(3+4)*2/23");
    let empty_s = String::from("");
    let v = vec!['(', '3', '+', '4', ')', '*', '2', '/', '2', '3'];
    let empty_v: Vec<char> = vec![];

    assert_eq!(v, expression_string_to_char_vector(&s));
    assert_eq!(empty_v, expression_string_to_char_vector(&empty_s))
}

fn apply_operation(opernd1: i64, opernd2: i64, operation: char) -> Option<i64> {
    match operation {
        '+' => return Some(opernd1 + opernd2),
        '-' => return Some(opernd1 - opernd2),
        '*' => return Some(opernd1 * opernd2),
        // *** take care of division by zero!! ****
        '/' => if opernd2 == 0 {println!("{}", "You divided by zero! Exiting program...");
            return None
                }
            else {
                return Some(opernd1 / opernd2)
            },
        _ => return Some(0),
    }
}

#[test]
fn test_apply_operation() {
    assert_eq!(Some(2), apply_operation(1, 1, '+'));
    assert_eq!(Some(80), apply_operation(100, 20, '-'));
    assert_eq!(Some(8), apply_operation(2, 4, '*'));
    assert_eq!(Some(3), apply_operation(12, 4, '/'));
    assert_eq!(None, apply_operation(2342, 0, '/'));
}

fn operator_precedence(operator: char) -> i32 {
    if operator == '+' || operator == '-' {return 1};
    if operator == '*' || operator == '/' {return 2};
    return 0;
}

#[test]
fn test_operator_precedence() {
    assert_eq!(1, operator_precedence('+'));
    assert_eq!(1, operator_precedence('-'));
    assert_eq!(2, operator_precedence('*'));
    assert_eq!(2, operator_precedence('/'));
    assert_eq!(0, operator_precedence('('));
    assert_eq!(0, operator_precedence(')'));
}

fn remove_all_white_space_from_string(incoming_string: &str) -> String {
    // establish vec iterator
    let string_obj: String = String::from(incoming_string);
    let mut out_string: String = String::from("");
    // string vec
    let string_vec = string_obj.chars();
    // loop through vector
    for c in string_vec {
        if c != ' ' { out_string.push(c); }
    }
    out_string
}

#[test]
fn test_remove_all_white_space_from_string() {
    let str0 = "something";
    let str1 = " s o m e t h i n g ";
    let str2 = "som          ething";
    let str3 = " so   m  e   t   hin   g";
    let str4 = "      som eth in   g  ";
    let str5 = "";
    let str6 = "   ";

    assert_eq!("something", remove_all_white_space_from_string(str0));
    assert_eq!("something", remove_all_white_space_from_string(str1));
    assert_eq!("something", remove_all_white_space_from_string(str2));
    assert_eq!("something", remove_all_white_space_from_string(str3));
    assert_eq!("something", remove_all_white_space_from_string(str4));
    assert_eq!("", remove_all_white_space_from_string(str5));
    assert_eq!("", remove_all_white_space_from_string(str6));
}

/// this function splits the incoming comparison string into two expression strings
/// along the appropriate comparison symbol - returns a string vector
fn split_comparison_string(comparison_string: String) -> Vec<String> {
    if comparison_string.contains("==") {
        return comparison_string.split("==").map(|s| s.to_string()).collect()}
    else if comparison_string.contains("<=") {
        return comparison_string.split("<=").map(|s| s.to_string()).collect()}
    else if comparison_string.contains(">=") {
        return comparison_string.split(">=").map(|s| s.to_string()).collect()}
    else if comparison_string.contains("<") {
        return comparison_string.split("<").map(|s| s.to_string()).collect()}
    else if comparison_string.contains(">") {
        return comparison_string.split(">").map(|s| s.to_string()).collect()}
    return vec!["".to_string()]
}

#[test]
fn test_split_comparison_string() {
    let vec0 = vec!["4", "3"];

    assert_eq!(vec0, split_comparison_string("4==3".to_string()));
    assert_eq!(vec0, split_comparison_string("4<=3".to_string()));
    assert_eq!(vec0, split_comparison_string("4>=3".to_string()));
    assert_eq!(vec0, split_comparison_string("4<3".to_string()));
    assert_eq!(vec0, split_comparison_string("4>3".to_string()));

}

fn extract_comparison_notation(comparison_string: String) -> String {
    if comparison_string.contains("==") {return "==".to_string()}
    else if comparison_string.contains("<=") {return "<=".to_string()}
    else if comparison_string.contains(">=") {return ">=".to_string()}
    else if comparison_string.contains("<") {return "<".to_string()}
    else if comparison_string.contains(">") {return ">".to_string()}
    return "".to_string()
}

#[test]
fn test_extract_comparison_notation() {
    assert_eq!("==".to_string(), extract_comparison_notation("23==45".to_string()));
    assert_eq!("<=".to_string(), extract_comparison_notation("23<=45".to_string()));
    assert_eq!(">=".to_string(), extract_comparison_notation("23>=45".to_string()));
    assert_eq!("<".to_string(), extract_comparison_notation("23<5".to_string()));
    assert_eq!(">".to_string(), extract_comparison_notation("23>45".to_string()));
    assert_eq!("".to_string(), extract_comparison_notation("2345".to_string()));
}


///*************
fn handle_comparison_expression(string_expression: String) -> i32 {
    let operation = extract_comparison_notation(string_expression.to_owned());
    let comparison_operands_vec = split_comparison_string(string_expression.to_owned());
    // make vec of &str for string elements
    let str_comparison_operands_vec = vec![&*comparison_operands_vec[0], &*comparison_operands_vec[1]];

    return evaluate_comparison_expression(str_comparison_operands_vec, operation)
}

#[test]
fn test_handle_comparison_expression() {
    assert_eq!(1, handle_comparison_expression("3==3".to_string()));
    assert_eq!(0, handle_comparison_expression("3<3".to_string()));
    assert_eq!(0, handle_comparison_expression("(3+3)/2>3".to_string()));
    assert_eq!(1, handle_comparison_expression("(((32+6)+2*5)-8)/2<=21".to_string()));
    assert_eq!(0, handle_comparison_expression("34==(4+1)*2".to_string()));
    assert_eq!(1, handle_comparison_expression("3>=3".to_string()));
}


// ------------------ Expression Evaluation Algorithm -------------------------

fn evaluate_clean_expression(clean_expression: &str) -> i64 {
    // change input expression string to character vector
    let expression_vector = expression_string_to_char_vector(clean_expression);
    // initialize character vector to hold operation tokens
    let mut operation_vector: Vec<char> = [].to_vec();
    // initialize intermediate answer value vector
    let mut intermediate_value_vector: Vec<i64> = [].to_vec();

    let mut vector_iterator = expression_vector.iter().peekable();

    while let Some(token) = vector_iterator.next() {
        // skips white spaces contained in string
        if token == &' ' {
            continue;
        }
        // collect '('
        else if token == &'(' {
            operation_vector.push(*token);
        }
        // collect numerical value
        else if token.is_numeric() {

            let mut current_val = token.to_string().parse::<i64>().unwrap();

            // collect entire numeric value if next value is a number
            if vector_iterator.peek() != None && vector_iterator.peek().unwrap().is_numeric() {
                while let Some(token) = vector_iterator.next(){
                    current_val = current_val*10 + token.to_string().parse::<i64>().unwrap();
                    if vector_iterator.peek() == None || !vector_iterator.peek().unwrap().is_numeric(){
                        break}
                }
            }
            intermediate_value_vector.push(current_val);
        }
        else if token == &')' {
            // "top" of stack is vector[vector.len() - 1]
            while !operation_vector.is_empty() && operation_vector[operation_vector.len() - 1] != '(' {
                let operand2 = intermediate_value_vector.pop().unwrap();
                let operand1 = intermediate_value_vector.pop().unwrap();
                let operation = operation_vector.pop().unwrap();

                // perform operation
                let operation_result = apply_operation(operand1, operand2, operation);
                match operation_result {
                    Some(vall) => intermediate_value_vector.push(vall),
                    None => process::exit(1)
                };
            }
            // pop '(' from operation vector
            operation_vector.pop();
        }
        else {
            while !operation_vector.is_empty() &&
                operator_precedence(operation_vector[operation_vector.len() - 1]) >=
                    operator_precedence(*token) {
                let operand2 = intermediate_value_vector.pop().unwrap();
                let operand1 = intermediate_value_vector.pop().unwrap();
                let operation = operation_vector.pop().unwrap();

                // perform operation
                let operation_result = apply_operation(operand1, operand2, operation);
                match operation_result {
                    Some(vall) => intermediate_value_vector.push(vall),
                    None => process::exit(1)
                };
            }
            operation_vector.push(*token);
        }
    }
    // entire expression has been parsed, apply remaining operations to remaining values
    while !operation_vector.is_empty() {
        let operand2 = intermediate_value_vector.pop().unwrap();
        let operand1 = intermediate_value_vector.pop().unwrap();
        let operation = operation_vector.pop().unwrap();

        // perform operation
        let operation_result = apply_operation(operand1, operand2, operation);
        // put result in value vector
        match operation_result {
            Some(vall) => intermediate_value_vector.push(vall),
            None => process::exit(1),
        };
    }
    // this will be the final answer..
    return intermediate_value_vector[intermediate_value_vector.len()-1];
}

#[test]
fn test_evaluate_clean_expression() {
    assert_eq!(5, evaluate_clean_expression(&"3+2"));
    assert_eq!(48, evaluate_clean_expression(&"(32+6)+2*5"));
    assert_eq!(20, evaluate_clean_expression(&"(((32+6)+2*5)-8)/2"));
    assert_eq!(20, evaluate_clean_expression(&"  ((( 32+  6)+2 *5) -8  )/   2" ));
    assert_eq!(20, evaluate_clean_expression(&"  ((( 32+  6)    +2 *5) -8  )/   2" ));
    assert_eq!(1, evaluate_clean_expression(&"21/21"));
    assert_eq!(3, evaluate_clean_expression(&"3"));
    assert_eq!(0, evaluate_clean_expression(&"0"))
}

// --------------- END Expression Evaluation Algorithm --------------------

// ------------------ Comparison Evaluation Algorithm ---------------------

fn evaluate_comparison_expression(expressions_vec: Vec<&str>, comparison_notation: String) -> i32 {
    let value_of_exp0 = evaluate_clean_expression(&expressions_vec[0]);
    let value_of_exp1 = evaluate_clean_expression(&expressions_vec[1]);

    let mut comparison_valid = false;

    if comparison_notation == "==".to_string() {comparison_valid = value_of_exp0 == value_of_exp1}
    else if comparison_notation == ">=".to_string() {comparison_valid = value_of_exp0 >= value_of_exp1}
    else if comparison_notation == "<=".to_string() {comparison_valid = value_of_exp0 <= value_of_exp1}
    else if comparison_notation == ">".to_string() {comparison_valid = value_of_exp0 > value_of_exp1}
    else if comparison_notation == "<".to_string() {comparison_valid = value_of_exp0 < value_of_exp1}

    match comparison_valid {
        true => return 1,
        false => return 0,
    }
}

#[test]
fn test_evaluate_comparison_expression() {
    // vec = [20, 20]
    let vec0 = vec!["(((32+6)+2*5)-8)/2", "20"];
    let vec1 = vec!["(((32+6)+2*5)-8)/2", "20"];
    let vec2 = vec!["(((32+6)+2*5)-8)/2", "20"];
    let vec3 = vec!["(((32+6)+2*5)-8)/2", "20"];
    let vec4 = vec!["(((32+6)+2*5)-8)/2", "21"];

    assert_eq!(1, evaluate_comparison_expression(vec0, String::from("==")));
    assert_eq!(0, evaluate_comparison_expression(vec1, String::from("<")));
    assert_eq!(0, evaluate_comparison_expression(vec2, String::from(">")));
    assert_eq!(1, evaluate_comparison_expression(vec3, String::from("<=")));
    assert_eq!(0, evaluate_comparison_expression(vec4, String::from(">=")));
}

// -----------------End Comparison Algorithm-------------------------------

/// function that replaces all variable declarations with appropriate value in hashmap
/// if no such variable exists in map, create one and assign value to zero
fn replace_variable_references_with_value_strings(expression_str: String,
                                                  variable_map: &mut HashMap<String, String>) -> String {
    // put all characters in expr string into vector
    let expression_vec = expression_string_to_char_vector(&expression_str);
    // this will be the new vector built from replaced values for variables
    let mut new_expression_vec: Vec<String> = vec![].to_vec();
    // establish iterator for string
    let mut vector_iterator = expression_vec.iter().peekable();

    let mut temp_str = "".to_string();

    while let Some(token) = vector_iterator.next() {
        // if lowercase letter is detected, start variable name collection
        if token.is_ascii_lowercase() {
            // push letter to temp string (temp string is search key in map)
            temp_str.push(*token);
            // collect the rest of the variable name reference
            // check if the next char is a lower case letter or a number, but not None
            if vector_iterator.peek() != None && (vector_iterator.peek().unwrap().is_ascii_lowercase()
                || vector_iterator.peek().unwrap().is_numeric()) {
                // keep collecting until something other than a lowercase letter or number is detected
                while let Some(token) = vector_iterator.next() {
                    temp_str.push(*token);
                    if vector_iterator.peek() == None || (!vector_iterator.peek().unwrap().is_ascii_lowercase() &&
                        !vector_iterator.peek().unwrap().is_numeric()) {
                        break;
                    }
                }
            }
            // println!("{}", temp_str);
            // now temp string holds variable reference...
            // retrieve value string of variable reference and insert it into new_expression_vec
            let variable_value_string = get_variable_value(temp_str.to_owned(),
                                                           variable_map);
            new_expression_vec.push(variable_value_string);
            // clear temp string
            temp_str = "".to_string();
        }
        else {
            // if variable reference is not detected, simply push char to new vec as normal
            new_expression_vec.push(token.to_string());
        }
    }
    // put vec together and return newly formed string
    new_expression_vec.join("")
}

#[test]
fn test_replace_variable_references_with_value_strings() {
    let mut sample_hash_map: HashMap<String, String> = HashMap::new();
    sample_hash_map.insert("variable0".to_string(), "7".to_string());
    sample_hash_map.insert("variable1".to_string(), "510".to_string());
    let mut sample_hash_map1: HashMap<String, String> = HashMap::new();
    sample_hash_map1.insert("variable0".to_string(), "7".to_string());
    sample_hash_map1.insert("variable1".to_string(), "1".to_string());

    assert_eq!("3+7".to_string(), replace_variable_references_with_value_strings("3+variable0".to_string(),
                                                                                 &mut sample_hash_map));
    assert_eq!("3+0".to_string(), replace_variable_references_with_value_strings("3+variable5fr6".to_string(),
                                                                                 &mut sample_hash_map));
    assert_eq!("(3+7)*510-0+32".to_string(), replace_variable_references_with_value_strings("(3+variable0)*variable1-a+32".to_string(),
                                                                                 &mut sample_hash_map))
}


/// this function retrieves the value string of the variable reference
/// if the reference is not found in the hashmap, insert it and assign a value of "0"
/// takes a string
fn get_variable_value(variable_reference: String, variable_hash_map: &mut HashMap<String, String>) -> String {
    let mut variable_value_str = String::new();
    if variable_hash_map.contains_key(&variable_reference) {
        variable_value_str = variable_hash_map.get(&variable_reference).unwrap().to_owned();
    }
    else {
        variable_hash_map.insert(variable_reference.to_string(), String::from("0"));
        variable_value_str = variable_hash_map.get(&variable_reference).unwrap().to_owned();
    }
    return variable_value_str

}

#[test]
fn test_get_variable_value() {
    let mut sample_hash_map: HashMap<String, String> = HashMap::new();
    sample_hash_map.insert("variable0".to_string(), "7".to_string());
    sample_hash_map.insert("variable1".to_string(), "10".to_string());
    let mut sample_hash_map1: HashMap<String, String> = HashMap::new();
    sample_hash_map1.insert("variable0".to_string(), "7".to_string());
    sample_hash_map1.insert("variable1".to_string(), "1".to_string());

    // println!("{:?}", sample_hash_map1.to_owned());
    assert_eq!("0".to_string(), get_variable_value("variable2".to_string(),
                                                 &mut sample_hash_map1));
    // println!("{:?}", sample_hash_map1.to_owned());

    assert_eq!("10".to_string(), get_variable_value("variable1".to_string(),
                                                    &mut sample_hash_map));
}