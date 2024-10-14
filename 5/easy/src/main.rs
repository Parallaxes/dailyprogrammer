/*
[2/12/2012] Challenge #5 [easy]
Your challenge for today is to create a program which is password protected, and wont open unless the correct user and password is given.

For extra credit, have the user and password in a seperate .txt file.

for even more extra credit, break into your own program :)
*/

use bcrypt::{hash, verify, DEFAULT_COST};
use rpassword::read_password;

fn main() {
    println!("Register a new user");
    let hashed_password = register_user();

    println!("Login:");
    let is_authenticated = login_user(&hashed_password);

    if is_authenticated {
        println!("You are logged in!");
    } else {
        println!("Invalid credentials!");
    }
}

// Function to register a user and hash the password
fn register_user() -> String {
    println!("Enter a password: ");
    
    // Read password securely from user input
    let password = read_password().expect("Failed to read password");

    // Hash the password using bcrypt
    let hashed_password = hash(password, DEFAULT_COST).expect("Failed to hash password");
    
    // In a real app, save `hashed_password` securely in your database
    hashed_password
}

// Function to authenticate a user
fn login_user(hashed_password: &str) -> bool {
    println!("Enter your password: ");
    
    // Read the user's input password
    let password = read_password().expect("Failed to read password");

    // Verify the entered password with the stored hash
    verify(password, hashed_password).unwrap_or(false)
}