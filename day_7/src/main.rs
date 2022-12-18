use core::panic;
use std::{env, fs, io, time::Instant, vec};

struct Dir {
    outer: String,
    name: String,
    size: i64,
}
fn main() {
    let now = Instant::now();
    let args: Vec<String> = env::args().collect();
    // let file_path: &str = &args[1];
    let file_path: &str = "./puzzle/input.txt";
    let contents: String = read_file(file_path);
    // do something with the contents
    let parsed_input = parse_input(&contents);
    let result_1 = check_dir_sizes(&parsed_input);

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

fn parse_input(contents: &str) -> Vec<&str> {
    contents.split("\r\n$ ").collect()
}

fn check_dir_sizes(cmd_list: &Vec<&str>) -> i64 {
    println!("{cmd_list:?}");
    let mut dirs: Vec<Dir> = Vec::new();

    let mut current_dir: &Dir = &Dir {
        outer: String::from(""),
        name: String::from("/"),
        size: 0,
    };

    for line in cmd_list {
        match line {
            line if (line.starts_with("cd ")) => {
                let splitted = line.split_once(" ").unwrap();
                if (splitted.1 == "..") {
                    current_dir = dirs.iter().find(|x| x.name == current_dir.outer).unwrap();
                } else {
                    let new_dir = Dir {
                        name: splitted.1.to_string(),
                        outer: current_dir.name.to_string(),
                        size: 0,
                    };
                }
            }
            line if (line.starts_with("ls\r\n")) => {
                let mut children: Vec<&str> = line.split("\r\n").collect();
                children.remove(0);
                for child in children {
                    let info: (&str, &str) = child.split_once(" ").unwrap();
                }
            }
            _ => {
                panic!("Unable to parse the file input");
            }
        }
    }
    6
}
