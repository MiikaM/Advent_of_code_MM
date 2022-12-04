use std::{env, ffi::c_long, fs, io};

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    // let file_path = String::from("./puzzle/input.txt");
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

    let parsed_data = parse_input(&contents);

    let result = find_contained_pairs(&parsed_data);

    println!("{result:?}");
}

fn parse_input(input_string: &String) -> Vec<((u16, u16), (u16, u16))> {
    let list_of_pairs: Vec<&str> = input_string.split("\r\n").collect();
    let mut parsed_vec: Vec<((u16, u16), (u16, u16))> = vec![];

    for pair in list_of_pairs {
        let (first, second) = pair.split_once(",").unwrap();
        let (first_start, first_end) = first.split_once("-").unwrap();
        let (second_start, second_end) = second.split_once("-").unwrap();
        let tuple_numbers: ((u16, u16), (u16, u16)) = (
            (
                first_start.parse::<u16>().unwrap(),
                first_end.parse::<u16>().unwrap(),
            ),
            (
                second_start.parse::<u16>().unwrap(),
                second_end.parse::<u16>().unwrap(),
            ),
        );
        parsed_vec.push(tuple_numbers);
    }

    parsed_vec
}

fn find_contained_pairs(list_of_pairs: &Vec<((u16, u16), (u16, u16))>) -> u16 {
    let mut result: u16 = 0;

    for pair in list_of_pairs {
        let first_start: u16 = pair.0 .0;
        let first_end: u16 = pair.0 .1;
        let second_start: u16 = pair.1 .0;
        let second_end: u16 = pair.1 .1;
        let first_range = first_start..(first_end + 1);
        let second_range = second_start..(second_end + 1);

        if ((first_range.contains(&second_start) && first_range.contains(&second_end))
            || (second_range.contains(&first_start) && second_range.contains(&first_end)))
        {
            result += 1;
        }
    }

    result
}
