use std::{fs, iter::Product};
const TEST_PATH: &'static str = "./src/test_input.txt";
const REAL_PATH: &'static str = "./src/real_input.txt";


fn part1(raw_data:&str) -> usize{

    let data_mat:Vec<Vec<i32>> = raw_data.split("\n").map(|row| row.chars().map(|c| c.to_digit(10).unwrap() as i32).collect()).collect();
    let data_map_t:Vec<Vec<i32>> = (0..data_mat[0].len()).map(|i| data_mat.iter().map(|inner| inner[i].clone()).collect()).collect();
    let mut n = data_mat.len() *2 + (data_mat[0].len()-2)*2;

    for row in 1..data_mat.len()-1{
        for col in 1..data_mat[0].len()-1{
            let tree = data_mat[row][col];

            let left = &data_mat[row][0..col];
            let right = &data_mat[row][col+1..];
            let up = &data_map_t[col][0..row];
            let down = &data_map_t[col][row+1..];
            let directions = [left,right,up,down];
            let check:Vec<&&[i32]> = directions.iter().filter(|x| x.iter().max().unwrap() >= &tree).collect();
            if check.len() < directions.len(){
                n += 1;
            }
        }
    }
    return n;
}
fn part2(raw_data:&str) -> usize{

    let data_mat:Vec<Vec<i32>> = raw_data.split("\n").map(|row| row.chars().map(|c| c.to_digit(10).unwrap() as i32).collect()).collect();
    let data_map_t:Vec<Vec<i32>> = (0..data_mat[0].len()).map(|i| data_mat.iter().map(|inner| inner[i].clone()).collect()).collect();
 
    let mut max_score = 0;
    for row in 1..data_mat.len()-1{
        for col in 1..data_mat[0].len()-1{
            let tree = data_mat[row][col];

            let left:Vec<i32> = data_mat[row][0..col].iter().map(|x|0.max(tree - x)).rev().collect();
            let right:Vec<i32> = data_mat[row][col+1..].iter().map(|x|0.max(tree - x)).collect();
            let up:Vec<i32> = data_map_t[col][0..row].iter().map(|x|0.max(tree - x)).rev().collect();
            let down:Vec<i32> = data_map_t[col][row+1..].iter().map(|x|0.max(tree - x)).collect();
            let directions = [up,left,down,right];
            
            let scenic_score:usize = Product::product(directions.map(|x| x.iter().position(|i| *i==0).unwrap_or(x.len()-1)+1).iter());
          
            if scenic_score  > max_score{
                max_score = scenic_score;
            }
        }   
    }
    return max_score;
}



fn main() {
    let test_data = fs::read_to_string(TEST_PATH).expect("Should have been able to read the file");
    let real_data = fs::read_to_string(REAL_PATH).expect("Should have been able to read the file");
    
    println!("PART 1");
    let test_result = part1(&test_data);
    println!("Test: {}", test_result);
    let real_result = part1(&real_data);
    println!("Real: {}", real_result);

    println!("PART 2");
    let test_result = part2(&test_data);
    println!("Test: {}", test_result);
    let real_result = part2(&real_data);
    println!("Real: {}", real_result);
}
