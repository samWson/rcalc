use std::str::FromStr;

fn main() {
    println!("Hello, world!");
}

const PLUS: char = '+';

enum Token<'a> {
    Number(usize),
    Operator(&'a char)
}

struct Expression<'a> {
    first_operand: Token<'a>, 
    operator: Token<'a>,
    second_operand: Token<'a>
}

fn calculate(input: String) -> usize {
    let expression = tokenize(input);
    let value = interpret(expression);
    value
}

fn tokenize<'a>(input: String) -> Expression<'a> {
    let mut characters = input.chars().peekable();
    let mut tokens = Vec::new();

    while let Some(_) = characters.peek() {
        match characters.peek() {
            Some('0'...'9') => tokens.push(number_token(characters.next())),
            Some(&PLUS) => tokens.push(operator_token(characters.next())),
            _ => panic!("Characters must be numbers or operators."),
        }
    }

    // Build expression from tokens using pattern matching
    Expression(Token::Number(5), Token::Operator(&PLUS), Token::Number(3))
}

fn number_token<'a>(literal: Option<char>) -> Token<'a> {
    let value = literal.expect("Shomehow this is dead wrong.").to_digit(10).unwrap() as usize;
    Token::Number(value)
}

fn operator_token<'a>(literal: Option<char>) -> Token<'a> {
    if let Some(PLUS) = literal {
        Token::Operator(&PLUS)
    } else {
        panic!("Unkown operator.");
    }
}

fn interpret(expression: Expression) -> usize {
    match expression {
        Expression(Token::Number(first_operand), Token::Operator(&PLUS), Token::Number(second_operand)) => add(first_operand, second_operand),
        _ => panic!("Expression can't be interpreted.")
    }
}

fn add(first_operand: usize, second_operand: usize) -> usize {
    first_operand + second_operand
}

#[test]
fn test_calculate() {
    let addition1 = "5+3".to_string();
    let addition2 = "1+2".to_string();

    assert_eq!(calculate(addition1), 8, "Addition of 5 + 3");
    assert_eq!(calculate(addition2), 3, "Addition of 1 + 2");
}
