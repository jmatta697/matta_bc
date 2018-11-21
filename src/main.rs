extern crate regex;

use std::env;
use std::io;
use std::process;
use std::collections::HashMap;
use regex::Regex;


fn main() {
    // hash map will hold all variables and their values
    let variable_map: &mut HashMap<String, String> = &mut HashMap::new();
    // program control - also gets command line arg
    let command_line_arg = get_command_line_args();

    if command_line_arg.trim() == "-1" {
        println!("\nToo many command line arguments! Enter one argument or none. Exiting program...\n");
    }
    else if command_line_arg.trim() == "" {
        println!("{}", "Copyright 2018 - Joe Matta. All rights reserved.");
        main_user_input_loop(variable_map);
    }
    else if command_line_arg.trim() == "--help" {
        print_help_text();
    }
    else {
        main_one_time_eval(command_line_arg, variable_map);
    }
}

// -----------------------------------------------------------------------------------------------

/// this function processes the one-time evaluation when program is called with a command line arg
fn main_one_time_eval(command_line_arg: String, varib_map: &mut HashMap<String, String>) {

    let regex_pat = Regex::new(r"^[a-z 0-9+\-*/()=<>]*$").unwrap();

    // clean all white space from string and any trailing or heading escape chars
    let clean_white_space_str = remove_all_white_space_from_string(command_line_arg.trim());

    if !valid_expression_str(&command_line_arg, &regex_pat) {
        println!("{}", "Invalid Input!");
        process::exit(0);
    }

    // check explicit division by 0
    if clean_white_space_str.contains("/0") {
        println!("{}", "You divided by 0! Enter a valid expression..");
    }
    // replace variables with values - (will always be zero in the case of passing command line arg)
    // replace all variable declarations with appropriate values ** this is the clean string
    let variables_replaced_str = replace_variable_references_with_value_strings(
        clean_white_space_str.to_owned(), varib_map);
    // check explicit division by 0 after replacing variable values...
    if variables_replaced_str.contains("/0") {
        println!("{}", "You divided by 0! Enter a valid expression..");
    }
    // check if expression has comparison
    let is_comparison = string_has_comparison(clean_white_space_str.to_owned());
    // if input string is a comparison, evaluate comparison..
    if clean_white_space_str.chars().nth(0).unwrap().is_ascii_lowercase() &&
        is_a_variable_assignment(&clean_white_space_str) && !is_comparison {
        println!("{}", "\nVariable not stored! Variables cannot be assigned as a command line argument.\n");
    }
    else if is_comparison {
        println!("{}", handle_comparison_expression(variables_replaced_str.to_owned()));
    }
    else if clean_white_space_str.chars().nth(0).unwrap().is_ascii_lowercase() &&
        clean_white_space_str.chars().all(|x| x.is_alphanumeric()) {
        println!("{}", '0');
    }
    else {
        println!("{}", evaluate_clean_expression(&variables_replaced_str));
    }
}

