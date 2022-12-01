use std::fs;
const TEST_PATH :&'static str= "./src/test_input.txt";
const REAL_PATH :&'static str= "./src/real_input.txt";


fn get_elf_calories(path:&str) -> Vec<i64>{
    let contents = fs::read_to_string(path)
    .expect("Should have been able to read the file");
    let data = contents.split("\n\n");
    let summations:Vec<i64> = data.map(|x| x.split("\n").flat_map(|d| d.parse::<i64>().ok()).sum()).collect();
    return summations;
}

fn get_max_calories(summations:Vec<i64>) -> i64 {
    match summations.iter().max() {
        Some(p) => return *p,
        None => return 0
    }
}

fn get_top_three_sum(summations:Vec<i64>) -> i64 {
    let mut sorted_sum = summations.to_vec();
    sorted_sum.sort();
    return sorted_sum[sorted_sum.len()-3 .. sorted_sum.len()].iter().sum();

}


fn main() {
    let test_calories = get_elf_calories(TEST_PATH);
    let real_calories = get_elf_calories(REAL_PATH);

    let task1_test_results = get_max_calories(test_calories.clone());
    let task1_real_results = get_max_calories(real_calories.clone());
    println!("Task1");
    println!("Test results: {:?}",task1_test_results);
    println!("Real results: {:?}",task1_real_results);


    let task2_test_results = get_top_three_sum(test_calories.clone());
    let task2_real_results = get_top_three_sum(real_calories.clone());
    println!("Task2");
    println!("Test results: {:?}",task2_test_results);
    println!("Real results: {:?}",task2_real_results);
}