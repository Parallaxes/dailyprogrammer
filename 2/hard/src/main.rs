/*
[difficult] challenge #2
Your mission is to create a stopwatch program. this program should have start, stop, and lap options, and it should write out to a file to be viewed later.
*/

use std::io::{self, Write};
use std::time::{Duration, Instant};
use std::fs::OpenOptions;

fn main() {
    let mut laps = Vec::new();
    let mut start_time = None;
    let mut running = false;

    loop {
        println!("Enter command (start, stop, lap, quit):");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let input = input.trim();

        match input {
            "start" => {
                if running {
                    println!("Stopwatch is already running.");
                } else {
                    start_time = Some(Instant::now());
                    running = true;
                    println!("Stopwatch started.");
                }
            }
            "stop" => {
                if running {
                    let elapsed = start_time.unwrap().elapsed();
                    laps.push(elapsed);
                    running = false;
                    println!("Stopwatch stopped. Elapsed time: {:?}", elapsed);
                    write_to_file(&laps).expect("Failed to write to file");
                } else {
                    println!("Stopwatch is not running.");
                }
            }
            "lap" => {
                if running {
                    let elapsed = start_time.unwrap().elapsed();
                    laps.push(elapsed);
                    println!("Lap recorded. Elapsed time: {:?}", elapsed);
                } else {
                    println!("Stopwatch is not running.");
                }
            }
            "quit" => {
                if running {
                    let elapsed = start_time.unwrap().elapsed();
                    laps.push(elapsed);
                    running = false;
                    println!("Stopwatch stopped. Elapsed time: {:?}", elapsed);
                }
                write_to_file(&laps).expect("Failed to write to file");
                break;
            }
            _ => println!("Invalid command."),
        }
    }
}

fn write_to_file(laps: &[Duration]) -> io::Result<()> {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("laps.txt")?;
    for (i, lap) in laps.iter().enumerate() {
        writeln!(file, "Lap {}: {:?}", i + 1, lap)?;
    }
    Ok(())
}