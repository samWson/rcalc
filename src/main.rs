fn main() {
    println!("Hello, world!");
}

const PLUS: char = '+';

enum Token<'a> {
    Number(usize),
    Operator(&'a char)
}

fn calculate(expression: String) -> usize {
    let tokens = tokenize(expression);
    let value = interpret(tokens);
    value
}

// Maybe make tokenize return an expression struct which will have named fields, making it easier
// to return something that can be interpreted.
fn tokenize<'a>(expression: String) -> (Token<'a>, Token<'a>, Token<'a>) {
    let mut characters = expression.chars().peekable();
    let tokens: Vec<Token> = vec![];

    while let Some(_) = characters.peek() {
        match characters.peek() {
            Some('0'...'9') => tokens.push(number_token(characters.next())),
            Some(&PLUS) => tokens.push(operator_token(characters.next())),
            _ => panic!("Characters must be numbers or operators."),
        }
    }

    (Token::Number(5), Token::Operator(&PLUS), Token::Number(3))
}

fn interpret(tokens: (Token, Token, Token)) -> usize {
    match tokens {
        (Token::Number(first_operand), Token::Operator(&PLUS), Token::Number(second_operand)) => add(first_operand, second_operand),
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
