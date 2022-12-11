use std::{
    collections::{HashMap, LinkedList},
    fs,
};

use day11::Monkey;
use num::integer::lcm;
const TEST_PATH: &'static str = "./src/test_input.txt";
const REAL_PATH: &'static str = "./src/real_input.txt";

fn extract_number(inp_str: &str) -> usize {
    return (inp_str.split(" ").collect::<Vec<&str>>())
        .last()
        .unwrap()
        .parse()
        .unwrap();
}

fn parse_monkey(id: usize, monkey: &Vec<&str>) -> Monkey {
    let items: LinkedList<usize> = monkey[1][17..]
        .split(",")
        .map(|x| x.trim().parse::<usize>().unwrap())
        .collect();
    let operation: Vec<String> = monkey[2][22..]
        .split_whitespace()
        .map(|x| x.to_string())
        .collect();
    let test = extract_number(monkey[3]);
    let if_true = extract_number(monkey[4]);
    let if_false = extract_number(monkey[5]);
    let new_monkey = Monkey::new(
        id,
        items,
        (operation[0].clone(), operation[1].clone()),
        test,
        if_true,
        if_false,
        0,
    );
    return new_monkey;
}


fn solve(raw_data: &str, rounds: i32, divide: bool) -> u128 {
    let data: Vec<Vec<&str>> = raw_data
        .split("\n\n")
        .map(|x| x.lines().collect())
        .collect();

    let mut monkeys: HashMap<usize, Monkey> = HashMap::new();
    for (id, monkey) in data.iter().enumerate() {
        monkeys.insert(id, parse_monkey(id, monkey));
    }

    // This solves the handling of large numbers
    let modulo = monkeys.values().map(|x| x.test).reduce(|a,b| lcm(a,b)).unwrap();
   

    for _ in 0..rounds {
        for id in 0..monkeys.len() {
            let monkey = monkeys.get_mut(&id).unwrap();
            let throws = monkey.inspect(divide);
            for throw in throws {
               monkeys.get_mut(&throw.0).unwrap().add_item(throw.1%modulo);
            }
        }
    }
    let mut inspections: Vec<u128> = monkeys.values().map(|x| x.inspections as u128).collect();
    inspections.sort_by(|a, b| b.partial_cmp(a).unwrap());
    let monkey_business = inspections[0] * inspections[1];
    return monkey_business;
}

fn main() {
    let test_data = fs::read_to_string(TEST_PATH).expect("Should have been able to read the file");
    let real_data = fs::read_to_string(REAL_PATH).expect("Should have been able to read the file");

    println!("PART 1");

    println!("Test: {}", solve(&test_data, 20, true));
    println!("Real: {}", solve(&real_data, 20, true));

    println!("\nPART 2");
    println!("Test: {}", solve(&test_data, 10000, false));
    println!("Real: {}", solve(&real_data, 10000, false));
}
