use lazy_static::lazy_static; // 1.4.0
use parking_lot::Mutex;
use std::{collections::HashSet, fs, ops::Neg};
const TEST_PATH: &'static str = "./src/test_input.txt";
const REAL_PATH: &'static str = "./src/real_input.txt";

lazy_static! {
    static ref KNOWN_BLOCKED: Mutex<HashSet<(i32, i32, i32)>> = Mutex::new(HashSet::new());
}

fn get_adjacent(
    blocks: &HashSet<(i32, i32, i32)>,
    block: (i32, i32, i32),
    want_air: bool,
) -> Vec<(i32, i32, i32)> {
    let additions: Vec<(i32, i32, i32)> = vec![
        (1, 0, 0),
        (1.neg(), 0, 0),
        (0, 1, 0),
        (0, 1.neg(), 0),
        (0, 0, 1),
        (0, 0, 1.neg()),
    ];
    let adjacent_map = additions.iter().map(|x| {
        (
            block.0 as i32 + x.0,
            block.1 as i32 + x.1,
            block.2 as i32 + x.2,
        )
    });

    let mut adjacent: Vec<(i32, i32, i32)> = Vec::new();
    if !want_air {
        adjacent = adjacent_map.filter(|x| blocks.contains(x)).collect();
    } else {
        adjacent = adjacent_map.filter(|x| !blocks.contains(x)).collect();
    }

    return adjacent;
}

// Checks if there is at least one direction in which the block is free
fn touches_air(blocks: &HashSet<(i32, i32, i32)>, block: &(i32, i32, i32)) -> bool {
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

fn on_edge(map: &HashSet<(i32, i32, i32)>, start: &(i32, i32, i32)) -> bool {
    let mut queue: Vec<(i32, i32, i32)> = Vec::new();
    let mut explored: HashSet<(i32, i32, i32)> = HashSet::new();

    get_adjacent(map, *start, true)
        .iter()
        .for_each(|x| queue.insert(0, *x));

    while queue.len() > 0 {
        let current = queue.pop().unwrap();

        if touches_air(map, &current) {
            return true;
        }

        if !KNOWN_BLOCKED.lock().contains(&current) && !map.contains(&current) {
            explored.insert(current);
            get_adjacent(map, current, true).iter().for_each(|x| {
                if !explored.contains(x) && !queue.contains(x) {
                    queue.insert(0, *x)
                }
            });
        }
    }
    explored.iter().for_each(|x| {
        KNOWN_BLOCKED.lock().insert(*x);
    });
    return false;
}

fn get_free_sides(
    blocks: &HashSet<(i32, i32, i32)>,
    block: (i32, i32, i32),
) -> Vec<(i32, i32, i32)> {
    let additions: Vec<(i32, i32, i32)> = vec![
        (1, 0, 0),
        (1.neg(), 0, 0),
        (0, 1, 0),
        (0, 1.neg(), 0),
        (0, 0, 1),
        (0, 0, 1.neg()),
    ];
    let adjacent: Vec<(i32, i32, i32)> = additions
        .iter()
        .map(|x| {
            (
                block.0 as i32 + x.0,
                block.1 as i32 + x.1,
                block.2 as i32 + x.2,
            )
        })
        .filter(|x| !blocks.contains(x) && on_edge(blocks, x))
        .collect();

    return adjacent;
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
        touching_faces += get_adjacent(&blocks, block, false).len() as i32;
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
        .filter(|x| on_edge(&blocks, &*x))
        .map(|x| *x)
        .collect();

    let mut free_sides: i32 = 0;
    brink.iter().for_each(|b| {
        free_sides += get_free_sides(&blocks, *b).len() as i32;
    });
    KNOWN_BLOCKED.lock().clear();
    return free_sides;
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
    println!("Real: {}", part2(real_input));
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}
