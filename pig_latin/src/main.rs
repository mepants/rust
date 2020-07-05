//Implementation of Pig Latin exercise from Chapter 8 of The Rust Programming Language
use std::io;
use std::collections::HashSet;
use std::iter::FromIterator;

fn main() {

    let mut word = String::new();

    let VOWELS = {
        let vowel_vec = vec!['a', 'e', 'i', 'o', 'u'];
        //into_iter passes ownership, so we get chars instead of &chars
        HashSet::<char>::from_iter(vowel_vec.into_iter())
    };

    println!("Enter a word: ");
    io::stdin().read_line(&mut word).expect("Failed to read word.");
    let word = String::from(word.trim());

    let output = {
        match word.chars().next(){
            Some(first_char) => {
                if VOWELS.contains(&first_char) {
                    //The word starts with a vowel
                    let mut buf = word;
                    buf += "-hay";
                    buf
                }
                else{
                    //The word starts with a consonant
                    let mut buf = String::from_iter(word.chars().skip(1));
                    buf += "-";
                    buf.push(first_char);
                    buf += "ay";
                    buf
                }
            },
            None => {
                String::from("")
            }
        }
    };

    println!("{}", output);
}
