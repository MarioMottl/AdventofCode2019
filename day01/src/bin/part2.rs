use anyhow::Result;
use std::fs::File;
use std::io::Read;

#[allow(dead_code)]
const INPUT_FILE: &str = "input.txt";
#[allow(dead_code)]
const EXAMPLE_FILE: &str = "example.txt";

/*
https://adventofcode.com/2019/day/01
*/

pub fn read_input(file_path: &str) -> Result<String> {
    let mut file = File::open(file_path).expect("File not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Error reading file");
    Ok(contents)
}

fn calculate_fuel(mass: i32) -> i32 {
    let needed_fuel = mass / 3 - 2;
    if needed_fuel < 0 {
        0
    } else {
        needed_fuel
    }
}

fn calculate_fuel_recursive(mass: i32) -> i32 {
    let fuel = calculate_fuel(mass);
    if fuel <= 0 {
        0
    } else {
        fuel + calculate_fuel_recursive(fuel)
    }
}

fn main() {
    #[allow(unused_variables)]
    let contents: String = read_input(INPUT_FILE).unwrap_or_else(|err| panic!("{}", err));

    let total_fuel_needed = contents.lines().map(|line| {
        let mass: i32 = line.parse().expect("Couldnt parse str to int");
        calculate_fuel_recursive(mass)
    }).sum::<i32>();

    println!("Total fuel needed recursive: {}", total_fuel_needed);
}
