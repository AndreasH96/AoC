use serde_json::Value;
use std::fs;
const TEST_PATH: &'static str = "./src/test_input.txt";
const REAL_PATH: &'static str = "./src/real_input.txt";

fn compare_values(left: &Value, right: &Value) -> i64 {

    if left.is_number() && right.is_number() {
        return left.as_i64().unwrap() - right.as_i64().unwrap();
    } else if left.is_array() && right.is_array() {
        let l_arr = left.as_array().unwrap();
        let r_arr = right.as_array().unwrap();
        // Ugly AF if-statements
        if l_arr.len() == 0 && r_arr.len() == 0 {
            return 0;
        }
        if l_arr.len() == 0 {
            return -1;
        }
        if r_arr.len() == 0 {
            return 1;
        }
        let rec = compare_values(&l_arr[0], &r_arr[0]);
        let ret = if rec != 0 {
            rec
        } else {
            compare_values(
                &Value::Array(l_arr[1..].to_vec()),
                &Value::Array(r_arr[1..].to_vec()),
            )
        };
        return ret;
    }

    if left.is_number() {
        return compare_values(&Value::Array(vec![left.clone()]), right);
    } else {
        return compare_values(left, &Value::Array(vec![right.clone()]));
    }
}

fn part1(groups_raw: Vec<Vec<Value>>) -> usize {
    let mut sum = 0;
    for (i, group) in groups_raw.iter().enumerate() {
        let mut r = true;
        let res = compare_values(group.get(0).unwrap(), group.get(1).unwrap());
        if res < 0 {
            sum += i + 1;
        }
    }
    return sum;
}

fn part2(groups_raw: Vec<Vec<Value>>) -> usize {
    let mut flat: Vec<&Value> = groups_raw.iter().flatten().collect();
    let new_1: Value = serde_json::from_str("[[2]]").unwrap();
    let new_2: Value = serde_json::from_str("[[6]]").unwrap();
    flat.insert(0, &new_1);
    flat.insert(0, &new_2);
    flat.sort_by(|a, b| compare_values(a, b).cmp(&0));
    let res = (flat.iter().position(|x| **x == new_1).unwrap() + 1)
        * (flat.iter().position(|x| **x == new_2).unwrap() + 1);
    return res;
}

fn main() {
    let test_input = fs::read_to_string(TEST_PATH).expect("Should have been able to read the file");
    let real_input = fs::read_to_string(REAL_PATH).expect("Should have been able to read the file");

    let test_groups: Vec<Vec<Value>> = test_input
        .split("\n\n")
        .map(|x| {
            x.split("\n")
                .map(|y| serde_json::from_str(y).unwrap())
                .collect()
        })
        .collect();

    let real_groups: Vec<Vec<Value>> = real_input
        .split("\n\n")
        .map(|x| {
            x.split("\n")
                .map(|y| serde_json::from_str(y).unwrap())
                .collect()
        })
        .collect();

    println!("PART 1");
    println!("Test: {}", part1(test_groups.clone()));
    println!("Real: {}\n", part1(real_groups.clone()));

    println!("PART 2");
    println!("Test: {}", part2(test_groups));
    println!("Real: {}", part2(real_groups));
}
