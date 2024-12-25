use anyhow::Result;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::Read;

const INPUT_FILE: &str = "input.txt";

pub fn read_input(file_path: &str) -> Result<String> {
    let mut file = File::open(file_path).expect("File not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Error reading file");
    Ok(contents)
}

fn find_orbit_path(orbits: &str) -> Option<usize> {
    let mut orbit_map: HashMap<String, Vec<String>> = HashMap::new();

    for line in orbits.lines() {
        let parts: Vec<&str> = line.split(')').collect();
        let parent = parts[0].to_string();
        let child = parts[1].to_string();

        orbit_map.entry(parent.clone()).or_default().push(child.clone());
        orbit_map.entry(child).or_default().push(parent);
    }

    // Find objects that YOU and SAN are orbiting
    let you_orbit = orbits.lines()
        .find(|line| line.split(')').nth(1).unwrap() == "YOU")
        .map(|line| line.split(')').next().unwrap())
        .unwrap();

    let san_orbit = orbits.lines()
        .find(|line| line.split(')').nth(1).unwrap() == "SAN")
        .map(|line| line.split(')').next().unwrap())
        .unwrap();

    // BFS to find shortest path
    let mut visited = HashSet::new();
    let mut queue = vec![(you_orbit.to_string(), 0)];
    visited.insert(you_orbit.to_string());

    while let Some((current, distance)) = queue.pop() {
        if current == san_orbit {
            return Some(distance);
        }

        if let Some(neighbors) = orbit_map.get(&current) {
            for next in neighbors {
                if !visited.contains(next) {
                    visited.insert(next.clone());
                    queue.push((next.clone(), distance + 1));
                }
            }
        }
    }

    None
}

fn main() {
    let contents = read_input(INPUT_FILE).unwrap_or_else(|err| panic!("{}", err));

    if let Some(distance) = find_orbit_path(&contents) {
        println!("Minimum orbital transfers required: {}", distance);
    } else {
        println!("No path found!");
    }
}
