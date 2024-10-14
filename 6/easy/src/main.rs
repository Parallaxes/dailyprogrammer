/*
[2/14/2012] Challenge #6 [easy]
You're challenge for today is to create a program that can calculate pi accurately to at least 30 decimal places.

Try not to cheat :)
*/

// Idk how to approximate, too lazy, WIP
fn main() {
    let mut pi = 0.0;

    for i in 0..1_000_000 {
        pi += (-1.0_f64).powi(i) / (2 * i + 1) as f64;
    }

    pi *= 4.0;

    println!("Pi: {:.30}", pi);
}