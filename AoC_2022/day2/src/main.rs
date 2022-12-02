use std::fs;
use std::collections::HashMap;
const TEST_PATH :&'static str= "./src/test_input.txt";
const REAL_PATH :&'static str= "./src/real_input.txt";


fn part1(path:&str)-> i64{
    let translate_map = HashMap::from([
        ("A X",1 + 3 ),
        ("B X",1 + 0),
        ("C X",1 + 6) ,
        ("A Y",2 + 6) ,
        ("B Y",2 + 3) ,
        ("C Y",2 + 0) ,
        ("A Z",3 + 0) ,
        ("B Z",3 + 6) ,
        ("C Z",3 + 3) ,
    ]);
    let contents = fs::read_to_string(path)
    .expect("Should have been able to read the file");
    let rows:Vec<&str> = contents.split("\n").collect();
    
    let mut score = 0;
    for row in &rows {
        score =  score + translate_map.get(row).unwrap();
    }
   return score;

}  

fn part2(path:&str)-> i64{
    let translate_map = HashMap::from([
        ("A X",3 + 0),
        ("B X",1 + 0),
        ("C X",2 + 0) ,
        ("A Y",1 + 3) ,
        ("B Y",2 + 3) ,
        ("C Y",3 + 3) ,
        ("A Z",2 + 6) ,
        ("B Z",3 + 6) ,
        ("C Z",1 + 6) ,
    ]);
    let contents = fs::read_to_string(path)
    .expect("Should have been able to read the file");
    let rows:Vec<&str> = contents.split("\n").collect();
    
    let mut score = 0;
    for row in &rows {
        score =  score + translate_map.get(row).unwrap();
    }
   return score;

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
