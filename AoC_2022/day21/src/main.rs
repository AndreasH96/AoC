use std::{collections::HashMap, fs};

const TEST_PATH: &'static str = "./src/test_input.txt";
const REAL_PATH: &'static str = "./src/real_input.txt";

fn get_monkey_val(name: String, monkeys: &HashMap<String, String>) -> i64{
    let monkey: &String = monkeys.get(&name).unwrap();
    let depending_on_human = name == "humn".to_string();
    if monkey.contains(' ') {
        let spl: Vec<&str> = monkey.split(' ').collect();

        let first = get_monkey_val(spl[0].to_string(), monkeys);
        let second = get_monkey_val(spl[2].to_string(), monkeys);

        match spl[1] {
            "+" => return first + second,
            "-" => return first - second,
            "/" => return first / second,
            "*" => return first * second,
            _ => return 0,
        }
    }
    monkey.parse().unwrap()
}

fn part1(input: String) -> i64 {
    let data: HashMap<String, String> = input
        .lines()
        .map(|x| x.split(":").collect())
        .map(|x: Vec<&str>| (x[0].to_string(), x[1].trim().to_string()))
        .collect();
    return get_monkey_val("root".to_string(), &data);
}

fn part2(input: String) -> i64 {
    let mut data: HashMap<String, String> = input
        .lines()
        .map(|x| x.split(":").collect())
        .map(|x: Vec<&str>| (x[0].to_string(), x[1].trim().to_string()))
        .collect();

    let names: Vec<String> = data
        .get("root")
        .unwrap()
        .split(' ')
        .map(|x| x.to_string())
        .collect();
    let me = "humn".to_string();
    let mut val: i64 = data.get(&me).unwrap().parse().unwrap();
    let mut first = get_monkey_val(names[0].clone(), &data);
    let first_init = first.clone();

    data.remove(&me);
    val += 1000;
    data.insert(me.clone(), val.to_string());
    first = get_monkey_val(names[0].clone(), &data);

    let mut second = get_monkey_val(names[2].clone(), &data);
    let mut diff = first - second;
    let sign = if first > first_init { 1 } else { -1 };
    let scaling = 51;
    while first != second {
        diff = first - second;

        if diff < 0 {
            val += (diff / scaling).min(-1) * sign;
        } else if diff > 0 {
            val -= (diff / scaling).max(1) * sign;
        }

        data.remove(&me);
        data.insert(me.clone(), val.to_string());
        first = get_monkey_val(names[0].clone(), &data);
        second = get_monkey_val(names[2].clone(), &data);
    }

    return val;
}

fn main() {
    let test_input = fs::read_to_string(TEST_PATH).expect("Should have been able to read the file");
    let real_input = fs::read_to_string(REAL_PATH).expect("Should have been able to read the file");

    println!("PART 1");
    use std::time::Instant;
    let now = Instant::now();
    println!("Test: {}", part1(test_input.clone()));
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
    let now = Instant::now();
    println!("Real: {}", part1(real_input.clone()));
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}\n", elapsed);
    let now = Instant::now();
    //
    println!("PART 2");
    println!("Test: {:?}", part2(test_input));
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
    let now = Instant::now();
    println!("Real: {}", part2(real_input));
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}
