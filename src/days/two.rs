#![allow(dead_code)]
use std::collections::HashMap;

fn commands(args: Vec<String>) -> String {
    return args[1].to_string();
}

const TEST_STRING : &str = "12 red cubes, 13 green cubes, and 14 blue cubes";
const COLORS : [&str; 3] = ["red", "green", "blue"];

fn part_1(input: Vec<String>) {
    let test: HashMap<&str, i32> = cubes_to_map(TEST_STRING);
    let mut sum: i32 = 0;
    for line in input {
        let game_split: Vec<&str> = line.split(": ").collect::<Vec<&str>>();
        let game_num: &str = game_split[0].split(" ").collect::<Vec<&str>>()[1];
        let mut game_possible = true;

        for cubes in game_split[1].split("; ") {
            let cubes_map: HashMap<&str, i32> = cubes_to_map(cubes);

            // insert for each color its count in the cubes_map hashmap
            for color in COLORS {
                let count = cubes_map.get(color).unwrap_or(&0);

                if count > test.get(color).unwrap_or(&0) {
                    game_possible = false;
                    break;
                }
            }
        }

        if game_possible {
            sum += game_num.parse::<i32>().unwrap();
        }
    }

    // print length of hashmap
    println!("{:?} and sum {}", test, sum);
}

fn part_2(input: Vec<String>) {    
    let mut sum: i32 = 0;
    for line in input {
        let game_split: Vec<&str> = line.split(": ").collect::<Vec<&str>>();
        let mut lowest: HashMap<&str, i32> = HashMap::new();
        lowest.insert("red", 0);
        lowest.insert("green", 0);
        lowest.insert("blue", 0);

        for cubes in game_split[1].split("; ") {
            let cubes_map: HashMap<&str, i32> = cubes_to_map(cubes);

            // insert for each color its count in the cubes_map hashmap
            for color in COLORS {
                let count = cubes_map.get(color).unwrap_or(&0);
                if count > lowest.get(color).unwrap_or(&0) {
                    // update lowest
                    lowest.insert(color, *count);
                }
            }
        }

        // multiply the three values in lowest 
        sum += lowest.values().fold(1, |acc, x| acc * x);
    }

    // print length of hashmap
    println!("{}", sum);
}

pub fn handle(input: Vec<String>, args: Vec<String>) {
    let command: String = commands(args);

    if command == "part2" {
        part_2(input);
    } else {
        part_1(input);
    }
}

fn cubes_to_map(cubes: &str) -> HashMap<&str, i32> {
    let mut map:HashMap<&str, i32> = HashMap::new();
    
    for cube in cubes.split(", ") {
        let cube_split = cube.split(" ").collect::<Vec<&str>>();
        let mut num: i32 = 0;
        let mut color: &str = "";

        for split in cube_split {
            if check_if_number(split) {
                num = split.parse::<i32>().unwrap();
            } else {
                if check_if_color(split) {
                    color = split;
                }
            }
        }

        if color != "" && num != 0 {
            map.insert(color, num);
        }
    }
    
    return map;
}

fn check_if_color(val: &str) -> bool {
    let colors: [&str; 3] = ["red", "green", "blue"];

    return colors.contains(&val);
}

fn check_if_number(test: &str) -> bool {
    let test_num = test.parse::<i32>();
    match test_num {
        Ok(_n) => return true,
        Err(_e) => return false,
    }
}
