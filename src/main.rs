use std::io;
use std::iter::Peekable;
use std::str::Chars;

fn main() {
    println!("Rcalc. Enter an expression:\n");
    repl();
}

fn repl() {
    let scanner = io::stdin();
    let mut input = String::new();

    loop {
        match scanner.read_line(&mut input) {
            Ok(_) => {
                if is_invalid(&input) {
                    input = "".to_string(); // Clear the buffer.
                    continue;
                }

                print_result(calculate(&input));
            },
            Err(error) => println!("Error scanning input: {}", error)
        }

        input = "".to_string(); // Clear the buffer.
    }
}

fn is_invalid(input: &String) -> bool {
    input.chars().all(|c| c.is_whitespace())
}

fn calculate(expression: &String) -> Result<u32, String> {
    let mut terms = expression.chars().peekable();

    let first_operand = number_term(&mut terms).unwrap();
    let operator = terms.next().unwrap();
    let second_operand = number_term(&mut terms).unwrap();

    match operator {
        '+' => Ok(add(first_operand, second_operand)),
        _ => Err(format!("Unsupported operation: '{}'", operator))
    }
}

fn number_term(terms: &mut Peekable<Chars>) -> Option<u32> {
    let mut digits = String::new();

    while let Some('0'...'9') = terms.peek() {
        digits.push(terms.next().unwrap());
    }

    if digits.is_empty() {
        None
    } else {
        Some(digits.parse().unwrap())
    }
}

fn add(first_operand:u32, second_operand: u32) -> u32 {
    first_operand + second_operand
}

fn print_result(result: Result<u32, String>) {
    match result {
        Ok(value) => println!("{}", value),
        Err(err) => println!("{}", err)
    }
}

#[test]
fn test_is_invalid() {
    let empty_line = "\n".to_string();
    let empty_string = "".to_string();
    let whitespace = "\n     \t\t  \n".to_string();

    assert!(is_invalid(&empty_line));
    assert!(is_invalid(&empty_string));
    assert!(is_invalid(&whitespace));
}

#[test] 
fn test_calculate_addition() {
    let addition = "5+3".to_string();

    assert!(calculate(&addition).is_ok());
    assert_eq!(calculate(&addition).unwrap(), 8, "Addition of 5 + 3");
}

#[test]
fn test_calculate_unsupported_operation() {
    let subtraction = "4-3".to_string();

    assert!(calculate(&subtraction).is_err());
}

#[test] fn test_calculate_multiple_digits() {
    let multi_digits = "123+456".to_string();

    assert!(calculate(&multi_digits).is_ok());
    assert_eq!(calculate(&multi_digits).unwrap(), 579, "Addition of 123 + 456");
}


