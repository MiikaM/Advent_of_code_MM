use core::panic;
use std::{cmp, collections::HashMap, fs, io, ops::Add, time::Instant, vec};

fn main() {
    let now = Instant::now();
    // let args: Vec<String> = env::args().collect();
    // let file_path: &str = &args[1];
    let file_path: &str = "./puzzle/input.txt";
    let contents: String = read_file(file_path);
    // do something with the contents
    let parsed_input = parse_input(&contents);
    let result_1 = check_dir_sizes(&parsed_input);
    let result_2 = choose_correct_dir_size(&parsed_input);

    println!(
        "Time it took to run: {} seconds",
        now.elapsed().as_secs_f64()
    );

    println!("Result_1: {result_1}, Result_2: {result_2}");
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
    let mut path: Vec<&str> = Vec::new();

    let mut sizes: HashMap<String, i64> = HashMap::new();
    let mut children: HashMap<String, Vec<String>> = HashMap::new();

    for block in cmd_list {
        let mut lines: Vec<&str> = block.split("\r\n").collect();
        let command: Vec<&str> = lines.drain(..1).collect();

        let parts: Vec<&str> = command[0].split(" ").collect();
        let operation = parts[0];

        if operation == "cd" {
            if parts[1] == ".." {
                path.pop();
            } else {
                path.push(parts[1])
            }
            continue;
        }

        assert!(operation == "ls");

        let joined_path = path.join("/");
        let mut size: i64 = 0;

        for child in lines {
            if !child.starts_with("dir") {
                size = size.add(
                    child
                        .split(" ")
                        .collect::<Vec<&str>>()
                        .first()
                        .expect("Jaaa")
                        .parse::<i64>()
                        .unwrap_or(0),
                );
            } else {
                let dir = &child.split(" ").collect::<Vec<&str>>();
                let dir_name = dir.last().unwrap();
                let mut s = String::from(&joined_path.to_string());
                s.push_str("/");
                s.push_str(dir_name);
                children
                    .entry(joined_path.clone())
                    .and_modify(|x| x.push(s.clone()))
                    .or_insert(vec![s]);
            }
        }
        sizes
            .entry(joined_path.clone())
            .and_modify(|x| *x += size)
            .or_insert(size);
    }

    let mut result: i64 = 0;
    for path_var in sizes.keys() {
        let size_closure = dfs(path_var, &sizes, &children);
        if size_closure <= 100000 {
            result += size_closure;
        }
    }

    result
}

fn dfs(var: &String, sizes: &HashMap<String, i64>, children: &HashMap<String, Vec<String>>) -> i64 {
    let mut size: i64 = sizes.get(var).unwrap().to_owned();
    let element = children.get(var);
    match element {
        Some(element) => {
            for child in element {
                size = size.add(dfs(child, sizes, children));
            }
        }
        None => {
            println!("No sub directories with directory: {var}")
        }
    }

    size
}

fn choose_correct_dir_size(cmd_list: &Vec<&str>) -> i64 {
    let mut path: Vec<&str> = Vec::new();

    let mut sizes: HashMap<String, i64> = HashMap::new();
    let mut children: HashMap<String, Vec<String>> = HashMap::new();

    for block in cmd_list {
        let mut lines: Vec<&str> = block.split("\r\n").collect();
        let command: Vec<&str> = lines.drain(..1).collect();

        let parts: Vec<&str> = command[0].split(" ").collect();
        let operation = parts[0];

        if operation == "cd" {
            if parts[1] == ".." {
                path.pop();
            } else {
                path.push(parts[1])
            }
            continue;
        }

        assert!(operation == "ls");

        let joined_path = path.join("/");
        let mut size: i64 = 0;

        for child in lines {
            if !child.starts_with("dir") {
                size = size.add(
                    child
                        .split(" ")
                        .collect::<Vec<&str>>()
                        .first()
                        .expect("Jaaa")
                        .parse::<i64>()
                        .unwrap_or(0),
                );
            } else {
                let dir = &child.split(" ").collect::<Vec<&str>>();
                let dir_name = dir.last().unwrap();
                let mut s = String::from(&joined_path.to_string());
                s.push_str("/");
                s.push_str(dir_name);
                children
                    .entry(joined_path.clone())
                    .and_modify(|x| x.push(s.clone()))
                    .or_insert(vec![s]);
            }
        }
        sizes
            .entry(joined_path.clone())
            .and_modify(|x| *x += size)
            .or_insert(size);
    }

    let unused_space = 70000000 - dfs(&"/".to_string(), &sizes, &children);
    let required_space = 30000000 - unused_space;

    let mut ans: i64 = 1 << 60;

    for path_var in sizes.keys() {
        let size_closure = dfs(path_var, &sizes, &children);
        if size_closure >= required_space {
            ans = cmp::min(ans, size_closure);
        }
    }

    ans
}
