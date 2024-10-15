/*
[2/14/2012] Challenge #6 [easy]
You're challenge for today is to create a program that can calculate pi accurately to at least 30 decimal places.

Try not to cheat :)
*/

fn factorial(n: u32) -> u32 {
    (1..=n).product()
}

fn main() {
    // Leibniz formula for pi
    /*
    let mut pi: f64 = 0.0;

    for i in 0..1_000_000_000 {
        pi += (-1.0_f64).powi(i) / (2 * i + 1) as f64;
    }

    pi *= 4.0;

    println!("Received value: {:.30}", pi);
    println!("Expected value: 3.141592653589793238462643383279");
    */

    // Ramanujan-Sato formula for pi
    let mut sum: f64 = 0.0;
    let max_elem: u32 = 3;

    for n in 0..max_elem {
        let first: f64 = factorial(4 * n) as f64 / (u32::pow(4, n) * factorial(n)).pow(4) as f64;

        let second: f64 = (23690 * n + 1103) as f64 / (99_f64.powf((4 * n) as f64));

        let sum_elem: f64 = first * second;

        sum += sum_elem
    }

    let fconst: f64 = 8_f64.sqrt() / (99_f64.powf(2_f64));

    let result: f64 = 1_f64 / (fconst * sum);

    let fconst: f64 = 8_f64.sqrt() / (99_f64.powf(2_f64));

    let result: f64 = 1_f64 / (fconst * sum);

    println!("Received value: {:.30}", result);
    println!("Expected value: 3.141592653589793238462643383279");
}

// I have no idea what I'm doing