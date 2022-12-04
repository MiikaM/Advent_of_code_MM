use std::{env, fs, io};

fn main() {
    let args: Vec<String> = env::args().collect();
    // let file_path = &args[1];
    let file_path = String::from("./puzzle/input.txt");
    let result = fs::read_to_string(file_path);
    let contents = match result {
        Ok(message) => message,
        Err(error) => match error.kind() {
            io::ErrorKind::NotFound => {
                println!("File was not found: {error:?}");
                String::from("File was not found")
            }
            _ => panic!("There was an error reading the file: {error:?}"),
        },
    };

    let list_of_pairs: Vec<&str> = contents.split("\r\n").collect();

    println!("{list_of_pairs:?}")
}
