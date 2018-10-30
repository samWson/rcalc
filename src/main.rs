use std::io;

fn main() {
    println!("Rcalc. Enter an expression:\n");
    repl();
}

fn calculate(expression: &String) -> Result<u32, String> {
    let mut terms = expression.chars();
    let first_operand = terms.next().unwrap().to_digit(10).unwrap();
    let operator = terms.next().unwrap();
    let second_operand = terms.next().unwrap().to_digit(10).unwrap();

    match operator {
        '+' => Ok(add(first_operand, second_operand)),
        _ => Err(format!("Unsupported operation: '{}'", operator))
    }
}

fn add(first_operand:u32, second_operand: u32) -> u32 {
    first_operand + second_operand
}

fn repl() {
    let scanner = io::stdin();
    let mut input = String::new();

    loop {
        match scanner.read_line(&mut input) {
            Ok(_) => {
                println!("You entered: {:?}", &input);
                print_result(calculate(&input));
            },
            Err(error) => println!("Error scanning input: {}", error)
        }
        input = "".to_string(); // Clear the buffer.
    }
}

fn print_result(result: Result<u32, String>) {
    match result {
        Ok(value) => println!("{}", value),
        Err(err) => println!("{}", err)
    }
}

#[test]
fn test_calculate() {
    let addition1 = "5+3".to_string();
    let addition2 = "1+2".to_string();
    let unsupported = "4&3".to_string();

    assert!(calculate(&addition1).is_ok());
    assert!(calculate(&unsupported).is_err());
    assert_eq!(calculate(&addition1).unwrap(), 8, "Addition of 5 + 3");
    assert_eq!(calculate(&addition2).unwrap(), 3, "Addition of 1 + 2");
}

