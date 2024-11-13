#![allow(dead_code)]
fn close_to_symbol(letter: char) -> bool {
    let symbols: Vec<char> = vec!['.', '*', '#', '$', '+'];
    for symbol in symbols {
        if letter == symbol {
            return true;
        }
    }

    return false;
}

fn part_1(_input: Vec<String>) {
    let input: &str = 
        "467..114..
        ...*......
        ..35..633.
        ......#...
        617*......
        .....+.58.
        ..592.....
        ......755.
        ...$.*....
        .664.598..";
    
    // remove whitespace
    let new_input: String = remove_whitespace(input);
    
    // check if symbol near each letter
    print!("{}", sum_chars_with_adjacent_symbols(&new_input));
}

fn sum_chars_with_adjacent_symbols(input: &String) -> i32 {
    let sum: i32 = 0;
    let mut i: usize = 0;

    while i < input.len() {
        let char: char = input.chars().nth(i).unwrap();
        if char.is_numeric() {
            // get the full digit
            let digit: Vec<String> = get_full_number(char, i, input);
            print!("{}", digit[0]);
        }

        i += 1;
    }

    sum
}

fn get_full_number(s: char, index: usize, input: &String) -> Vec<String> {
    print!("{}", s);
    let mut digit: String = "".to_string(); 
    let mut j: usize = 0;

    loop {
        let char = input.chars().nth(index+j).unwrap();
        if char.is_numeric() {
            // append char to digit
            digit.push(char)
        } else {
            break;
        }

        j += 1;
    }

    return vec![digit, (index+j).to_string()];
}

fn if_symbol_adjacent(s: char) -> bool {
    true
}

fn remove_whitespace(input: &str) -> String {
    let mut i: usize = 0;
    let mut new_input: String = "".to_string();
    
    while i < input.len() {
        let char = input.chars().nth(i).unwrap();

        if !char.is_whitespace() {
            // append char to new_input
            new_input.push(char);
        }

        i+= 1;
    }

    new_input
}
fn part_2(input: Vec<String>) {    
    print!("{}", input.len());
}

pub fn handle(input: Vec<String>, args: Vec<String>) {
    let command: String = args[1].to_string();

    if command == "part2" {
        part_2(input);
    } else {
        part_1(input);
    }
}
