use std::{collections::HashSet, fs, ops::Neg};
const TEST_PATH: &'static str = "./src/test_input.txt";
const REAL_PATH: &'static str = "./src/real_input.txt";

fn get_number_of_adjacent(blocks: &HashSet<(i32, i32, i32)>, block: (i32, i32, i32)) -> i32 {
    let additions: Vec<(i32, i32, i32)> = vec![
        (1, 0, 0),
        (1.neg(), 0, 0),
        (0, 1, 0),
        (0, 1.neg(), 0),
        (0, 0, 1),
        (0, 0, 1.neg()),
    ];
    let adjacents: Vec<(i32, i32, i32)> = additions
        .iter()
        .map(|x| {
            vec![
                block.0 as i32 + x.0,
                block.1 as i32 + x.1,
                block.2 as i32 + x.2,
            ]
        })
        .filter(|x| x.iter().all(|y| *y >= 0))
        .map(|x| (x[0] as i32, x[1] as i32, x[2] as i32))
        .filter(|x| blocks.contains(x))
        .collect();

    return adjacents.len() as i32;
}

// Checks if there is at least one direction in which the block is free
fn on_brink(blocks: &HashSet<(i32, i32, i32)>, block: &(i32, i32, i32)) -> bool {
    let max_x = blocks.iter().map(|x| x.0).max().unwrap();
    let min_x = blocks.iter().map(|x| x.0).min().unwrap();

    let max_y = blocks.iter().map(|x| x.1).max().unwrap();
    let min_y = blocks.iter().map(|x| x.1).min().unwrap();

    let max_z = blocks.iter().map(|x| x.2).max().unwrap();
    let min_z = blocks.iter().map(|x| x.2).min().unwrap();

    let max_c: Vec<std::ops::Range<i32>> = vec![
        min_x - 1..block.0,
        block.0 + 1..max_x + 1,
        min_y - 1..block.1,
        block.1 + 1..max_y + 1,
        min_z - 1..block.2,
        block.2 + 1..max_z + 1,
    ];
    let mut free_directions: i32 = 6;
    for (i, r) in max_c.iter().enumerate() {
        let b = vec![block.0, block.1, block.2];
        let ind = (i / 2) as usize;

        for p in r.clone().into_iter() {
            let mut pos = b.clone();
            pos[ind] = p;

            let pos_tuple = (pos[0], pos[1], pos[2]);
            if blocks.contains(&pos_tuple) {
                free_directions -= 1;
                break;
            }
        }
    }

    return free_directions > 0;
}

fn part1(input_data: String) -> i32 {
    let mut blocks: HashSet<(i32, i32, i32)> = HashSet::new();

    let data: Vec<Vec<i32>> = input_data
        .lines()
        .map(|x| x.split(",").map(|y| y.parse::<i32>().unwrap()).collect())
        .collect();

    let mut touching_faces: i32 = 0;

    for line in data {
        let block = (line[0], line[1], line[2]);
        touching_faces += get_number_of_adjacent(&blocks, block);
        blocks.insert(block);
    }
    return 6 * blocks.len() as i32 - 2 * touching_faces;
}

fn part2(input_data: String) -> i32 {
    let mut blocks: HashSet<(i32, i32, i32)> = HashSet::new();

    input_data
        .lines()
        .map(|x| x.split(",").map(|y| y.parse::<i32>().unwrap()).collect())
        .for_each(|line: Vec<i32>| {
            blocks.insert((line[0], line[1], line[2]));
        });

    let brink: HashSet<(i32, i32, i32)> = blocks
        .iter()
        .filter(|x| on_brink(&blocks, &*x))
        .map(|x| *x)
        .collect();
        
    let mut touching_faces: i32 = 0;
    brink.iter().for_each(|b| {
        touching_faces += get_number_of_adjacent(&blocks, *b);
    });

    return 6 * brink.len() as i32 - touching_faces;
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

    println!("PART 2");
    println!("Test: {:?}", part2(test_input));
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
    let now = Instant::now();
    println!("Real: {}", part2(real_input) - 2510);
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}
