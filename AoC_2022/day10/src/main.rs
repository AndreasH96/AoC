use std::{fs, collections::HashMap};
const TEST_PATH: &'static str = "./src/test_input.txt";
const REAL_PATH: &'static str = "./src/real_input.txt";



fn part1(raw_data:&str) -> i32 {
    let operations:Vec<Vec<&str>> = raw_data.lines().map(|x| x.split_whitespace().collect()).collect();
    let mut cycle:i32  = 0 ;
    let cycle_deltas:HashMap<&str, i32> = HashMap::from([("addx",2),("noop",1)]);
    let mut sig = 0;
    let mut x = 1;
    for op in operations{
        for _ in 0..cycle_deltas[op[0]]{
            cycle+=1;
            if (cycle -20) % 40 == 0 && cycle <= 220{
                sig+= x*cycle;
            }
        }
        if op[0] == "addx"{
            x += op[1].parse::<i32>().unwrap();
         
        }
        
    }
    return sig;
}



fn part2(raw_data:&str) {
    let operations:Vec<Vec<&str>> = raw_data.lines().map(|x| x.split_whitespace().collect()).collect();
    let mut cycle:i32  = 0 ;
    let cycle_deltas:HashMap<&str, i32> = HashMap::from([("addx",2),("noop",1)]);
    let mut x = 1;
    for op in operations{
        for _ in 0..cycle_deltas[op[0]]{
            cycle+=1;
            if  (x-1..=x+1).contains(&((cycle-1)%40)){
                print!("#")
            }
            else{
                print!(".")
            }
            
            if cycle % 40 == 0 {
                println!()
            }
        }
        if op[0] == "addx"{
            x += op[1].parse::<i32>().unwrap();
         
        }
        
    }
}


fn main() {
    let test_data = fs::read_to_string(TEST_PATH).expect("Should have been able to read the file");
    let real_data = fs::read_to_string(REAL_PATH).expect("Should have been able to read the file");
    
    println!("PART 1");
    let test_result = part1(&test_data);
    println!("Test: {}", test_result);
    let real_result = part1(&real_data);
    println!("Real: {}", real_result);

    println!("\nPART 2");
    part2(&test_data);
    println!();
    part2(&real_data);
}
