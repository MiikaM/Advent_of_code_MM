use std::{env, fs, io, time::Instant, vec};

fn main() {
    let now = Instant::now();
    let _args: Vec<String> = env::args().collect();
    // let file_path = &args[1];
    let file_path = "./puzzle/input.txt";
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
    // do something with the contents
    let parsed_input = parse_input(&contents);
    println!(
        "Time it took to run: {} seconds",
        now.elapsed().as_secs_f64()
    );

    println!("Result");
}

fn parse_input<'a>(input: &'a String) -> (Vec<&'a str>, Vec<&'a str>) {
    let (crates, instructions): (&str, &str) = input.split_once("\r\n\r\n").unwrap();

    let crate_lines: Vec<&str> = crates.split("\r\n").collect();
    let instruction_lines: Vec<&str> = instructions.split("\r\n").collect();

    for line in &crate_lines {
        let stack_numbers: Vec<&str> = line.split("").collect();
        println!("Length of line: {} and the items: {:?}", stack_numbers.len(), stack_numbers);

        
    }

    println!("{}", crate_lines.last().unwrap().len());

    let stack_line = crate_lines.last().unwrap();

    let stack_numbers: Vec<&str> = stack_line.split(" ").collect();
    let amount_of_stacks = stack_numbers[stack_numbers.len() - 2].parse::<u8>().unwrap_or(0);
    let mut stacks:Vec<Vec<&str>> = vec![vec![]; amount_of_stacks.into()];

    
    stacks[0].push("test");
    

    println!("{stacks:?}" );

    (instruction_lines, crate_lines)
}

fn move_crates() -> () {
    println!("Move crates")
}
