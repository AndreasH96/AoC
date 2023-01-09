use num::Complex;
use std::{collections::HashMap, fs};

const TEST_PATH: &'static str = "./src/test_input.txt";
const REAL_PATH: &'static str = "./src/real_input.txt";

fn make_move(elves: &mut HashMap<Complex<i32>, Vec<u8>>) -> usize {
    let deltas = vec![
        Complex::new(0, -1),
        Complex::new(0, 1),
        Complex::new(-1, 0),
        Complex::new(1, 0),
    ];

    let neighbors = vec![
        vec![
            Complex::new(-1, -1),
            Complex::new(0, -1),
            Complex::new(1, -1),
        ], // n
        vec![Complex::new(-1, 1), Complex::new(0, 1), Complex::new(1, 1)], // s
        vec![
            Complex::new(-1, 1),
            Complex::new(-1, 0),
            Complex::new(-1, -1),
        ], // w
        vec![Complex::new(1, -1), Complex::new(1, 0), Complex::new(1, 1)], // e
    ];
    let mut propositions: HashMap<Complex<i32>, Complex<i32>> = HashMap::new();

    for (k, v) in elves.clone() {
        let possible: Vec<usize> = neighbors
            .iter()
            .enumerate()
            .filter(|d| !d.1.iter().any(|p| elves.contains_key(&(p + k))))
            .map(|d| d.0)
            .collect();

        let mut new_v = v.clone();

        if possible.len() < 4 && possible.len() > 0 {
            for i in v {
                if possible.contains(&(i as usize)) {
                    let proposition = k + deltas[i as usize];

                    if propositions.contains_key(&proposition) {
                        propositions.remove(&proposition);
                    } else {
                        propositions.insert(proposition, k);
                    }
                    break;
                }
            }
        } else {
        }
        new_v.rotate_left(1);
        elves.insert(k, new_v);
    }
    let changes = propositions.len();
    for prop in &propositions {
        let p = elves.get(&prop.1).unwrap().clone();
        elves.remove(prop.1);
        elves.insert(*prop.0, p);
    }
    return changes;
}

fn solve(text_input: String, part2: bool) -> i32 {
    let mut elves: HashMap<Complex<i32>, Vec<u8>> = HashMap::new();

    let prios = vec![0, 1, 2, 3];
    for (y, line) in text_input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                elves.insert(Complex::new(x as i32, y as i32), prios.clone());
            }
        }
    }
    if part2 {
        let mut counter = 1;
        loop {
            if make_move(&mut elves) == 0 {
                return counter;
            }
            counter += 1;
        }
    }
    for _ in 0..10 {
        _ = make_move(&mut elves);
    }
    let max_x = elves.keys().map(|e| e.re).max().unwrap();
    let min_x = elves.keys().map(|e| e.re).min().unwrap();
    let max_y = elves.keys().map(|e| e.im).max().unwrap();
    let min_y = elves.keys().map(|e| e.im).min().unwrap();

    return ((max_x - min_x).abs() + 1) * ((max_y - min_y).abs() + 1) - elves.len() as i32;
}

fn main() {
    let test_input = fs::read_to_string(TEST_PATH).expect("Should have been able to read the file");
    let real_input = fs::read_to_string(REAL_PATH).expect("Should have been able to read the file");

    println!("PART 1");
    use std::time::Instant;
    let now = Instant::now();
    println!("Test: {}", solve(test_input.clone(), false));
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
    let now = Instant::now();
    println!("Real: {}", solve(real_input.clone(), false));
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}\n", elapsed);
    let now = Instant::now();

    println!("PART 2");
    println!("Test: {:?}", solve(test_input, true));
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
    let now = Instant::now();
    println!("Real: {}", solve(real_input, true));
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}
