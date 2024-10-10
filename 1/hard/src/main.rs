/*
 * [difficult] challenge #1 
 * we all know the classic "guessing game" with higher or lower prompts. lets do a role reversal; you create a program that will guess numbers between 1-100,  
 * and respond appropriately based on whether users say that the number is too high or too low.  
 * Try to make a program that can guess your number based on user input and great code!
*/

use std::io::{self, Write};

fn main() {
    let mut min = 1;
    let mut max = 100;

    let mut cnt = 0;

    loop {
        let guess = (min + max) / 2;

        println!("Is your number {}?", guess);

        let mut input = String::new();
        print!("Enter 'h' if the number is too high, 'l' if the number is too low, or 'c' if the number is correct: ");
        io::stdout().flush().unwrap();
        input.clear();


        io::stdin().read_line(&mut input).unwrap();
        match input.trim() {
            "h" => {
                max = guess - 1;
                cnt += 1;
            },
            "l" => {
                min = guess + 1;
                cnt += 1;
            },
            "c" => {
                println!("I guessed your number {} in {} attempts!", guess, cnt);
                break;
            },
            _ => println!("Invalid option!"),
        }
    }
}