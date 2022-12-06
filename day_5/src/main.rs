use std::{
    env,
    fs::{self, create_dir},
    io,
    time::Instant,
    vec,
};

fn main() {
    let now = Instant::now();
    let args: Vec<String> = env::args().collect();
    // let file_path: &str = &args[1];
    let file_path: &str = "./puzzle/input.txt";
    let contents: String = read_file(file_path);
    // do something with the contents
    let parsed_input: (Vec<String>, Vec<String>) = parse_input(contents);
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
fn parse_input(input: String) -> (Vec<String>, Vec<String>) {
    let (crates, instructions): (&str, &str) = input.split_once("\r\n\r\n").unwrap();

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
    let instruction_lines: Vec<&str> = instructions.to_owned().split("\r\n").collect();

    println!(
        " and length of stacks: , amount of stacks: , stacks: {stacks:?}",
        // stack_numbers.len(),
        // amount_of_stacks
    );
    (Vec::new(), Vec::new())
}

fn move_crates() -> () {
    println!("Move crates")
}
