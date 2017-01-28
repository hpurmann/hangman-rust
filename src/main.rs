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

    let mut gap_word = init_gap_word(&solution);

    for i in 0..MAXIMUM_TRIES {
        print_gap_word(&gap_word);
        let mut guess = String::new();
        println!("{} tries left.", MAXIMUM_TRIES-i);
        println!("Guess a character.");

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        let guessed_char = guess.as_str().chars().nth(0);

        gap_word = update_gap_word(&gap_word, &guessed_char);
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
    let mut vector: GapWord = Vec::new();
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

fn update_gap_word(gap_word: &GapWord, guessed_char: &Option<char>) -> GapWord {
    let mut vector: GapWord = Vec::new();
    for digit in gap_word {
        let current_digit: Digit;
        if digit.value == guessed_char.unwrap() {
            current_digit = Digit {
                value: digit.value,
                revealed: true,
            };
        }
        else {
            current_digit = Digit {
                value: digit.value,
                revealed: if digit.revealed { digit.revealed } else { false },
            };
    }
        vector.push(current_digit)
    }
    return vector;
}
