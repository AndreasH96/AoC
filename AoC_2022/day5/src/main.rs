use std::str;
const TEST_PATH: &'static str = "./src/test_input.txt";
const REAL_PATH: &'static str = "./src/real_input.txt";

fn preprocess_init_state(raw_state: &str) -> Vec<Vec<&str>> {
    let mut raw_initial_state: Vec<Vec<&str>> = raw_state
        .split("\n")
        .map(|x| {
            x.as_bytes()
                .chunks(4)
                .map(|c| str::from_utf8(c).unwrap().trim())
                .collect::<Vec<&str>>()
        })
        .collect();

    raw_initial_state.pop();
    let mut initial_state: Vec<Vec<&str>> = vec![vec![]; raw_initial_state[0].len()];

    raw_initial_state.iter().for_each(|row| {
        row.iter().enumerate().for_each(|x| {
            if *x.1 != "" {
                initial_state[x.0].insert(0, *x.1);
            }
        })
    });
    return initial_state;
}
fn preprocess_instructions(raw_instr: &str) -> Vec<Vec<usize>> {
    let instructions: Vec<Vec<usize>> = raw_instr
        .split("\n")
        .map(|r| {
            r.split(" ")
                .map(|x| x.parse().unwrap_or_default())
                .filter(|y| *y > 0)
                .collect()
        })
        .collect();
    return instructions;
}

fn part1(path: &str) -> String{
    let binding =
        std::fs::read_to_string(path).expect("Should have been able to read the file");
    let binding: Vec<&str> = binding.split("\n\n").collect();
    let mut initial_state = preprocess_init_state(binding[0].clone());

    let instructions: Vec<Vec<usize>> = preprocess_instructions(binding[1]);

    for instruction in &instructions {
        let amount = instruction[0];
        let from = instruction[1] - 1;
        let to = instruction[2] - 1;
        for _ in (0..amount) {
            let p = initial_state[from].pop().unwrap();
            let i = initial_state[to].len();
            initial_state[to].insert(i, p);
        }
    }
    let result: Vec<String> = initial_state.iter().map(|x| x.last().unwrap().replace("[", "").replace("]", "")).collect();
    
    return result.join("");
}

fn part2(path: &str) -> String{
    let binding =
        std::fs::read_to_string(path).expect("Should have been able to read the file");
    let binding: Vec<&str> = binding.split("\n\n").collect();
    let mut initial_state = preprocess_init_state(binding[0].clone());

    let instructions: Vec<Vec<usize>> = preprocess_instructions(binding[1]);

    for instruction in &instructions {
        let amount = instruction[0];
        let from = instruction[1] - 1;
        let to = instruction[2] - 1;
        let mut to_be_moved:Vec<&str> = vec![];
        for _ in (0..amount) {
            let p = initial_state[from].pop().unwrap();
            to_be_moved.insert(0, p);
        }
        initial_state[to].append(&mut to_be_moved);
    }
    let result: Vec<String> = initial_state.iter().map(|x| x.last().unwrap().replace("[", "").replace("]", "")).collect();
    
    return result.join("");
}



fn main() {
    println!("PART 1");
    let test_result = part1(TEST_PATH);
    println!("Test: {:?}", test_result);
    let real_result = part1(REAL_PATH);
    println!("Real: {:?}", real_result);

    println!("PART 2");
    let test_result = part2(TEST_PATH);
    println!("Test: {}", test_result);
    let real_result = part2(REAL_PATH);
    println!("Real: {}", real_result);
}
