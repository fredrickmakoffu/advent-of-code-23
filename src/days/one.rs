// allow dead code
#![allow(dead_code)]
use std::collections::HashMap;

fn get_nums() -> HashMap<&'static str, u32> {
    let mut nums: HashMap<&str, u32> = HashMap::new();
    nums.insert("1", 1);
    nums.insert("2", 2);
    nums.insert("3", 3);
    nums.insert("4", 4);
    nums.insert("5", 5);
    nums.insert("6", 6);
    nums.insert("7", 7);
    nums.insert("8", 8);
    nums.insert("9", 9);
    nums.insert("one", 1);
    nums.insert("two", 2);
    nums.insert("three", 3);
    nums.insert("four", 4);
    nums.insert("five", 5);
    nums.insert("six", 6);
    nums.insert("seven", 7);
    nums.insert("eight", 8);
    nums.insert("nine", 9);

    nums
}

fn part_2(input: &str) -> u32 {
    // got this from here: https://dev.to/nickymeuleman/advent-of-code-2023-day-1-2o97
    let nums: HashMap<&'static str, u32> = get_nums();
    let mut sum = 0;
    
    for line in input.lines() {
        let mut forwards = line;
        let mut backwards = line;

        let first = 'outer: loop {
            for (prefix, num) in nums.iter() {
                if forwards.starts_with(prefix) {
                    break 'outer num;
                }
            }

            forwards = &forwards[1..];
        };

        let last = 'outer: loop {
            for (suffix, num) in nums.iter() {
                if backwards.ends_with(suffix) {
                    break 'outer num;
                }
            }
            backwards = &backwards[..backwards.len() - 1];
        };

        let num = first * 10 + last;
        sum += num;
    }

    sum
}

fn get_number_from_string(word: &str) -> String {
    let mut number: Vec<char> = Vec::new();

    for c in word.chars() {
        if c.is_numeric() {
            // add char to number
            number.push(c);
        }
    }

    // merge them and collect into a string but if length is 0 return 0
    return vec![number[0], number[number.len() - 1]]
        .into_iter()
        .collect();
}

fn commands(args: Vec<String>) -> String {
    return args[1].to_string();
}

pub fn handle(input: Vec<String>, args: Vec<String>) {
    let command: String = commands(args);
    let mut sum = 0;
    for line in input {
        let number: u32;
        if command == "part2" {
            number = part_2(&line);
        } else {
            number = get_number_from_string(&line).parse::<u32>().unwrap();
        }

        sum = sum + number;
    }

    print!("{}", sum);
}
