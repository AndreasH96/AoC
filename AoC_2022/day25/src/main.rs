use std::{fs};

const TEST_PATH: &'static str = "./src/test_input.txt";
const REAL_PATH: &'static str = "./src/real_input.txt";

fn add_snafu(s0: String, s1: String) -> String {
    let largest = if s0.len() > s1.len() {
        s0.clone()
    } else {
        s1.clone()
    };
    let mut snafu: Vec<char> = largest.chars().collect();
    let snafu_len = snafu.len() - 1;
    let mut overflow: Vec<char> = (0..=snafu_len + 1).map(|_| '0').collect();
    let mut overflow_occured = false;
    for (i, (c0, c1)) in s0.chars().rev().zip(s1.chars().rev()).enumerate() {
        let d = c0 as i32 + c1 as i32 - (2 * 48);

        if d.abs() < 3 {
            snafu[snafu_len - i] = format!("{}", (d + 48) as u8 as char)
                .chars()
                .next()
                .unwrap();
        } else {
            overflow_occured = true;
            if d < 0 {
                snafu[snafu_len - i] = format!("{}", (d + 53) as u8 as char)
                    .chars()
                    .next()
                    .unwrap();
                overflow[snafu_len - i] = '/';
            } else {
                snafu[snafu_len - i] = format!("{}", (d + 43) as u8 as char)
                    .chars()
                    .next()
                    .unwrap();
                overflow[snafu_len - i] = '1';
            }
        }
    }

    if overflow_occured {
        return add_snafu(
            String::from_iter(snafu),
            String::from_iter(overflow.clone()),
        );
    } else {
        return String::from_iter(snafu);
    }
}

fn solve(text_input: String) -> String {
    let mut snafu = "0".to_string();
    for row in text_input.lines() {
        snafu = add_snafu(snafu, row.to_string());
        let first_val_pos = snafu.clone().chars().position(|x| x != '0').unwrap();
        let chs: Vec<String> = snafu.chars().map(|x| x.to_string()).collect();
        snafu = chs[first_val_pos..].join("");
    }
    snafu = snafu.replace(".", "=").replace("/", "-");

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
