use std::{collections::HashSet, env, fs, hash::Hash, io, thread::current, time::Instant, vec};

fn main() {
    let now = Instant::now();
    // let args: Vec<String> = env::args().collect();
    // let file_path: &str = &args[1];
    let file_path: &str = "./puzzle/input.txt";
    let contents: String = read_file(file_path);
    // do something with the contents
    let result_1 = find_marker(&contents, 4);
    let result_2 = find_marker(&contents, 14);
    println!(
        "Time it took to run: {} seconds",
        now.elapsed().as_secs_f64()
    );

    println!("Result 1: {}, Result 2: {}", result_1.0, result_2.0);
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

fn find_marker(contents: &str, message_length: usize) -> (usize, Vec<char>) {
    let char_list: Vec<char> = contents.chars().collect();
    let mut chars: Vec<char> = Vec::with_capacity(message_length);
    let mut current_index: usize = 0;
    for (index, char) in char_list.iter().enumerate() {
        if chars.len() == message_length && has_unique_elements(&chars) {
            break;
        }
        current_index = index;

        if chars.len() < message_length {
            chars.push(*char);
            continue;
        }

        if chars.contains(char) {
            let position = chars.iter().position(|x| x == char).unwrap();
            chars.drain(..position);
        } else {
            chars.remove(0);
        }
        chars.push(*char);
    }

    (current_index + 1, chars)
}

fn has_unique_elements<T>(iter: T) -> bool
where
    T: IntoIterator,
    T::Item: Eq + Hash,
{
    let mut uniq = HashSet::new();
    iter.into_iter().all(move |x| uniq.insert(x))
}
