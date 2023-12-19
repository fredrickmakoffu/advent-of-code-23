use std::{fs, env};

pub fn get_file_contents(val: &str) -> Vec<String> {
    // convert val from &str to String
    let file_path: String = format!("./src/assets/data/input{}.txt", val);
    return fs::read_to_string(file_path)
        .unwrap()
        .split("\n")
        .map(|s| s.to_string())
        .collect();
}

pub fn commands() -> Vec<String> {
    return env::args().collect();
}