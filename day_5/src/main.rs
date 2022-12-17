use std::{env, fs, io, time::Instant, vec};

#[derive(Debug, Clone, Copy)]
struct Instruction {
    Move: u16,
    From: u16,
    To: u16,
}
fn main() {
    let now = Instant::now();
    let args: Vec<String> = env::args().collect();
    // let file_path: &str = &args[1];
    let file_path: &str = "./puzzle/input.txt";
    let contents: String = read_file(file_path);
    // do something with the contents
    let (stacks, instructions): (Vec<Vec<String>>, Vec<Instruction>) = parse_input(contents);
    let moved_single_crates: Vec<Vec<String>> = move_single_crates(&stacks, &instructions);
    let moved_multiple_crates: Vec<Vec<String>> = move_multiple_crates(&stacks, &instructions);
    let result_part1 = get_top(moved_single_crates);
    let result_part2 = get_top(moved_multiple_crates);
    println!("Results => 1: {result_part1:?}");
    println!("Results => 2: {result_part2:?}");

    println!(
        "Time it took to run: {} seconds",
        now.elapsed().as_secs_f64()
    );

    println!("Result");
}

fn read_file(file_path: &str) -> String {
    let result = fs::read_to_string(file_path);
    let contents = match result {
        Ok(message) => message,
        Err(error) => match error.kind() {
            io::ErrorKind::NotFound => {
                panic!("File was not found: {error:?}")
            }
            _ => panic!("There was an error reading the file: {error:?}"),
        },
    };

    contents
}
/*
    Differentiating input between the instruction set and the crates and Transforming these into Vector representations of the data.
*/
fn parse_input(input: String) -> (Vec<Vec<String>>, Vec<Instruction>) {
    let (crates_string, instructions_string): (&str, &str) = input.split_once("\r\n\r\n").unwrap();

    let stacks: Vec<Vec<String>> = parse_graph(crates_string);
    let instructions: Vec<Instruction> = parse_instructions(instructions_string);

    (stacks, instructions)
}

fn parse_graph(crates: &str) -> Vec<Vec<String>> {
    let mut crate_lines: Vec<&str> = crates.split("\r\n").collect();

    let stack_line = crate_lines.last().unwrap();

    let stack_numbers: Vec<&str> = stack_line.split("").collect();
    let amount_of_stacks = stack_numbers[stack_numbers.len() - 3]
        .parse::<u8>()
        .unwrap_or(0);

    let mut stacks: Vec<Vec<String>> = vec![vec![]; amount_of_stacks.into()];

    let mut stack_positions: Vec<(u8, usize)> = vec![];

    for number in 1..(amount_of_stacks + 1) {
        let index = stack_numbers
            .iter()
            .position(|&r| r == number.to_string())
            .unwrap_or(0);
        stack_positions.push((number, index));
    }

    crate_lines.pop();
    crate_lines.reverse();

    for (number, index) in stack_positions {
        for line in &crate_lines {
            let line_vec: Vec<&str> = line.split("").collect();
            let mut crate_string = String::new();

            if (line_vec[index] != " ") {
                crate_string.push_str("[");
                crate_string.push_str(line_vec[index]);
                crate_string.push_str("]");
                stacks[(number - 1) as usize].push(crate_string);
            }
        }
    }

    return stacks;
}

fn parse_instructions(instructions: &str) -> Vec<Instruction> {
    let instruction_lines: Vec<&str> = instructions.split("\r\n").collect();

    let mut instructions_parsed: Vec<Instruction> = Vec::new();

    for line in instruction_lines {
        let line_strings: Vec<&str> = line.split(" ").collect();
        let line_instructions: Vec<(String, u16)> = line_strings
            .iter()
            .step_by(2)
            .zip(line_strings.iter().skip(1).step_by(2))
            .map(|(a, b)| (a.to_string(), b.parse::<u16>().unwrap_or(0)))
            .collect::<Vec<(String, u16)>>();

        let mut instruction: Instruction = Instruction {
            Move: 0,
            From: 0,
            To: 0,
        };
        let movement = line_strings
            .iter()
            .position(|x| x.to_string() == String::from("move"))
            .unwrap();
        let from = line_strings
            .iter()
            .position(|x| x.to_string() == String::from("from"))
            .unwrap();
        let to = line_strings
            .iter()
            .position(|x| x.to_string() == String::from("to"))
            .unwrap();

        instruction.Move = line_strings[movement + 1].parse::<u16>().unwrap_or(0);
        instruction.From = line_strings[from + 1].parse::<u16>().unwrap_or(0) - 1;
        instruction.To = line_strings[to + 1].parse::<u16>().unwrap_or(0) - 1;

        instructions_parsed.push(instruction);
    }

    instructions_parsed
}

fn move_single_crates(
    stacks: &Vec<Vec<String>>,
    instructions: &Vec<Instruction>,
) -> Vec<Vec<String>> {
    let mut stacks_cloned = stacks.to_vec();
    for instruction in instructions {
        for i in 0..instruction.Move {
            let value = stacks_cloned[(instruction.From) as usize].pop().unwrap();
            stacks_cloned[(instruction.To) as usize].push(value);
        }
    }
    stacks_cloned
}

fn get_top(stacks: Vec<Vec<String>>) -> Vec<(usize, String)> {
    let mut top_elements: Vec<(usize, String)> = Vec::new();
    for (index, stack) in stacks.iter().enumerate() {
        top_elements.push((index + 1, stack.last().unwrap().to_string()))
    }

    top_elements
}

fn move_multiple_crates(
    stacks: &Vec<Vec<String>>,
    instructions: &Vec<Instruction>,
) -> Vec<Vec<String>> {
    let mut stacks_cloned = stacks.to_vec();
    for instruction in instructions {
        let len = stacks_cloned[instruction.From as usize].len();
        let split_position = len - instruction.Move as usize;
        let mut value: Vec<String> =
            stacks_cloned[(instruction.From) as usize].split_off(split_position);

        stacks_cloned[(instruction.To) as usize].append(&mut value);
    }
    stacks_cloned
}
