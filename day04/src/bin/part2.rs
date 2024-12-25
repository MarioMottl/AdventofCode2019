use anyhow::Result;
use std::collections::HashSet;
use std::fs::File;
use std::io::Read;

#[allow(dead_code)]
const INPUT_FILE: &str = "input.txt";
#[allow(dead_code)]
const EXAMPLE_FILE: &str = "example.txt";

/*
https://adventofcode.com/2019/day/04
*/

pub fn read_input(file_path: &str) -> Result<String> {
    let mut file = File::open(file_path).expect("File not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Error reading file");
    Ok(contents)
}

fn contains_duplicates(code: &str) -> bool {
    let set: HashSet<char> = code.chars().collect();
    set.len() != 6
}

fn left_right_increase_value(code: &str) -> bool {
    code.chars().is_sorted()
}

fn contains_adjacent_duplicates(code: &str) -> bool {
    let chars: Vec<char> = code.chars().collect();
    let mut i = 0;

    while i < chars.len() - 1 {
        let current_char = chars[i];
        let mut count = 1;

        while i + 1 < chars.len() && chars[i + 1] == current_char {
            count += 1;
            i += 1;
        }

        if count == 2 {
            return true;
        }
        i += 1;
    }
    false
}


fn main() {
    #[allow(unused_variables)]
    let contents: String = read_input(INPUT_FILE).unwrap_or_else(|err| panic!("{}", err));
    let mut split = contents.split("-");
    let lower: u32 = split.next().unwrap().parse().unwrap();
    let upper: u32 = split.next().unwrap().parse().unwrap();

    let mut valid_codes: u32 = 0;

    for code in lower..upper {
        if contains_duplicates(&code.to_string())
            && left_right_increase_value(&code.to_string())
            && contains_adjacent_duplicates(&code.to_string())
        {
            valid_codes += 1;
        }
    }

    println!("Valid Codes: {}", valid_codes);
}

