use std::{collections::HashMap, fs};

const TEST_PATH: &'static str = "./src/test_input.txt";
const REAL_PATH: &'static str = "./src/real_input.txt";


fn part1(data:Vec<i32>) -> i32 {
    let initial_state = data.clone();
    let mut state = data;


    return 0
}


fn main() {
    let test_input = fs::read_to_string(TEST_PATH).expect("Should have been able to read the file");
    let real_input = fs::read_to_string(REAL_PATH).expect("Should have been able to read the file");

    let data:Vec<i32> = test_input.lines().map(|x| x.parse().unwrap()).collect();
    


    //println!("PART 1");
    //use std::time::Instant;
    //let now = Instant::now();
    //println!("Test: {}", part1(test_input.clone()));
    //let elapsed = now.elapsed();
    //println!("Elapsed: {:.2?}", elapsed);
    //let now = Instant::now();
    //println!("Real: {}", part1(real_input.clone()));
    //let elapsed = now.elapsed();
    //println!("Elapsed: {:.2?}\n", elapsed);
    //let now = Instant::now();
//
    //println!("PART 2");
    //println!("Test: {:?}", part2(test_input, true));
    //let elapsed = now.elapsed();
    //println!("Elapsed: {:.2?}", elapsed);
    //let now = Instant::now();
    //println!("Real: {}", part2(real_input, false));
    //let elapsed = now.elapsed();
    //println!("Elapsed: {:.2?}", elapsed);
}
