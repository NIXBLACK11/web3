use std::env::{args, Args};

fn main() {
    let mut args: Args = args();
    // println!("{:?}", args);

    let first = args.nth(1).unwrap();
    let operator = args.nth(0).unwrap();
    let second = args.nth(0).unwrap();

    let first_number: f32 = first.parse().unwrap();
    let second_number: f32 = second.parse().unwrap();
    print!("{} {} {} = ", first, operator, second);

    println!("{}", operate(operator.chars().next().unwrap(), first_number, second_number));
}

fn operate(operator: char, first_number: f32, second_number: f32) -> f32 {
    if operator == '+' {
        first_number + second_number
    } else if operator == '-' {
        first_number - second_number
    } else if operator == '*' {
        first_number * second_number
    } else if operator == '/' {
        first_number / second_number
    } else {
        panic!("Unknown operator: {}", operator);
    }
}
