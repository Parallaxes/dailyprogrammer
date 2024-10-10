/*
 * [easy] challenge #1
 * create a program that will ask the users name, age, and reddit username. have it tell them the information back, in the format:
 *
 * your name is (blank), you are (blank) years old, and your username is (blank)
 * 
 * for extra credit, have the program log this information in a file to be accessed later. 
*/

use std::io::{self, Write};

fn main() {
    let mut input = String::new();

    let name = prompt("What is your name?\n", &mut input);
    let age = prompt("What is your age?\n", &mut input);
    let username = prompt("What is your username?\n", &mut input);

    println!("your name is {}, you are {} years old, and your username is {}", name, age, username);
}

fn prompt(message: &str, input: &mut String) -> String {
    print!("{}", message);
    io::stdout().flush().unwrap();
    input.clear();
    io::stdin().read_line(input).unwrap();
    input.trim().to_string()
}
