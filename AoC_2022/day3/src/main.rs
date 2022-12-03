use itertools::Itertools;
use std::{collections::HashSet, fs};
const TEST_PATH: &'static str = "./src/test_input.txt";
const REAL_PATH: &'static str = "./src/real_input.txt";
const ALPHABET: &str = " abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

fn part1(path: &str) -> i32 {
    let contents = fs::read_to_string(path).expect("Should have been able to read the file");
    let rows = contents.split("\n").map(|r| r.split_at(r.len() / 2));
    let hash_set_rows = rows.map(|r| {
        r.0.chars()
            .collect::<HashSet<char>>()
            .intersection(&r.1.chars().collect::<HashSet<char>>())
            .collect::<String>()
    });
    let result: i32 = hash_set_rows
        .map(|r| ALPHABET.find(r.chars().next().unwrap()).unwrap() as i32)
        .sum();
    return result;
}

fn part2(path: &str) -> i32 {
    let contents = fs::read_to_string(path).expect("Should have been able to read the file");
    let groups: Vec<Vec<&str>> = contents
        .split("\n")
        .chunks(3)
        .into_iter()
        .map(|x| x.collect_vec())
        .collect();

    let mut group_codes: Vec<char> = Vec::new();

    for group in groups {
        // Make sure to remove row-duplicated chars
        let unique_chars: Vec<String> = group
            .iter()
            .map(|r| itertools::join(&r.chars().collect::<HashSet<char>>(), ""))
            .collect();
        // Make a concatenated string for the entire group
        // In order to count occurences of chars in it
        let concatenated = unique_chars.join("");
        let characters = concatenated.chars().collect::<HashSet<char>>();
        for character in characters {
            let count: usize = concatenated.matches(character).count();
            if count == 3 {
                group_codes.push(character);
            }
        }
    }
    let result: i32 = group_codes
        .iter()
        .map(|r| ALPHABET.find(*r).unwrap() as i32)
        .sum();
    return result;
}
fn main() {
    println!("PART 1");
    let test_result = part1(TEST_PATH);
    println!("Test: {}", test_result);
    let real_result = part1(REAL_PATH);
    println!("Real: {}", real_result);

    println!("PART 2");
    let test_result = part2(TEST_PATH);
    println!("Test: {}", test_result);
    let real_result = part2(REAL_PATH);
    println!("Real: {}", real_result);
}
