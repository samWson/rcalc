fn main() {
    println!("Hello, world!");
}

fn calculate(expression: String) -> u32 {
    let mut terms = expression.chars();
    let first_operand = terms.next().unwrap().to_digit(10).unwrap();
    let operator = terms.next().unwrap();
    let second_operand = terms.next().unwrap().to_digit(10).unwrap();

    match operator {
        '+' => add(first_operand, second_operand),
        _ => panic!("Unsupported operation.")
    }
}

fn add(first_operand:u32, second_operand: u32) -> u32 {
    first_operand + second_operand
}

#[test]
fn test_calculate() {
    let addition1 = "5+3".to_string();
    let addition2 = "1+2".to_string();

    assert_eq!(calculate(addition1), 8, "Addition of 5 + 3");
    assert_eq!(calculate(addition2), 3, "Addition of 1 + 2");
}
