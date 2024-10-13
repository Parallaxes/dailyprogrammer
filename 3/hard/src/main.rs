/*
[2/11/2012] challenge #3 [difficult]
Welcome to cipher day!

For this challenge, you need to write a program that will take the scrambled words from this post, and compare them against THIS WORD LIST to unscramble them. For bonus points, sort the words by length when you are finished. Post your programs and/or subroutines!

Here are your words to de-scramble:

mkeart

sleewa

edcudls

iragoge

usrlsle

nalraoci

nsdeuto

amrhat

inknsy

iferkna
*/

use std::fs;

fn main() {
    let word_list = vec!["mkeart", "sleewa", "edcudls", "iragoge", "usrlsle", "nalraoci", "nsdeuto", "amrhat", "inknsy", "iferkna"];
    let base = fs::read_to_string("baselist.txt")
        .expect("Should have been able to read the file");

    // Probably a better way to do this but whatever
    for word in word_list {
        for base_word in base.lines() {
            let mut word_chars: Vec<char> = word.chars().collect();
            let mut base_word_chars: Vec<char> = base_word.chars().collect();
            word_chars.sort_unstable(); // Idk about this one...
            base_word_chars.sort_unstable(); // Or this
            if word_chars == base_word_chars {
                println!("{} unscrambled is {}", word, base_word);
            }
        }
    }
}