use std::{time::Instant, env, fs, io};

fn main() {
    let now = Instant::now();
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
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

    println!(
        "Time it took to run: {} seconds",
        now.elapsed().as_secs_f64()
    );
    
    println!("Result");
}
