mod days {
    pub mod one;
    pub mod two;
}

pub mod helpers;
use days::{one, two};

fn main() {
    // manage arguments
    let args = helpers::commands();
    let question: &String = &args[1];
    let child_args: Vec<String> = args[1..].to_vec();

    // get input
    let input: Vec<String> = helpers::get_file_contents(question);

    // handle question
    match question.as_str() {
        "1" => one::handle(input, child_args),
        "2" => two::handle(input, child_args),
        _ => println!("Question not found"),
    }
}
