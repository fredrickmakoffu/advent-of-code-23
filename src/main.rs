mod days {
    pub mod one;
    pub mod two;
}

// use days::one;
use days::two;
use std::env;
use std::fs;

fn get_file_contents(val: &str) -> Vec<String> {
    // convert val from &str to String
    let file_path: String = format!("./src/assets/data/input{}.txt", val);
    return fs::read_to_string(file_path)
        .unwrap()
        .split("\n")
        .map(|s| s.to_string())
        .collect();
}

fn main() {
    // manage arguments
    let args: Vec<String> = env::args().collect();
    let question: &String = &args[1];
    let child_args: Vec<String> = args[1..].to_vec();

    // get input
    let input: Vec<String> = get_file_contents(question);

    // handle question
    match question.as_str() {
        "1" => print!("{}", "test"),
        "2" => two::handle(input, child_args),
        _ => println!("Question not found"),
    }
}
