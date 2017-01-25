use std::io;

const DEBUG: bool = true;
const MAXIMUM_TRIES: i8 = 7;

fn main() {
    let solution: String = get_solution();
    debug(&solution);

    let solution_vector = string_to_vector(&solution);

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

fn string_to_vector(string: &String) -> Vec<char> {
    let mut vector: Vec<char> = Vec::new();
    let str = string.as_str();
    for my_char in str.chars() {
        vector.push(my_char);
    }
    return vector;
}
