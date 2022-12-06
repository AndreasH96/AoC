use memory_stats::memory_stats;
use std::{collections::HashSet, fs};

const TEST_PATH: &'static str = "./src/test_input.txt";
const REAL_PATH: &'static str = "./src/real_input.txt";

fn part1(path: &str) -> usize {
    let binding = fs::read_to_string(path).expect("Should have been able to read the file");

    let contents = binding.split("\n").map(|r| {
        r.split(',')
            .map(|x| x.split('-'))
            .flatten()
            .collect::<Vec<&str>>()
    });

    let ranges: Vec<Vec<HashSet<i32>>> = contents
        .map(|c: Vec<&str>| {
            vec![
                HashSet::from_iter(c[0].parse::<i32>().unwrap()..c[1].parse::<i32>().unwrap() + 1),
                HashSet::from_iter(c[2].parse::<i32>().unwrap()..c[3].parse::<i32>().unwrap() + 1),
            ]
        })
        .collect();

    let overlaps: Vec<&Vec<HashSet<i32>>> = ranges
        .iter()
        .filter(|x: &&Vec<HashSet<i32>>| x[0].is_subset(&x[1]) || x[1].is_subset(&x[0]))
        .collect();

    return overlaps.len();
}

fn part2(path: &str) -> usize {
    let binding = fs::read_to_string(path).expect("Should have been able to read the file");

    let contents = binding.split("\n").map(|r| {
        r.split(',')
            .map(|x| x.split('-'))
            .flatten()
            .collect::<Vec<&str>>()
    });

    let ranges: Vec<Vec<HashSet<i32>>> = contents
        .map(|c: Vec<&str>| {
            vec![
                HashSet::from_iter(c[0].parse::<i32>().unwrap()..c[1].parse::<i32>().unwrap() + 1),
                HashSet::from_iter(c[2].parse::<i32>().unwrap()..c[3].parse::<i32>().unwrap() + 1),
            ]
        })
        .collect();

    let overlaps: Vec<&Vec<HashSet<i32>>> = ranges
        .iter()
        .filter(|x: &&Vec<HashSet<i32>>| {
            x[0].intersection(&x[1]).collect::<HashSet<&i32>>().len() > 0
        })
        .collect();
        let mut te = vec![5;100000];
        te.sort();
    return overlaps.len();
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
