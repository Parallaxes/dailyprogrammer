/*
[11/6/2012] Challenge #111 [Easy] Star delete
Write a function that, given a string, removes from the string any * character, or any character that's one to the left or one to the right of a * character. Examples:

"adf*lp" --> "adp"
"a*o" --> ""
"*dech*" --> "ec"
"de**po" --> "do"
"sa*n*ti" --> "si"
"abc" --> "abc"

Thanks to user larg3-p3nis for suggesting this problem in r/dailyprogrammer_ideas!
*/

use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Failed to read line");
    let output = star_delete(input);

    println!("Output: {}", output);
}

fn star_delete(input: String) -> String {
    let mut output = String::new();
    let chars = input.chars().collect::<Vec<char>>();

    for i in 0..chars.len() {
        if chars[i] == '*' {
            continue;
        }   

        if i > 0 && chars[i - 1] == '*' {
            continue;
        }

        if i < chars.len() - 1 && chars[i + 1] == '*' {
            continue;
        }

        output.push(chars[i]);
    }

    output
}