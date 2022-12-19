use std::fs;
const BLOCK_ORDER_PATH : &'static str = "./src/block_order.txt";
const TEST_PATH: &'static str = "./src/test_input.txt";
const REAL_PATH: &'static str = "./src/real_input.txt";

const LINE_WIDTH:usize = 7;


fn rotate_shape(inp:  Vec<String>,left:bool,active_shape_x:usize) -> (Vec<String>,usize){



    let shape_width = inp.iter().map(|x| x.chars().filter(|y| *y =='@').collect::<Vec<char>>()).map(|x| x.len()).max().unwrap();
    println!("{:?}",shape_width+active_shape_x);
    if (left && active_shape_x == 0) || (!left && (active_shape_x+shape_width) ==LINE_WIDTH ) {
        return (inp,active_shape_x);
    }


    let mut t:Vec<Vec<String>> = inp.iter().map(|x| x.chars().map(|x| x.to_string()).collect()).collect();
    let mut new_pos = active_shape_x.clone();
    for i in 0..t.len(){
        if left{
            new_pos = active_shape_x-1;
            t[i].rotate_left(1);
        }
        else {
            new_pos = active_shape_x+1;
            t[i].rotate_right(1);
        }
    }
    let res:Vec<String> = t.iter().map(|x| x.join("").to_string()).collect();
    return (res,new_pos);
}

fn main() {
    let block_order_str = fs::read_to_string(BLOCK_ORDER_PATH).expect("Should have been able to read the file");
    let test_input = fs::read_to_string(TEST_PATH).expect("Should have been able to read the file");
    
    
    let mut field:Vec<Vec<i32>> = Vec::new();
    let shapes:Vec<Vec<String>>= Vec::from(vec![
        vec!["..@@@@.".to_string()],
        vec!["...@...".to_string(),"..@@@..".to_string(),"...@...".to_string()],
        vec!["....@..".to_string(),"....@..".to_string(),"..@@@..".to_string()],
        vec!["..@....".to_string(),"..@....".to_string(),"..@....".to_string(),"..@....".to_string()],
        vec!["..@@...".to_string(),"..@@...".to_string()]
        ]
    );
   
    let mut active_shape:Vec<String> = shapes.get(1).unwrap().clone();

    let mut game_board:Vec<Vec<String>>= Vec::from(vec![
        vec![".......".to_string()],
        vec![".......".to_string()],
        vec![".......".to_string()],
        ]
    );
    let mut active_shape_x:usize=2;
    let mut active_shape_y :usize= 0;
    
    for x in active_shape.clone(){
        println!("{:?}",x);
    }
    for mv in  test_input.chars(){
        let new_state:(Vec<String>,usize) =  rotate_shape(active_shape.clone(), mv=='<',active_shape_x);
        active_shape = new_state.0;
        active_shape_x = new_state.1;
        
        if game_board.len() == active_shape_y {
            game_board
        }




        for x in active_shape.clone(){
            println!("{:?}",x);
        }
      
    }
    







    //let real_input = fs::read_to_string(REAL_PATH).expect("Should have been able to read the file");




    //println!("PART 1");
    //use std::time::Instant;
    //let now = Instant::now();
    //println!("Test: {}", part1(&test_input, 10));
    //let elapsed = now.elapsed();
    //println!("Elapsed: {:.2?}", elapsed);
    //let now = Instant::now();
    //println!("Real: {}", part1(&real_input, 2000000));
    //let elapsed = now.elapsed();
    //println!("Elapsed: {:.2?}\n", elapsed);
    //let now = Instant::now();
//
    //println!("PART 2");
    //println!("Test: {:?}", part2(&test_input, (0, 20)));
    //let elapsed = now.elapsed();
    //println!("Elapsed: {:.2?}", elapsed);
    //let now = Instant::now();
    //println!("Real: {}", part2(&real_input, (0, 4000000)));
    //let elapsed = now.elapsed();
    //println!("Elapsed: {:.2?}", elapsed);
}

// Create two sets, one for sensor positions and one for beacon positions *
// Create map with distance between sensors and nearest beacon, key = sensor pos, val = beacon pos and manhattan distance pos *

// Find X range of beacon ranges for sensors, extract min X and max X from this
// Go to row Y, loop through X positions, check if it's distance to any Sensor is less than their beacon
// If less than or equal, cannot be placed there
// Else, can be placed there
