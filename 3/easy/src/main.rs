/*
[2/11/2012] Challenge #3 [easy]
Welcome to cipher day!

write a program that can encrypt texts with an alphabetical caesar cipher. This cipher can ignore numbers, symbols, and whitespace.

for extra credit, add a "decrypt" function to your program!
*/

use std::io::{self, Write};

fn main() {
    loop {
        println!("Enter command (encrypt, decrypt, quit):");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let input = input.trim();

        match input {
            "encrypt" => {
                println!("Enter text to encrypt:");
                let mut text = String::new();
                io::stdin().read_line(&mut text).expect("Failed to read line");
                let text = text.trim();
                let encrypted = encrypt(text, 13); // Using a shift of 13 for Caesar cipher
                println!("Encrypted text: {}", encrypted);
            }
            "decrypt" => {
                println!("Enter text to decrypt:");
                let mut text = String::new();
                io::stdin().read_line(&mut text).expect("Failed to read line");
                let text = text.trim();
                let decrypted = decrypt(text, 13); // Using a shift of 13 for Caesar cipher
                println!("Decrypted text: {}", decrypted);
            }
            "quit" => break,
            _ => println!("Invalid command"),
        }
    }
}

fn encrypt(text: &str, shift: u8) -> String {
    text.chars()
        .map(|c| {
            if c.is_ascii_alphabetic() {
                let first = if c.is_ascii_lowercase() { b'a' } else { b'A' };
                (first + (c as u8 - first + shift) % 26) as char
            } else {
                c
            }
        })
        .collect()
}

fn decrypt(text: &str, shift: u8) -> String {
    encrypt(text, 26 - shift) // Decryption is just encryption with 26 - shift
}