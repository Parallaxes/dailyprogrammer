/*
[2/12/2012] Challenge #4 [easy]
You're challenge for today is to create a random password generator!

For extra credit, allow the user to specify the amount of passwords to generate.

For even more extra credit, allow the user to specify the length of the strings he wants to generate!
*/

use rand::Rng;

fn main() {
    println!("Enter the number of passwords to generate:");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let num_passwords: u32 = input.trim().parse().unwrap();

    println!("Enter the length of the passwords:");
    input.clear();
    std::io::stdin().read_line(&mut input).unwrap();
    let length: u32 = input.trim().parse().unwrap();

    generate_passwords(num_passwords, length);
}

fn generate_passwords(num_passwords: u32, length: u32) {
    for _ in 0..num_passwords {
        let password: String = (0..length)
            .map(|_| {
            let mut rng = rand::thread_rng();
            let choice = rng.gen_range(0..3);
            match choice {
                0 => char::from_u32(rng.gen_range(48..58)).unwrap(), // 0-9
                1 => char::from_u32(rng.gen_range(65..91)).unwrap(), // A-Z
                2 => char::from_u32(rng.gen_range(97..123)).unwrap(), // a-z
                _ => panic!("This should never happen"),
            }
            })
            .collect();
        println!("{}", password);
    }
}