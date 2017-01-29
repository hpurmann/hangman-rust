use std::io;
use std::collections::HashSet;

const DEBUG: bool = true;
const MAXIMUM_TRIES: i8 = 7;

fn main() {
    let solution: String = get_solution();
    let mut guessed_chars: HashSet<char> = HashSet::new();
    debug(&solution);

    for i in 0..MAXIMUM_TRIES {
        print_gap_word(&solution, &guessed_chars);
        let mut guess = String::new();
        println!("{} tries left.", MAXIMUM_TRIES-i);
        println!("Guess a character.");

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        let guessed_char = guess.as_str().chars().nth(0).unwrap();
        guessed_chars.insert(guessed_char);
    }
}

fn get_solution() -> String {
    // TODO Make program case-insensitive
    return "Unimaginatively".to_string()
}

fn debug(message: &String) {
    if DEBUG {
        println!("DEBUG: {}", message);
    }
}

fn print_gap_word(solution: &String, guessed_chars: &HashSet<char>) {
    for letter in solution.chars() {
        print!("{}", guessed_chars.get(&letter).unwrap_or(&'_'))
    }
    print!("\n");
}
