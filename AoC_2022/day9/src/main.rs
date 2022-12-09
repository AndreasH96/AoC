use ndarray::{arr1, Array1};
use std::collections::{HashMap, HashSet};
use std::fmt::Display;
use std::fs;
use std::ops::Neg;

const TEST_PATH: &'static str = "./src/test_input.txt";
const TEST_PATH2: &'static str = "./src/test_input_2.txt";
const REAL_PATH: &'static str = "./src/real_input.txt";

fn part1(raw_data: &str) -> usize {
    let steps: Vec<(&str, i32)> = raw_data
        .split("\n")
        .map(|x| x.split(" ").collect())
        .map(|x: Vec<&str>| (x[0], x[1].parse::<i32>().unwrap()))
        .collect();
    let mut tail_visits: HashSet<String> = HashSet::from(["0,0".to_string()]);
    let mut current_head_pos = arr1(&[0, 0]);
    let mut current_tail_pos = arr1(&[0, 0]);

    let position_deltas = HashMap::from([
        ("R", (0, 1)),
        ("L", (0, 1.neg())),
        ("D", (1, 1.neg())),
        ("U", (1, 1)),
    ]);

    for step in steps {
        let action = position_deltas[&step.0];
        current_head_pos[action.0] += action.1 * step.1;

        let mut distance: Array1<i32> = &current_head_pos - &current_tail_pos;
        while distance[0].abs() > 1 || distance[1].abs() > 1 {
            if distance[0].abs() > 1 || distance[1].abs() > 1 {
                current_tail_pos[0] += if distance[0] != 0 {
                    distance[0] / distance[0].abs()
                } else {
                    0
                };
                current_tail_pos[1] += if distance[1] != 0 {
                    distance[1] / distance[1].abs()
                } else {
                    0
                };

                distance = &current_head_pos - &current_tail_pos;

                tail_visits.insert(format!(
                    "{:?},{:?}",
                    current_tail_pos[0], current_tail_pos[1]
                ));
            }
        }
    }
    return tail_visits.len();
}

fn part2(raw_data: &str) -> usize {
    let steps: Vec<(&str, i32)> = raw_data
        .split("\n")
        .map(|x| x.split(" ").collect())
        .map(|x: Vec<&str>| (x[0], x[1].parse::<i32>().unwrap()))
        .collect();
    let mut knot_positions: Vec<Array1<i32>> = vec![arr1(&[0, 0]); 10];
    let mut tail_visits: HashSet<String> = HashSet::from(["0,0".to_string()]);
    let position_deltas = HashMap::from([
        ("R", (0, 1)),
        ("L", (0, 1.neg())),
        ("D", (1, 1.neg())),
        ("U", (1, 1)),
    ]);

    for step in steps {
        let action = position_deltas[&step.0];
        for _ in 0..step.1 {
            knot_positions[0][action.0] += action.1;
            for i in 1..knot_positions.len() {
                let mut distance: Array1<i32> = &knot_positions[i - 1] - &knot_positions[i];
                if distance[0].abs() > 1 || distance[1].abs() > 1 {
                    knot_positions[i][0] += if distance[0] != 0 {
                        distance[0] / distance[0].abs()
                    } else {
                        0
                    };
                    knot_positions[i][1] += if distance[1] != 0 {
                        distance[1] / distance[1].abs()
                    } else {
                        0
                    };

                    distance = &knot_positions[i - 1] - &knot_positions[i];

                    if i == 9 {
                        tail_visits.insert(format!(
                            "{:?},{:?}",
                            knot_positions[i][0], knot_positions[i][1]
                        ));
                    }
                }
            }
        }
    }

    return tail_visits.len();
}

fn main() {
    let test_data = fs::read_to_string(TEST_PATH).expect("Should have been able to read the file");
    let test_data2 =
        fs::read_to_string(TEST_PATH2).expect("Should have been able to read the file");
    let real_data = fs::read_to_string(REAL_PATH).expect("Should have been able to read the file");

    println!("PART 1");
    println!("Test: {}", part1(&test_data));
    println!("Real: {}", part1(&real_data));

    println!("PART 2");
    println!("Test: {}", part2(&test_data2));
    println!("Real: {}", part2(&real_data));
}
