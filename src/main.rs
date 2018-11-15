use std::env;



fn main() {
    check_for_command_line_args()
}

// ------------------------------------------------------------

/// determines amount of command line arguments passed to program call
fn check_for_command_line_args() {
    let command_line_args: Vec<_> = env::args().collect();

    if command_line_args.len() == 1 {
        println!("no command line args where passed, run regular program")
        // run regular program here
    }
    else if command_line_args.len() == 2 {
        println!("a command line argument was passed, evaluate, print answer, and exit program");
        let incoming_arg: &str = &command_line_args[1];
        let incoming_arg_string: String = String::from(incoming_arg);

        match string_has_comparison(incoming_arg_string) {
            true => return,
            false => println!("{}", evaluate_clean_expression(incoming_arg)),
        };

        // run alternate program here
        // if "--help" is passed:
        // display help doc and exit()ll
        // else:
        // check expression and evaluate, then exit()
    }
    else {
        println!("{}{}", "Number of arguments passed: ",  command_line_args.len()-1);
        println!("Too many command line arguments! Enter one argument or none. Exiting program...");
        // simply exit program after giving error message
    }
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

fn apply_operation(opernd1: i64, opernd2: i64, operation: char) -> i64 {
    match operation {
        '+' => return opernd1 + opernd2,
        '-' => return opernd1 - opernd2,
        '*' => return opernd1 * opernd2,
        // *** take care of division by zero!! ****
        '/' => return opernd1 / opernd2,
        _ => return 0,
    }
}

#[test]
fn test_apply_operation() {
    assert_eq!(2, apply_operation(1, 1, '+'));
    assert_eq!(80, apply_operation(100, 20, '-'));
    assert_eq!(8, apply_operation(2, 4, '*'));
    assert_eq!(3, apply_operation(12, 4, '/'));
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

// ------------------ Comparison Evaluation Algorithm -------------------------
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
    let vec3 = vec!["(((32+6)+2*5)-8)/2", "0"];
    let vec4 = vec!["(((32+6)+2*5)-8)/2", "21"];

    assert_eq!(1, evaluate_comparison_expression(vec0, String::from("==")));
    assert_eq!(0, evaluate_comparison_expression(vec1, String::from("<")));
    assert_eq!(0, evaluate_comparison_expression(vec2, String::from(">")));
    assert_eq!(0, evaluate_comparison_expression(vec3, String::from("<=")));
    assert_eq!(0, evaluate_comparison_expression(vec4, String::from(">=")));
}

///*************
fn handle_comparison_expression()


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
                // put result in value vector
                intermediate_value_vector.push(operation_result);
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
                // put result in value vector
                intermediate_value_vector.push(operation_result);
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
        intermediate_value_vector.push(operation_result);
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