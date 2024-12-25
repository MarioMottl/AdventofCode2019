use anyhow::Result;
use std::collections::HashSet;
use std::fmt;
use std::fs::File;
use std::io::Read;

#[allow(dead_code)]
const INPUT_FILE: &str = "input.txt";
#[allow(dead_code)]
const EXAMPLE_FILE: &str = "example.txt";

/*
https://adventofcode.com/2019/day/03
*/

pub fn read_input(file_path: &str) -> Result<String> {
    let mut file = File::open(file_path).expect("File not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Error reading file");
    Ok(contents)
}

#[derive(PartialEq, Debug, Clone)]
struct Instruction {
    direction: char,
    steps: i32,
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.direction {
            'U' => write!(f, "Direction: ⬆️  Steps: {}", self.steps),
            'D' => write!(f, "Direction: ⬇️  Steps: {}", self.steps),
            'L' => write!(f, "Direction: ⬅️  Steps: {}", self.steps),
            'R' => write!(f, "Direction: ➡️  Steps: {}", self.steps),
            _ => panic!("Should not be possible"),
        }
    }
}

fn parse_instruction(instruction: &str) -> Instruction {
    let (direction, steps) = instruction.split_at(1);
    Instruction {
        direction: direction.chars().next().unwrap(),
        steps: steps.parse().unwrap(),
    }
}

fn execute_instruction(instruction: Instruction, current_position: &mut Point) -> Vec<Point> {
    let mut path: Vec<Point> = Vec::new();

    for _ in 0..instruction.steps {
        match instruction.direction {
            'U' => current_position.y += 1,
            'D' => current_position.y -= 1,
            'L' => current_position.x -= 1,
            'R' => current_position.x += 1,
            _ => panic!("Unknown direction"),
        }
        path.push(current_position.clone());
    }

    path
}
#[derive(PartialEq, Eq, Hash, Debug, Clone)]
struct Point {
    x: i32,
    y: i32,
}

const ORIGIN: Point = Point { x: 0, y: 0 };

fn calculate_manhattan_distance(point: &Point) -> u32 {
    /*
    Manhattan Distance
        |x1 - x2| + |y1 - y2|
    */

    ((ORIGIN.x - point.x).abs() + (ORIGIN.y - point.y).abs()) as u32
}

fn find_overlaps(paths: &[Vec<Point>]) -> Vec<Point> {
    if paths.len() < 2 {
        return Vec::new();
    }

    // Convert each path to a set of points
    let mut sets: Vec<HashSet<Point>> = paths
        .iter()
        .map(|path| path.iter().cloned().collect())
        .collect();

    // Start with the set of the first path
    let mut intersection = sets.remove(0);

    // Compute the intersection with the remaining sets
    for set in sets {
        intersection = intersection.intersection(&set).cloned().collect();
    }

    intersection.into_iter().collect()
}

fn main() {
    #[allow(unused_variables)]
    let contents: String = read_input(INPUT_FILE).unwrap_or_else(|err| panic!("{}", err));

    let closest_crossing: Vec<Vec<Instruction>> = contents
        .lines()
        .map(|line| {
            line.split(",")
                .map(|instruction| {
                    let instruction = instruction.trim();
                    parse_instruction(instruction)
                })
                .collect()
        })
        .collect();

    /*
    Execute all Instructions
        Check for overlaps
        Return the smalles Manhattan Distance
    */
    let mut all_paths: Vec<Vec<Point>> = Vec::new();

    for line in closest_crossing {
        let mut path: Vec<Point> = Vec::new();
        let mut current_position = ORIGIN;

        for instruction in line {
            let temp_path = execute_instruction(instruction, &mut current_position);
            for step in temp_path {
                path.push(step);
            }
        }
        all_paths.push(path);
    }

    let overlaps = find_overlaps(&all_paths);

    let smallest_distance = overlaps
        .iter()
        .map(calculate_manhattan_distance)
        .min()
        .unwrap_or(0);

    println!("Smalles distance: {}", smallest_distance);
}
