use std::fs;

const TEST_PATH :&'static str= "./src/test_input.txt";
const REAL_PATH :&'static str= "./src/real_input.txt";

macro_rules! calc_round_score {
    ($a:expr,$b:expr) => {
        ($a-87) + (1-((($a-$b)%3)%2)) * (6  -  3 * (($a-$b)%3)/2)
    };
    
}
fn part1(path:&str) ->i32{
    let contents = fs::read_to_string(path)
    .expect("Should have been able to read the file");
    // Turn "A Y\nB X... into [[1,2],[2,1]]..."
    let rows: Vec<Vec<i32>> = contents.split("\n").map(|r| vec![r.chars().next().unwrap() as i32 , r.chars().last().unwrap() as i32 ]).collect();
    let sum:i32 = rows.iter().map(|r| calc_round_score!(r[1],r[0])).sum();
    return sum;

}
fn part2(path:&str) ->i32{
    let contents = fs::read_to_string(path)
    .expect("Should have been able to read the file");
    // Turn "A Y\nB X... into [[1,2],[2,1]]..."
    let rows: Vec<Vec<i32>> = contents.split("\n").map(|r| vec![r.chars().next().unwrap() as i32 , r.chars().last().unwrap() as i32 ]).collect();
    
    // Modifiers depending on A,B,C and X, Y, Z
    //    X  Y  Z
    // A +2  0 +1
    // B -1  0 +1
    // C -1  0 +1
    // use this to turn i.e "A X" into "A Z" since we need to play "Z(scissors) to loose" 
    let modifier_matrix = vec![[2,0,1],[-1,0,1],[-1,0,-2]];

    let sum:i32 = rows.iter().map(|r| calc_round_score!(modifier_matrix[(r[0]-65) as usize][(r[1]-88) as usize] + 23 +r[0],r[0])).sum();
    return sum;

}



fn main() {

    println!("PART 1");
    let test_result = part1(TEST_PATH);
    println!("Test: {}", test_result);
    let real_result = part1(REAL_PATH);
    println!("Real: {}", real_result);

    println!("PART 2");
    let test_result = part2(TEST_PATH);
    println!("Test: {}", test_result);
    let real_result = part2(REAL_PATH);
    println!("Real: {}", real_result);
}