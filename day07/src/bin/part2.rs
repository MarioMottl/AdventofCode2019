use anyhow::Result;
use std::fs::File;
use std::io::Read;

#[allow(dead_code)]
const INPUT_FILE: &str = "input.txt";
#[allow(dead_code)]
const EXAMPLE_FILE: &str = "example.txt";

fn read_input(file_path: &str) -> Result<String> {
    let mut file = File::open(file_path).expect("File not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Error reading file");
    Ok(contents)
}

#[derive(Clone, Debug)]
struct AMP {
    phase_setting: i32,
}

struct AmplificationCircuit {
    stages: Vec<AMP>,
}

impl AmplificationCircuit {
    fn new() -> Self {
        // Initialize with 5 amplifiers
        AmplificationCircuit {
            stages: vec![AMP { phase_setting: 0 }; 5],
        }
    }

    // Generate next permutation of phase settings (0-4)
    fn next_permutation(&mut self) -> bool {
        let n = self.stages.len();
        let mut i = n - 1;

        // Find the largest index i such that stages[i-1] < stages[i]
        while i > 0 && self.stages[i - 1].phase_setting >= self.stages[i].phase_setting {
            i -= 1;
        }

        // If no such index exists, we're at the last permutation
        if i == 0 {
            return false;
        }

        // Find the largest index j such that stages[j] > stages[i-1]
        let mut j = n - 1;
        while j >= i && self.stages[j].phase_setting <= self.stages[i - 1].phase_setting {
            j -= 1;
        }

        // Swap the elements at i-1 and j
        let temp = self.stages[i - 1].phase_setting;
        self.stages[i - 1].phase_setting = self.stages[j].phase_setting;
        self.stages[j].phase_setting = temp;

        // Reverse the sequence from i to the end
        let mut left = i;
        let mut right = n - 1;
        while left < right {
            let temp = self.stages[left].phase_setting;
            self.stages[left].phase_setting = self.stages[right].phase_setting;
            self.stages[right].phase_setting = temp;
            left += 1;
            right -= 1;
        }

        true
    }
}

#[derive(PartialEq)]
struct OPCode {
    identifier: u32,
    clear_name: &'static str,
    cycles: u8,
}

struct Instruction {
    op_code: OPCode,
    third_parameter_mode: u32,
    second_parameter_mode: u32,
    first_parameter_mode: u32,
}

const NOOP: OPCode = OPCode {
    identifier: 0,
    clear_name: "NOOP",
    cycles: 1,
};

const ADD: OPCode = OPCode {
    identifier: 1,
    clear_name: "ADD",
    cycles: 4,
};

const MULTIPLY: OPCode = OPCode {
    identifier: 2,
    clear_name: "MULTIPLY",
    cycles: 4,
};

const INPUT: OPCode = OPCode {
    identifier: 3,
    clear_name: "INPUT",
    cycles: 2,
};

const OUTPUT: OPCode = OPCode {
    identifier: 4,
    clear_name: "OUTPUT",
    cycles: 2,
};

const JUMP_IF_TRUE: OPCode = OPCode {
    identifier: 5,
    clear_name: "JUMP_IF_TRUE",
    cycles: 3,
};

const JUMP_IF_FALSE: OPCode = OPCode {
    identifier: 6,
    clear_name: "JUMP_IF_FALSE",
    cycles: 3,
};

const LESS_THAN: OPCode = OPCode {
    identifier: 7,
    clear_name: "LESS_THAN",
    cycles: 4,
};

const EQUALS: OPCode = OPCode {
    identifier: 8,
    clear_name: "EQUALS",
    cycles: 4,
};

const HALT: OPCode = OPCode {
    identifier: 99,
    clear_name: "HALT",
    cycles: 1,
};

fn split_into_digits(n: u32) -> Vec<u32> {
    let mut digits: Vec<u32> = n
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect();

    if digits.len() < 5 {
        for _ in 1..=5 - digits.len() {
            digits.insert(0, 0);
        }
    }
    digits
}

/*
ABCDE
01002

DE - two-digit opcode,      02 == opcode 2
 C - mode of 1st parameter,  0 == position mode
 B - mode of 2nd parameter,  1 == immediate mode
 A - mode of 3rd parameter,  0 == position mode
*/
fn get_instruction_by_id(id: u32) -> Instruction {
    let digits = split_into_digits(id);
    assert_eq!(digits.len(), 5, "digits.len() < 5");
    let a = digits[0];
    let b = digits[1];
    let c = digits[2];
    let de = digits[3] * 10 + digits[4];
    Instruction {
        op_code: {
            match de {
                1 => ADD,
                2 => MULTIPLY,
                3 => INPUT,
                4 => OUTPUT,
                5 => JUMP_IF_TRUE,
                6 => JUMP_IF_FALSE,
                7 => LESS_THAN,
                8 => EQUALS,
                99 => HALT,
                _ => NOOP,
            }
        },
        third_parameter_mode: a,
        second_parameter_mode: b,
        first_parameter_mode: c,
    }
}

