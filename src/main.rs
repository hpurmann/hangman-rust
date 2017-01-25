use std::io;

const DEBUG: bool = true;
const MAXIMUM_TRIES: i8 = 7;

struct Digit {
    value: char,
    revealed: bool,
}

fn main() {
    let solution: String = get_solution();
    debug(&solution);

    let solution_vector = init_digits(&solution);
    for x in &solution_vector {
        println!("Value: {}, Revealed: {}", x.value, x.revealed)
    }

    for i in 0..MAXIMUM_TRIES {
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

fn init_digits(string: &String) -> Vec<Digit> {
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
