/*
For today's challenge, you should calculate some simple statistical values based on a list of values. Given this data set, write functions that will calculate:

The mean value

The variance

The standard deviation

Obviously, many programming languages and environments have standard functions for these (this problem is one of the few that is really easy to solve in Excel!), but you are not allowed to use those! The point of this problem is to write the functions yourself.

Thanks to Cosmologicon for suggesting this problem at r/dailyprogrammer_ideas!
*/

use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Failed to read file");
    
    let numbers: Vec<f64> = contents.lines()
        .map(|line| line.parse().expect("Failed to parse number"))
        .collect();

    let mean = calculate_mean(&numbers);
    let variance = calculate_variance(&numbers, mean);
    let standard_deviation = calculate_standard_deviation(variance);

    println!("Mean: {}", mean);
    println!("Variance: {}", variance);
    println!("Standard deviation: {}", standard_deviation);
}

fn calculate_mean(numbers: &Vec<f64>) -> f64 {
    numbers.iter().sum::<f64>() / numbers.len() as f64
}

fn calculate_variance(numbers: &Vec<f64>, mean: f64) -> f64 {
    let sum_of_squares = numbers.iter()
        .map(|number| (number - mean).powi(2))
        .sum::<f64>();
    sum_of_squares / numbers.len() as f64
}

fn calculate_standard_deviation(variance: f64) -> f64 {
    variance.sqrt()
}