fn run_program(mut source_code: Vec<i32>, phase_setting: i32, input_signal: i32) -> i32 {
    let mut pc = 0;
    let mut first_input = true;
    let mut output_value = -1;

    while pc < source_code.len() {
        let id = source_code[pc];
        let mut pc_increment = true;
        assert!(id > 0);
        let instruction = get_instruction_by_id(id as u32);

        match instruction.op_code {
            ADD => {
                let source1 = source_code[pc + 1];
                let source2 = source_code[pc + 2];
                let destination = source_code[pc + 3];
                assert_eq!(instruction.third_parameter_mode, 0);

                let lhs = if instruction.first_parameter_mode == 1 {
                    source1
                } else {
                    assert!(source1 >= 0);
                    source_code[source1 as usize]
                };
                let rhs = if instruction.second_parameter_mode == 1 {
                    source2
                } else {
                    assert!(source2 >= 0);
                    source_code[source2 as usize]
                };
                assert!(destination >= 0);
                source_code[destination as usize] = lhs + rhs;
            }
            MULTIPLY => {
                let source1 = source_code[pc + 1];
                let source2 = source_code[pc + 2];
                let destination = source_code[pc + 3];
                assert_eq!(instruction.third_parameter_mode, 0);

                let lhs = if instruction.first_parameter_mode == 1 {
                    source1
                } else {
                    assert!(source1 >= 0);
                    source_code[source1 as usize]
                };
                let rhs = if instruction.second_parameter_mode == 1 {
                    source2
                } else {
                    assert!(source2 >= 0);
                    source_code[source2 as usize]
                };
                assert!(destination >= 0);
                source_code[destination as usize] = lhs * rhs;
            }
            INPUT => {
                let destination = source_code[pc + 1] as usize;
                // First input is phase setting, second is input signal
                source_code[destination] = if first_input {
                    first_input = false;
                    phase_setting
                } else {
                    input_signal
                };
            }
            OUTPUT => {
                let source = source_code[pc + 1];
                output_value = if instruction.first_parameter_mode == 1 {
                    source
                } else {
                    assert!(source >= 0);
                    source_code[source as usize]
                };
            }
            JUMP_IF_TRUE => {
                let source = source_code[pc + 1];
                let destination = source_code[pc + 2];

                let lhs = if instruction.first_parameter_mode == 1 {
                    source
                } else {
                    assert!(source >= 0);
                    source_code[source as usize]
                };
                let rhs = if instruction.second_parameter_mode == 1 {
                    destination
                } else {
                    assert!(destination >= 0);
                    source_code[destination as usize]
                };

                if lhs != 0 {
                    assert!(rhs >= 0);
                    pc = rhs as usize;
                    pc_increment = false;
                }
            }
            JUMP_IF_FALSE => {
                let source = source_code[pc + 1];
                let destination = source_code[pc + 2];

                let lhs = if instruction.first_parameter_mode == 1 {
                    source
                } else {
                    assert!(source >= 0);
                    source_code[source as usize]
                };
                let rhs = if instruction.second_parameter_mode == 1 {
                    destination
                } else {
                    assert!(destination >= 0);
                    source_code[destination as usize]
                };

                if lhs == 0 {
                    assert!(rhs >= 0);
                    pc = rhs as usize;
                    pc_increment = false;
                }
            }
            LESS_THAN => {
                let source1 = source_code[pc + 1];
                let source2 = source_code[pc + 2];
                let destination = source_code[pc + 3];
                assert_eq!(instruction.third_parameter_mode, 0);

                let lhs = if instruction.first_parameter_mode == 1 {
                    source1
                } else {
                    assert!(source1 >= 0);
                    source_code[source1 as usize]
                };
                let rhs = if instruction.second_parameter_mode == 1 {
                    source2
                } else {
                    assert!(source2 >= 0);
                    source_code[source2 as usize]
                };

                if lhs < rhs {
                    source_code[destination as usize] = 1;
                } else {
                    source_code[destination as usize] = 0;
                }
            }
            EQUALS => {
                let source1 = source_code[pc + 1];
                let source2 = source_code[pc + 2];
                let destination = source_code[pc + 3];
                assert_eq!(instruction.third_parameter_mode, 0);

                let lhs = if instruction.first_parameter_mode == 1 {
                    source1
                } else {
                    assert!(source1 >= 0);
                    source_code[source1 as usize]
                };
                let rhs = if instruction.second_parameter_mode == 1 {
                    source2
                } else {
                    assert!(source2 >= 0);
                    source_code[source2 as usize]
                };

                if lhs == rhs {
                    source_code[destination as usize] = 1;
                } else {
                    source_code[destination as usize] = 0;
                }
            }
            HALT => {
                break;
            }
            _ => panic!("Unknown INSTRUCTION"),
        }
        if pc_increment {
            pc += instruction.op_code.cycles as usize;
        }
    }
    output_value
}

fn main() {
    let contents: String = read_input(INPUT_FILE).unwrap_or_else(|err| panic!("{}", err));
    let program: Vec<i32> = contents
        .split(",")
        .map(|s| s.trim().parse::<i32>().expect("Invalid number NaN"))
        .collect();

    let mut amp_circuit = AmplificationCircuit::new();
    let mut max_thruster_signal = 0;

    // Initialize first permutation (0,1,2,3,4)
    for i in 0..5 {
        amp_circuit.stages[i].phase_setting = i as i32;
    }

    loop {
        let mut signal = 0;  // Initial input signal is 0

        // Run through all amplifiers in sequence
        for amp in &amp_circuit.stages {
            signal = run_program(program.clone(), amp.phase_setting, signal);
        }

        max_thruster_signal = max_thruster_signal.max(signal);

        // Generate next permutation of phase settings
        if !amp_circuit.next_permutation() {
            break;
        }
    }

    println!("Max Thruster Signal: {max_thruster_signal}");
}
