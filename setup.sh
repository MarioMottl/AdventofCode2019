#!/bin/bash

YEAR_NUMBER=2019

if [ "$#" -ne 1 ]; then
    echo "Usage: $0 <day>"
    exit 1
fi

DAY=$1
# shellcheck disable=SC2046
DAY_NUMBER=$(printf "%02d" $(echo "$DAY" | sed 's/^day0*//'))
DAY="day$DAY_NUMBER"

# Create a new cargo project.
cargo new --bin "$DAY"
mkdir -p "$DAY"/src/bin

# Download input and puzzle.
aoc -d "$DAY_NUMBER" -y "$YEAR_NUMBER" download -i "$DAY/input.txt" -p "$DAY/puzzle.md"

# Remove main.rs if it exists
if [ -f "$DAY"/src/main.rs ]; then
    rm "$DAY"/src/main.rs
fi

TEMPLATE_CODE=$(cat <<EOF
use anyhow::Result;
use std::fs::File;
use std::io::Read;

#[allow(dead_code)]
const INPUT_FILE: &str = "input.txt";
#[allow(dead_code)]
const EXAMPLE_FILE: &str = "example.txt";

/*
https://adventofcode.com/$YEAR_NUMBER/day/$DAY_NUMBER
*/

pub fn read_input(file_path: &str) -> Result<String> {
    let mut file = File::open(file_path).expect("File not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Error reading file");
    Ok(contents)
}

fn main() {
    #[allow(unused_variables)]
    let contents: String = read_input(INPUT_FILE).unwrap_or_else(|err| panic!("{}", err));
}
EOF
)

# Write the template code to part1.rs and part2.rs
touch "$DAY"/src/bin/part1.rs
touch "$DAY"/src/bin/part2.rs
echo "$TEMPLATE_CODE" > "$DAY"/src/bin/part1.rs
echo "$TEMPLATE_CODE" > "$DAY"/src/bin/part2.rs

cat <<EOF > "$DAY"/Cargo.toml
[package]
name = "$DAY"
version = "0.1.0"
edition = "2021"

[dependencies]
anyhow = "1.0.93"

[[bin]]
name = "part1"
path = "src/bin/part1.rs"

[[bin]]
name = "part2"
path = "src/bin/part2.rs"

[profile.release]
opt-level = 3
lto = true
codegen-units = 1
panic = "abort"
EOF
