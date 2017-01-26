use std::io;

const DEBUG: bool = true;
const MAXIMUM_TRIES: i8 = 7;

struct Digit {
    value: char,
    revealed: bool,
}

type GapWord = Vec<Digit>;

fn main() {
    let solution: String = get_solution();
    debug(&solution);

    let gap_word = init_gap_word(&solution);

    for i in 0..MAXIMUM_TRIES {
        print_gap_word(&gap_word);
        let mut guess = String::new();
        println!("{} tries left.", MAXIMUM_TRIES-i);
        println!("Guess a character.");

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        println!("{}", guess)
    }
}

fn get_solution() -> String {
    return "computer".to_string()
}

fn debug(message: &String) {
    if DEBUG {
        println!("DEBUG: {}", message);
    }
}

fn init_gap_word(string: &String) -> GapWord {
    let mut vector: Vec<Digit> = Vec::new();
    let str = string.as_str();
    for my_char in str.chars() {
        let current_digit = Digit {
            value: my_char,
            revealed: false,
        };
        vector.push(current_digit);
    }
    return vector;
}

fn print_gap_word(gap_word: &GapWord) {
    for digit in gap_word {
        let value_or_blank: char;
        if digit.revealed {
            value_or_blank = digit.value;
        } else {
            value_or_blank = "_".chars().nth(0).unwrap_or('_');
        }
        print!("{}", value_or_blank)
    }
    print!("\n");
}