/// main user input while loop
fn main_user_input_loop(varib_map: &mut HashMap<String, String>) {

    let regex_pat = Regex::new(r"^[a-z 0-9+\-*/()=<>]*$").unwrap();

    let mut user_input = "".to_string();

    while user_input.trim() != "quit" {
        user_input = "".to_string();
        // wait for user input
        io::stdin().read_line(&mut user_input)
            .expect("failed to read input");

        // clean all white space from string and any trailing or heading escape chars
        let clean_white_space_str = remove_all_white_space_from_string(user_input.trim());

        // check for valid input - if not go to next iteration and wait for user input
        if !valid_expression_str(&clean_white_space_str, &regex_pat) {
            println!("{}", "Invalid Input!");
            continue;
        }

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

/// determines amount of command line arguments passed to program call
fn get_command_line_args() -> String {
    let command_line_args: Vec<_> = env::args().collect();

    if command_line_args.len() == 1 {
        return "".to_string()
    }
    else if command_line_args.len() == 2 {

        // if argument == '--help' ---> print contents of help file

        // else...
        return command_line_args[1].to_owned();
    }
    // simply exit program after giving error message
    return "-1".to_string()
}

fn string_has_comparison(in_string: String) -> bool {
    if in_string.contains("==") || in_string.contains("<=") || in_string.contains(">=")
        || in_string.contains("<") || in_string.contains(">") {
        return true
    }
    false
}

/// changes incoming expression string into a char vector
fn expression_string_to_char_vector(expr_string: &str) -> Vec<char> {
    let char_vec: Vec<_> = expr_string.chars().collect();
    return char_vec
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

fn operator_precedence(operator: char) -> i32 {
    if operator == '+' || operator == '-' {return 1};
    if operator == '*' || operator == '/' {return 2};
    return 0;
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

fn extract_comparison_notation(comparison_string: String) -> String {
    if comparison_string.contains("==") {return "==".to_string()}
    else if comparison_string.contains("<=") {return "<=".to_string()}
    else if comparison_string.contains(">=") {return ">=".to_string()}
    else if comparison_string.contains("<") {return "<".to_string()}
    else if comparison_string.contains(">") {return ">".to_string()}
    return "".to_string()
}

fn handle_comparison_expression(string_expression: String) -> i32 {
    let operation = extract_comparison_notation(string_expression.to_owned());
    let comparison_operands_vec = split_comparison_string(string_expression.to_owned());
    // make vec of &str for string elements
    let str_comparison_operands_vec = vec![&*comparison_operands_vec[0], &*comparison_operands_vec[1]];

    return evaluate_comparison_expression(str_comparison_operands_vec, operation)
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
// --------------- END Expression Evaluation Algorithm --------------------

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

// ---------------- Valid Expression Check Functions ------------------------------------

/// makes sure there are only allowed characters in string
fn check_only_valid_chars(expr_str: &str, reg: &Regex) -> bool {
    // check for empty string
    if expr_str == "" || expr_str == "(" {
        return false
    }
    // return T/F based on matching regex
    // ^[a-z 0-9\+\-\*\/\(\)\=\<\>]*$
    return reg.is_match(expr_str);
}

/// assumes check valid char has already accounted for empty string
fn check_first_char_in_exp(expr_str: &str) -> bool {
    let first_ch = expr_str.chars().nth(0).unwrap();
    if (first_ch.is_ascii_lowercase() || first_ch.is_ascii_digit() || first_ch == '(') &&
        first_ch != '0' {
        return true
    }
    false
}

fn check_last_char(expr_str: &str) -> bool {
    let last_char = expr_str.chars().last().unwrap();
    if expr_str.len() == 1 || last_char.is_ascii_alphanumeric() || last_char == ')' {
        return true
    }
    return false
}

/// checks if there are matching pairs of parentheses
fn count_valid_parentheses(expr_str: &str) -> bool {
    let mut num_open_parentheses = 0;
    let mut num_close_parentheses = 0;
    for ch in expr_str.chars() {
        if ch == '(' {num_open_parentheses = num_open_parentheses + 1;}
        else if ch == ')' {num_close_parentheses = num_close_parentheses + 1;}
    }
    return num_open_parentheses == num_close_parentheses
}

/// assumes valid first character and valid last char - checks for any syntax errors in expression string
fn check_expr_syntax(expr_str: &str) -> bool {
    let char_vec = expression_string_to_char_vector(expr_str);
    let mut char_vec_iter = char_vec.iter().peekable();
    let mut next_char: char;
    while let Some(current_char) = char_vec_iter.next() {
        if char_vec_iter.peek() != None {
            next_char = **char_vec_iter.peek().unwrap();
        }
        else {
            break;
        }

        if current_char.is_ascii_digit() {
            match next_char.is_ascii_alphabetic() {
                true => return false,
                _ => (),
            }
            if next_char == '(' {return false};
        }
        else if current_char.is_ascii_alphabetic() {
            match next_char {
                '(' => return false,
                _ => (),
            }
        }
        else if *current_char == ')' {
            if next_char.is_ascii_alphanumeric() || next_char == '(' {
                return false
            }
        }

        else if *current_char == '+' || *current_char == '-' || *current_char == '*' ||
            *current_char == '/' || *current_char == '(' {
            match next_char {
                ')'|'+'|'-'|'*'|'/'|'<'|'>'|'=' => return false,
                _ => (),
            }
        }
        else {
            match next_char {
                ')'|'+'|'-'|'*'|'/'|'<'|'>' => return false,
                _ => (),
            }
        }
    }
    return true;
}

/// test ALL string coming in
fn valid_expression_str(expr_str: &str, regx: &Regex) -> bool {
    if !check_only_valid_chars(expr_str, regx) {return false};
    if !check_first_char_in_exp(expr_str) {return false};
    if !check_last_char(expr_str) {return false};
    if !count_valid_parentheses(expr_str) {return false};
    if !check_expr_syntax(expr_str) {return false};
    return true
}

// -----------------------------

fn print_help_text() {
    let help_text = "\nAuthor: Joe Matta
Copyright 2018 - Joe Matta. All rights reserved.

This program aims to imitate bc which is a command line calculator. Typing bc at the command line of most linux systems
will bring up a copyright notice then the program waits for you to type in a bit of math, it will then evaluate that math
and print an answer, then wait for the user to type more.

This implementation of bc does the following:

supports INTEGER arithmetic (+ - * /)
supports parenthesis including nested parentheses
supports variables -
    variables that are referenced but not yet assigned automatically get a value of zero.
    allows assignment to variables using =
supports comparisons < =< etc. (return 1 for true and 0 for false)
supports passing arithmetic expressions on the command line as command line parameters when starting matta_bc
eg:
entering the following in the command line: matta_bc 5+3
    will invoke matta_bc for the single computation, print the answer of 8 and then exit the program.
help (this document) is available through the help flag: matta_bc --help
    - this will bring up this document explaining how to use matta_bc

To run the program from the command line:
Navigate to the folder that holds the executable matta_bc (file with no extension) and type the following:
$ ./matta_bc
Calling the program with no command line arguments will allow the user to enter an expression and obtain a result as
many times as they would like until 'quit' is entered. Entering 'quit' ends the program.
The user may pass a command line argument:
$ ./matta_bc 5+3
If more that one argument is passed to the program call, an error will occur.
$ ./matta_bc 5+3 7-2  ---> error
If the user includes spaces, parentheses, '<', or '>', single quotes are required around the argument. Some other cases
may exist that necessitate the use of quotes. Thus, the user is advised to always use quotes when passing a command line
argument.
$ ./matta_bc '(3+3)/2'

notes:
- Multiple digit numbers cannot have spaces between digits (ex. 32 cannot be expressed as 3 2) when passing an expression
    as a command line argument without quotes.
- Two comparisons cannot be combined in one expression such as 234<=23==1; this will cause a panic. Some other double
    comparisons are allowed such as 344<567==1.
- Multiplication cannot be done by (2)(3) or 2(3), must be (2)*(3) or 2*(3).
- Variable names must start with a lowercase letter. They may only contain lowercase letters and numbers.
- Variable names must end in numbers, if they include numbers. Varibale names cannot have numbers inside name
    (ie. 'var3iable' is not a valid variable name, but 'variable33' is)
- When passing a command line argument without quotes around it, argument cannot contain spaces or parentheses
- Variables can NOT be assigned to other variables (ie. 'variable0 = variable1' is not valid)
- If assigning a value to a variable, user input must start with a lowercase letter followed by any number of any
    combination of lowercase letters and then followed by numbers and then '='. The assigned value may be a
    mathematical expression or a comparison of expressions.
- Dividing by 0 explicitly (ie. entering '3/0') will cause the an error and the user will be prompted to enter a valid
    expression. Dividing by 0 implicitly (ie. entering '3/(2-2)' will cause a fatal error; a message will be printed
    and the program will exit.
- When entering an expression as a command line argument, the user is required to surround the expression with
    single quotes in some cases, specifically in the case of comparisons using '<' or '>'. Single quotes must also be
    used when the expression contains parentheses. For most other cases, the expression can be entered without being
    surrounded by quotes. This is the result of this notation coinciding with bash commands.
    Example: 34<=100 must be entered as '34<=100' as a command line argument
    Example: (3+3)/2 must be entered as '(3+3)/2' as a command line argument

source for expression evaluation algorithm:
https://www.geeksforgeeks.org/expression-evaluation/\n";
    println!("{}", help_text)
}


// -------------------------- TESTS ----------------------------------------------------------------

#[test]
fn test_is_a_variable_assignment() {
    assert_eq!(false, is_a_variable_assignment("sa4rw45fw4c5e"));
    assert_eq!(false, is_a_variable_assignment("123rq3d==aefe4c"));
    assert_eq!(true, is_a_variable_assignment("a=23"));
    assert_eq!(true, is_a_variable_assignment("var=(2345+45)*2<=2"));
    // assert_eq!(true, is_a_variable_assignment(sda<=))
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

#[test]
fn test_expression_string_to_vector() {
    let s = String::from("(3+4)*2/23");
    let empty_s = String::from("");
    let v = vec!['(', '3', '+', '4', ')', '*', '2', '/', '2', '3'];
    let empty_v: Vec<char> = vec![];

    assert_eq!(v, expression_string_to_char_vector(&s));
    assert_eq!(empty_v, expression_string_to_char_vector(&empty_s))
}

#[test]
fn test_apply_operation() {
    assert_eq!(Some(2), apply_operation(1, 1, '+'));
    assert_eq!(Some(80), apply_operation(100, 20, '-'));
    assert_eq!(Some(8), apply_operation(2, 4, '*'));
    assert_eq!(Some(3), apply_operation(12, 4, '/'));
    assert_eq!(None, apply_operation(2342, 0, '/'));
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

#[test]
fn test_split_comparison_string() {
    let vec0 = vec!["4", "3"];

    assert_eq!(vec0, split_comparison_string("4==3".to_string()));
    assert_eq!(vec0, split_comparison_string("4<=3".to_string()));
    assert_eq!(vec0, split_comparison_string("4>=3".to_string()));
    assert_eq!(vec0, split_comparison_string("4<3".to_string()));
    assert_eq!(vec0, split_comparison_string("4>3".to_string()));

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

#[test]
fn test_handle_comparison_expression() {
    assert_eq!(1, handle_comparison_expression("3==3".to_string()));
    assert_eq!(0, handle_comparison_expression("3<3".to_string()));
    assert_eq!(0, handle_comparison_expression("(3+3)/2>3".to_string()));
    assert_eq!(1, handle_comparison_expression("(((32+6)+2*5)-8)/2<=21".to_string()));
    assert_eq!(0, handle_comparison_expression("34==(4+1)*2".to_string()));
    assert_eq!(1, handle_comparison_expression("3>=3".to_string()));
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

#[test]
fn test_check_only_valid_chars() {
    let regex_pat: &Regex = &Regex::new(r"^[a-z 0-9+\-*/()=<>]*$").unwrap();
    assert_eq!(true, check_only_valid_chars("1234567890-+><=()qwertyuioplkjhgfdsazxcvbnm",
                                            regex_pat));
    assert_eq!(false, check_only_valid_chars("1234%567890-+><=()qwer@tyuioplkjhgfdsazxcvbnm",
                                             regex_pat));
    assert_eq!(false, check_only_valid_chars("1234567890-+><=()qwerMtyuioplkjhgfdsazxcvbnm",
                                             regex_pat));
    assert_eq!(false, check_only_valid_chars("1234567890-+><=()qw&ertyuioplkjhgfdsazxcvbnm",
                                             regex_pat));
    assert_eq!(false, check_only_valid_chars("1234567890-+><=()QWERTYUIOPLKJHGFDSAZXCVBNM",
                                             regex_pat));
    assert_eq!(false, check_only_valid_chars("", regex_pat))
}

#[test]
fn test_check_first_char_in_exp() {
    assert_eq!(false, check_first_char_in_exp("098usidk+o12"));
    assert_eq!(false, check_first_char_in_exp("+xf34trwcf4"));
    assert_eq!(false, check_first_char_in_exp("-sdfse"));
    assert_eq!(true, check_first_char_in_exp("asads"));
    assert_eq!(false, check_first_char_in_exp("<sdwe4"));
    assert_eq!(true, check_first_char_in_exp("423rw3rf"));
    assert_eq!(true, check_first_char_in_exp("(dw34rw"));
    assert_eq!(false, check_first_char_in_exp(")cwe4rcw"));
    assert_eq!(false, check_first_char_in_exp("=dqwrexw"));
    assert_eq!(false, check_first_char_in_exp("0"));
    assert_eq!(true, check_last_char(")"));
}

#[test]
fn test_check_last_char() {
    assert_eq!(true, check_last_char("34rdqq34"));
    assert_eq!(true, check_last_char("34rdqq3s"));
    assert_eq!(true, check_last_char("(34rdqq3)"));
    assert_eq!(false, check_last_char("34rdqq34("));
    assert_eq!(false, check_last_char("34rdqq34-"));
    assert_eq!(false, check_last_char("34rdqq34*"));
    assert_eq!(false, check_last_char("34rdqq34>"));
    assert_eq!(false, check_last_char("34rdqq34="));
    assert_eq!(false, check_last_char("34rdqq34/"));
    assert_eq!(true, check_last_char("3"));
}

#[test]
fn test_count_valid_parentheses() {
    assert_eq!(true, count_valid_parentheses("((3+3)/2*3)/3"));
    assert_eq!(false, count_valid_parentheses("((3+3)/2*3/3"));
    assert_eq!(false, count_valid_parentheses("(3+3/2*3/3"));
    assert_eq!(false, count_valid_parentheses("3+3/2*3)/3"));
    assert_eq!(false, count_valid_parentheses("3+3)/2*3)/3"));
    assert_eq!(true, count_valid_parentheses("3+3/2*3/3"));
    assert_eq!(true, count_valid_parentheses("(((er43r((fs((3+3)/2*3)/3)))sfe))66yr"));
}

#[test]
fn test_check_expr_syntax() {
    assert_eq!(true, check_expr_syntax("(((32+6)+2*5)-8)/2"));
    assert_eq!(false, check_expr_syntax("(((32+6)+2*5)-8)/2(3+2)"));
    assert_eq!(false, check_expr_syntax("(((32+6)+2*5)-8)/joe(3+2)"));
    assert_eq!(false, check_expr_syntax("(((+32+6)+2*5)-8)/2"));
    assert_eq!(false, check_expr_syntax("(((32+6)>+2*5)-8)/2"));
    assert_eq!(false, check_expr_syntax("(((32+6)++2*5)-8)/2"));
    assert_eq!(false, check_expr_syntax("(((32+6)+2*5)-8)/2<<8"));
    assert_eq!(false, check_expr_syntax("(((32+6)+2*5)-8)/2=>9"));
    assert_eq!(false, check_expr_syntax("(((32+6)+2*5)-8)/2=+"));
    assert_eq!(true, check_expr_syntax("(((32+6)+2*5)-8)/2=="));
    assert_eq!(true, check_expr_syntax("9"));
    assert_eq!(true, check_expr_syntax("a"))
}

#[test]
fn test_valid_expression_str() {
    let regex_pat: &Regex = &Regex::new(r"^[a-z 0-9+\-*/()=<>]*$").unwrap();
    assert_eq!(true, valid_expression_str("(((32+6)+2*5)-8)/2", regex_pat));
    assert_eq!(false, valid_expression_str("adrw34w4", regex_pat));
    assert_eq!(false, valid_expression_str("(", regex_pat));
    assert_eq!(false, valid_expression_str("((+2*5)-8)/2", regex_pat));
    assert_eq!(false, valid_expression_str("((()+2*5)-8)/2", regex_pat));
    assert_eq!(false, valid_expression_str("", regex_pat));
    assert_eq!(false, valid_expression_str("+(((32+6)+2*5)-8)/2", regex_pat));
    assert_eq!(false, valid_expression_str("(((32+6)+2*5)-8)/2l", regex_pat));
    assert_eq!(true, valid_expression_str("kkkkkkkkkkkk", regex_pat));
    assert_eq!(true, valid_expression_str("joe+3", regex_pat));
    assert_eq!(true, valid_expression_str("(joe+3)", regex_pat));
    assert_eq!(true, valid_expression_str("3*joe+90+e4", regex_pat));
    assert_eq!(true, valid_expression_str("3", regex_pat));
}
