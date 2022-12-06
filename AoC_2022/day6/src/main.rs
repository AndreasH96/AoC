use std::collections::HashSet;
use std::fs;

const REAL_PATH: &'static str = "./src/real_input.txt";

fn solve(raw_signal: &String, window_size: usize) -> usize {
    let data: Vec<char> = raw_signal.chars().collect();
    for (i, window) in data.windows(window_size).enumerate() {
        let set: HashSet<&char> = HashSet::from_iter(window);
        if set.len() == window_size {
            return i + window_size;
        }
    }
    return 0;
}

fn main() {
    let part1_test:String = "mjqjpqmgbljsphdztnvjfqwrcgsmlb".to_owned();// 7
    let real_data = fs::read_to_string(REAL_PATH).expect("Should have been able to read the file");

    println!("PART 1");
    let test_result = solve(&part1_test,4);
    println!("Test: {}", test_result);
    let real_result = solve(&real_data,4);
    println!("Real: {}", real_result);

    println!("PART 2");
    let test_result = solve(&part1_test,14);
    println!("Test: {}", test_result);
    let real_result = solve(&real_data,14);
    println!("Real: {}", real_result);
}
