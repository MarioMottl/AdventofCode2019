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

fn calculate_steps_to_point(path: &[Point], point: &Point) -> Option<usize> {
    //.position is short circuit as soon as it finds the point it will return true
    // OFF BY ONE ERROR CAUSE BY ORIGIN POINT NOT BEING IN THE LIST
    path.iter().position(|p| p == point).map(|index| index + 1)
}

fn fewest_combined_steps(paths: &[Vec<Point>], overlaps: &[Point]) -> Option<usize> {
    let steps: Vec<usize> = overlaps
        .iter()
        .filter_map(|point| {
            let steps1 = calculate_steps_to_point(&paths[0], point)?;
            let steps2 = calculate_steps_to_point(&paths[1], point)?;
            Some(steps1 + steps2)
        })
        .collect::<Vec<usize>>();
    steps.iter().min().copied()
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

    /*
    Fewest combined steps to reach an intersection
        Iterate over the overlaps and check the first occurance in the 2 paths
     */

    println!("Fewest combined steps: {}", fewest_combined_steps(&all_paths, &overlaps).unwrap_or(0) as u32)
}
