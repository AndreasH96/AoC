use std::fs;

const TEST_PATH: &'static str = "./src/test_input.txt";
const REAL_PATH: &'static str = "./src/real_input.txt";

fn add_snafu(s0: Vec<char>, s1: Vec<char>) -> String {
    let largest = if s0.len() > s1.len() {
        s0.clone()
    } else {
        s1.clone()
    };
    let mut snafu: Vec<char> = largest;
    let snafu_len = snafu.len() - 1;
    let mut overflow: Vec<char> = (0..=snafu_len + 1).map(|_| '0').collect();
    let mut overflow_occured = false;
    for (i, (c0, c1)) in s0.iter().rev().zip(s1.iter().rev()).enumerate() {
        let d = *c0 as i32 + *c1 as i32 - (2 * 48);

        if d.abs() < 3 {
            snafu[snafu_len - i] = (d + 48) as u8 as char;
        } else {
            overflow_occured = true;
            if d < 0 {
                snafu[snafu_len - i] = (d + 53) as u8 as char;
                overflow[snafu_len - i] = '/';
            } else {
                snafu[snafu_len - i] = (d + 43) as u8 as char;
                overflow[snafu_len - i] = '1';
            }
        }
    }

    if snafu[0] == '0' {
        snafu.remove(0);
        overflow.remove(0);
    }
    if overflow_occured {
        return add_snafu(snafu, overflow);
    } else {
        return String::from_iter(snafu);
    }
}

fn solve(text_input: String) -> String {
    let snafu = text_input
        .lines()
        .map(|x| x.to_string())
        .reduce(|a, b| add_snafu(a.chars().collect(), b.chars().collect()))
        .unwrap()
        .replace(".", "=")
        .replace("/", "-");
    return snafu;
}

fn main() {
    let test_input = fs::read_to_string(TEST_PATH)
        .expect("Should have been able to read the file")
        .replace('-', "/")
        .replace("=", ".");
    let real_input = fs::read_to_string(REAL_PATH)
        .expect("Should have been able to read the file")
        .replace('-', "/")
        .replace("=", ".");

    println!("SOLUTION");
    use std::time::Instant;
    let now = Instant::now();
    println!("Test: {}", solve(test_input.clone()));
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
    let now = Instant::now();
    println!("Real: {}", solve(real_input.clone()));
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}\n", elapsed);
}
