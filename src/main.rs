use std::io;
use std::collections::HashSet;

const DEBUG: bool = true;
const MAXIMUM_WRONG_ANSWERS: i8 = 7;

fn main() {
    let mut wrong_answers: i8 = 0;

    let solution: String = get_solution();
    let mut guessed_chars: HashSet<char> = HashSet::new();
    debug(&solution);

    loop {
        print_gap_word(&solution, &guessed_chars);

        if MAXIMUM_WRONG_ANSWERS == wrong_answers {
            println!("YOU LOSE!");
            break;
        }
        let mut guess = String::new();
        println!("{} tries left.", MAXIMUM_WRONG_ANSWERS - wrong_answers);
        println!("Guess a character.");

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        let guessed_char = guess.as_str().to_uppercase().chars().nth(0).unwrap();
        if !solution.contains(guessed_char) || guessed_chars.contains(&guessed_char) {
            wrong_answers += 1;
        }
        guessed_chars.insert(guessed_char);
    }
}

fn get_solution() -> String {
    return "Unimaginatively".to_string().to_uppercase();
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
