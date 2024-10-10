/*
 * [intermediate] challenge #1 
 * create a program that will allow you to enter events organizable by hour. There must be menu options of some form, and you must be able to easily edit, add, and delete events without directly changing the source code. 
 * 
 * (note that by menu i dont necessarily mean gui. as long as you can easily access the different options and receive prompts and instructions telling you how to use the program, it will probably be fine)
*/

use std::io::{self, Write};
use std::collections::HashMap;

fn main() {
    let mut input = String::new();

    let mut events = HashMap::new();

    loop {
        print_menu();
        io::stdout().flush().unwrap();
        input.clear();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim() {
            "1" => add_event(&mut events),
            "2" => edit_event(&mut events),
            "3" => delete_event(&mut events),
            "4" => list_events(&events),
            "5" => break,
            _ => println!("Invalid option!"),
        }
    }
}

fn print_menu() {
    println!("1. Add event");
    println!("2. Edit event");
    println!("3. Delete event");
    println!("4. List events");
    println!("5. Exit");
}

fn add_event(events: &mut HashMap<String, String>) {
    let mut input = String::new();

    let hour = prompt("Enter the hour: ", &mut input);
    let event = prompt("Enter the event: ", &mut input);

    events.insert(hour, event);
}

fn edit_event(events: &mut HashMap<String, String>) {
    let mut input = String::new();

    let hour = prompt("Enter the hour of the event to edit: ", &mut input);
    let event = prompt("Enter the new event: ", &mut input);

    events.insert(hour, event);
}

fn delete_event(events: &mut HashMap<String, String>) {
    let mut input = String::new();

    let hour = prompt("Enter the hour of the event to delete: ", &mut input);

    events.remove(&hour);
}

fn list_events(events: &HashMap<String, String>) {
    println!("+------+---------------------+");
    println!("| Hour |        Event        |");
    println!("+------+---------------------+");
    for (hour, event) in events {
        println!("| {:<4} | {:<19} |", hour, event);
    }
    println!("+------+---------------------+");
    for (hour, event) in events {
        println!("{}: {}", hour, event);
    }
}

fn prompt(message: &str, input: &mut String) -> String {
    print!("{}", message);
    io::stdout().flush().unwrap();
    input.clear();
    io::stdin().read_line(input).unwrap();
    input.trim().to_string()
}