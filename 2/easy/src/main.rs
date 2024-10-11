/*
[easy] challenge #2
Hello, coders! An important part of programming is being able to apply your programs, so your challenge for today is to create a calculator application that has use in your life. It might be an interest calculator, or it might be something that you can use in the classroom. For example, if you were in physics class, you might want to make a F = M * A calc.

EXTRA CREDIT: make the calculator have multiple functions! Not only should it be able to calculate F = M * A, but also A = F/M, and M = F/A!
*/

use std::io;

fn main() {
    println!("Euler's Totient calculator");

    let mut n = String::new();

    println!("Enter a number to calculate its Euler's Totient");
    io::stdin()
        .read_line(&mut n)
        .expect("Failed to read line");

    let n: i32 = n.trim().parse().expect("Please type a number!");

    let result = euler_totient(n);

    println!("Euler's Totient of {} is {}", n, result);
}

fn euler_totient(mut n: i32) -> i32 {
    let mut result = n;
    let mut i = 2;

    while i * i <= n {
        if n % i == 0 {
            while n % i == 0 {
                n /= i;
            }
            result -= result / i;
        }
        i += 1;
    }

    if n > 1 {
        result -= result / n;
    }

    result
}