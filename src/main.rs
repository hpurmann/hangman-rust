const DEBUG: bool = true;
const MAXIMUM_TRIES: i8 = 7;

fn main() {
    let solution: String = get_solution();
    debug(solution);
    for i in 0..MAXIMUM_TRIES {
        println!("{}", i);
    }
}

fn get_solution() -> String {
    return "computer".to_string()
}

fn debug(message: String) {
    if DEBUG {
        println!("DEBUG: {}", message);
    }
}
