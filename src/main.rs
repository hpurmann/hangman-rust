#[macro_use]
extern crate serde_derive;

extern crate serde_json;
use std::io;
use std::collections::HashSet;

mod words;

const DEBUG: bool = true;
const MAXIMUM_WRONG_ANSWERS: i8 = 7;

fn main() {
    let mut wrong_answers: i8 = 0;

    let solution: String = words::get_random();
    let mut guessed_chars: HashSet<char> = HashSet::new();
    debug(&solution);

    loop {
        let gap_word = get_gap_word(&solution, &guessed_chars);
        println!("{} ({}/{} wrong answers)", gap_word, wrong_answers, MAXIMUM_WRONG_ANSWERS);

        if !gap_word.contains('_') {
            println!("YOU WIN!");
            break;
        }

        if MAXIMUM_WRONG_ANSWERS == wrong_answers {
            println!("YOU LOSE!");
            break;
        }

        let mut guess = String::new();
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");
        let guessed_char = guess.as_str().to_uppercase().chars().nth(0).unwrap();

        if !solution.contains(guessed_char) || guessed_chars.contains(&guessed_char) {
            wrong_answers += 1;
        }
        guessed_chars.insert(guessed_char);
    }
}

fn debug(message: &String) {
    if DEBUG {
        println!("DEBUG: {}", message);
    }
}

fn get_gap_word(solution: &String, guessed_chars: &HashSet<char>) -> String {
    let mut gap_word: String = String::new();
    for letter in solution.chars() {
        gap_word.push(*guessed_chars.get(&letter).unwrap_or(&'_'));
    }
    return gap_word;
}
