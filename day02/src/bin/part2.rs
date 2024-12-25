use anyhow::Result;
use std::fs::File;
use std::io::Read;

#[allow(dead_code)]
const INPUT_FILE: &str = "input.txt";
#[allow(dead_code)]
const EXAMPLE_FILE: &str = "example.txt";

/*
https://adventofcode.com/2019/day/02
*/

pub fn read_input(file_path: &str) -> Result<String> {
    let mut file = File::open(file_path).expect("File not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Error reading file");
    Ok(contents)
}

#[derive(PartialEq)]
struct Instruction {
    identifier: u32,
    clear_name: &'static str,
    cycles: u8,
}

const ADD: Instruction = Instruction {
    identifier: 1,
    clear_name: "ADD",
    cycles: 4,
};

const MULTIPLY: Instruction = Instruction {
    identifier: 2,
    clear_name: "MULTIPLY",
    cycles: 4,
};

const HALT: Instruction = Instruction {
    identifier: 99,
    clear_name: "HALT",
    cycles: 1,
};

const NOOP: Instruction = Instruction {
    identifier: 0,
    clear_name: "NOOP",
    cycles: 1,
};

fn get_instruction_by_id(id: u32) -> &'static Instruction {
    match id {
        1 => &ADD,
        2 => &MULTIPLY,
        99 => &HALT,
        _ => &NOOP,
    }
}


fn main() {
    let contents: String = read_input(INPUT_FILE).unwrap_or_else(|err| panic!("{}", err));
    let initial_data: Vec<u32> = contents
        .split(",")
        .map(|s| s.trim().parse::<u32>().expect("Invalid number NaN"))
        .collect();

    /*
        Add, Multiply are always followed by 3 parameters (index in the array)
        e.g.

        ADD      Source1 Source2 Destination
        MULTIPLY Source1 Source2 Destination
    */

    for noun in 0..=99 {
        for verb in 0..=99 {
            /*
            Noun = data[1]
            Verb = data[2]

            0..=99 both ends inclusive

            data[0] = 19690720
            */
            let mut data = initial_data.clone();

            data[1] = noun;
            data[2] = verb;

            let mut pc = 0;

            while pc < data.len() {
                let id = data[pc];
                let instruction = get_instruction_by_id(id);

                /*
                    Do the lookup and calculations
                */

                match *instruction {
                    ADD => {
                        let source1 = data[pc + 1] as usize;
                        let source2 = data[pc + 2] as usize;
                        let destination = data[pc + 3] as usize;
                        data[destination] = data[source1] + data[source2];
                    }
                    MULTIPLY => {
                        let source1 = data[pc + 1] as usize;
                        let source2 = data[pc + 2] as usize;
                        let destination = data[pc + 3] as usize;
                        data[destination] = data[source1] * data[source2];
                    }
                    HALT => {
                        break;
                    }
                    _ => panic!("Unknown INSTRUCTION"),
                }

                pc += instruction.cycles as usize;
            }
            if data[0] == 19690720 {
                let result = noun * 100 + verb;
                println!("Part2: Noun: {noun}, Verb: {verb} Result: {result}");
            }
        }
    }
}
