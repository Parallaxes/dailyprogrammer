/*
[2/12/2012] Challenge #4 [intermediate]
create a calculator program that will take an input, following normal calculator input (5*5+4) and give an answer (29). This calculator should use all four operators.

For extra credit, add other operators (6(4+3), 3 ** 3, etc.)
*/

use std::io::{self, Write};

fn main() {
    loop {
        io::stdout().write(b"Enter an expression: ").unwrap();
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input == "exit" {
            break;
        }

        let result = evaluate(input);
        println!("{}", result);
    }
}

fn evaluate(input: &str) -> f64 {
    let mut tokens = input.chars().filter(|c| !c.is_whitespace()).peekable();
    let result = parse_expression(&mut tokens);
    if tokens.peek().is_some() {
        panic!("Unexpected token: {}", tokens.collect::<String>());
    }
    result
}

fn parse_expression<I>(tokens: &mut std::iter::Peekable<I>) -> f64
where
    I: Iterator<Item = char>,
{
    let mut result = parse_term(tokens);
    loop {
        match tokens.peek() {
            Some(&'+') => {
                tokens.next();
                result += parse_term(tokens);
            }
            Some(&'-') => {
                tokens.next();
                result -= parse_term(tokens);
            }
            _ => break,
        }
    }
    result
}

fn parse_term<I>(tokens: &mut std::iter::Peekable<I>) -> f64
where
    I: Iterator<Item = char>,
{
    let mut result = parse_factor(tokens);
    loop {
        match tokens.peek() {
            Some(&'*') => {
                tokens.next();
                result *= parse_factor(tokens);
            }
            Some(&'/') => {
                tokens.next();
                result /= parse_factor(tokens);
            }
            _ => break,
        }
    }
    result
}

fn parse_factor<I>(tokens: &mut std::iter::Peekable<I>) -> f64
where
    I: Iterator<Item = char>,
{
    match tokens.next() {
        Some('(') => {
            let result = parse_expression(tokens);
            if tokens.next() != Some(')') {
                panic!("Expected closing parenthesis");
            }
            result
        }
        Some(c @ '0'..='9') => c.to_digit(10).unwrap() as f64,
        Some(c) => panic!("Unexpected token: {}", c),
        None => panic!("Unexpected end of input"),
    }
}