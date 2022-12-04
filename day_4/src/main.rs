use std::{fs, io};

fn main() {
    let result = fs::read_to_string("./puzzle/input.txt");
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
}
