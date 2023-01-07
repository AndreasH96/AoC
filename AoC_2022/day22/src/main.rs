use day22::{adjust_pos, transition};
use num::Complex;
use std::ops::Neg;
use std::{collections::HashMap, fs};
const TEST_PATH: &'static str = "./src/test_input.txt";
const TEST_PATH_2: &'static str = "./src/test_input_altered.txt";
const REAL_PATH: &'static str = "./src/real_input.txt";

fn create_map(
    map_raw: &str,
) -> (
    (i8, i8),
    usize,
    (usize, usize),
    HashMap<(i8, i8), Vec<Vec<char>>>,
) {
    let mut boards: HashMap<(i8, i8), Vec<Vec<char>>> = HashMap::new();

    let tile_dim = map_raw.lines().map(|x| x.trim().len()).min().unwrap();
    let lines: Vec<String> = map_raw.lines().map(|x| x.to_string()).collect();
    let mut starting_pos = (tile_dim as i8, tile_dim as i8);
    let line_chunks = lines.chunks(tile_dim);
    let mut num_tiles = (line_chunks.len(), 0);

    for (i, map_chunk) in line_chunks.enumerate() {
        let mut rows: HashMap<usize, Vec<Vec<char>>> = HashMap::new();
        for chunk in map_chunk {
            let chars: Vec<char> = chunk.chars().collect();
            let char_chunks = chars.chunks(tile_dim);
            num_tiles.1 = num_tiles.1.max(char_chunks.len());
            for (j, c) in char_chunks.enumerate() {
                if c[0] != ' ' {
                    if i < starting_pos.0 as usize && j < starting_pos.1 as usize {
                        starting_pos = (i as i8, j as i8);
                    }
                    if !rows.contains_key(&j) {
                        rows.insert(j, Vec::new());
                    }

                    let r = rows.get(&j).unwrap().len();
                    rows.get_mut(&j).unwrap().insert(r, c.to_vec());
                }
            }
        }
        for r in rows.keys() {
            boards.insert((i as i8, *r as i8), rows.get(&r).unwrap().clone());
        }
    }
    return (starting_pos, tile_dim, num_tiles, boards);
}

fn get_cube_alignment(
    side_coords: Vec<(i8, i8)>,
    num_tiles: &(usize, usize),
) -> (HashMap<&str, ((i8, i8), bool)>, HashMap<(i8, i8), &str>) {
    let mut top: (i8, i8) = (0, 0);

    for i in 0..num_tiles.1 {
        let s: Vec<&(i8, i8)> = side_coords.iter().filter(|x| x.1 == i as i8).collect();
        if s.len() == 3 {
            let m = s.iter().map(|x| x.0).min().unwrap();
            top = (m + 1, s[0].1);
            break;
        }
    }
    let (back, front) = ((top.0 - 1, top.1), (top.0 + 1, top.1));

    let right_t: Vec<((i8, i8), bool)> = vec![
        ((front.0, front.1 + 1), false),
        ((back.0, back.1 + 1), true),
    ]
    .iter()
    .filter(|x| side_coords.contains(&x.0))
    .map(|x| *x)
    .collect();
    let right = right_t[0];

    let left_t: Vec<((i8, i8), bool)> =
        vec![((top.0, top.1 - 1), false), ((front.0, front.1 - 1), true)]
            .iter()
            .filter(|x| side_coords.contains(&x.0))
            .map(|x| *x)
            .collect();
    let left = left_t[0];

    let bottom_t: Vec<((i8, i8), bool)> = vec![
        ((left.0 .0, left.0 .1 - 1), false),
        ((left.0 .0 + 1, left.0 .1), true),
    ]
    .iter()
    .filter(|x| side_coords.contains(&x.0))
    .map(|x| *x)
    .collect();
    let bottom = bottom_t[0];

    (
        HashMap::from([
            ("top", (top, false)),
            ("front", (front, false)),
            ("back", (back, false)),
            ("right", right),
            ("left", left),
            ("bottom", bottom),
        ]),
        HashMap::from([
            (top, "top"),
            (front, "front"),
            (back, "back"),
            (right.0, "right"),
            (left.0, "left"),
            (bottom.0, "bottom"),
        ]),
    )
}

fn part1(text_input: String) -> usize {
    let parts: Vec<&str> = text_input.split("\n\n").collect();

    let (mut current_block, block_size, num_tiles, map) = create_map(parts[0]);

    let mut instructions_raw = parts[1].to_string();
    instructions_raw = instructions_raw.replace("R", " R ");
    instructions_raw = instructions_raw.replace("L", " L ");

    let instructions: Vec<&str> = instructions_raw
        .split(" ")
        .map(|x| x.trim())
        .filter(|x| *x != "")
        .collect();

    let mut current_pos = Complex::new(0, 0);

    let directions: Vec<Complex<i32>> = vec![
        Complex::new(0, 1),
        Complex::new(1, 0),
        Complex::new(0, -1),
        Complex::new(-1, 0),
    ];
    let mut current_dir = 0;

    for inst in instructions {
        match inst {
            "R" => {
                current_dir =
                    ((current_dir as i16 + 1).rem_euclid(directions.len() as i16)) as usize;
            }
            "L" => {
                current_dir =
                    ((current_dir as i16 - 1).rem_euclid(directions.len() as i16)) as usize;
            }
            _ => {
                let val: i32 = inst.parse().unwrap();
                let dirr = directions[current_dir];
                let mut new_pos = current_pos.clone();
                for _ in 0..val {
                    new_pos += dirr;
                    new_pos = adjust_pos(
                        &mut new_pos,
                        &mut current_block,
                        block_size,
                        &num_tiles,
                        &map,
                    );
                    let curr_block = map.get(&current_block).unwrap();
                    if curr_block[new_pos.re as usize][new_pos.im as usize] == '#' {
                        new_pos -= dirr;
                    }
                    new_pos = adjust_pos(
                        &mut new_pos,
                        &mut current_block,
                        block_size,
                        &num_tiles,
                        &map,
                    );
                }
                current_pos = new_pos;
            }
        }
    }
    let r = ((current_block.0 as usize) * block_size) + current_pos.re as usize + 1;
    let c = (current_block.1 as usize) * block_size + current_pos.im as usize + 1;
    let password = (1000 * r) + (4 * c) + current_dir;
    return password;
}
// Example should result in 10006
fn part2(text_input: String) -> usize {
    let parts: Vec<&str> = text_input.split("\n\n").collect();

    let (mut current_block, block_size, num_tiles, map) = create_map(parts[0]);

    let keys: Vec<(i8, i8)> = map.keys().map(|x| *x).collect();

    let cube_alignment = get_cube_alignment(keys, &num_tiles);

    let mut instructions_raw = parts[1].to_string();
    instructions_raw = instructions_raw.replace("R", " R ");
    instructions_raw = instructions_raw.replace("L", " L ");

    let instructions: Vec<&str> = instructions_raw
        .split(" ")
        .map(|x| x.trim())
        .filter(|x| *x != "")
        .collect();

    let mut current_pos = Complex::new(0, 0);

    let directions: Vec<Complex<i32>> = vec![
        Complex::new(0, 1),
        Complex::new(1, 0),
        Complex::new(0, -1),
        Complex::new(-1, 0),
    ];
    let mut current_dir = 0;

    for inst in instructions {
        match inst {
            "R" => {
                current_dir =
                    ((current_dir as i16 + 1).rem_euclid(directions.len() as i16)) as usize;
            }
            "L" => {
                current_dir =
                    ((current_dir as i16 - 1).rem_euclid(directions.len() as i16)) as usize;
            }
            _ => {
                let val: i32 = inst.parse().unwrap();
                let mut dirr = directions[current_dir];
                let mut new_pos = current_pos.clone();
                for _ in 0..val {
                    let last_dirr = current_dir.clone();
                    let last_pos = new_pos.clone();
                    let last_block = current_block.clone();
                    new_pos += dirr;

                    if [1.neg(), block_size as i32].contains(&new_pos.im)
                        || [1.neg(), block_size as i32].contains(&new_pos.re)
                    {
                        let (d, b, p) = transition(
                            cube_alignment.1.get(&current_block).unwrap(),
                            current_dir,
                            new_pos,
                            block_size as i32,
                        );
                        current_dir = d;
                        current_block = cube_alignment.0.get(b).unwrap().0;
                        new_pos = p;
                    }

                    dirr = directions[current_dir];
                    let curr_block = map.get(&current_block).unwrap();
                    if curr_block[new_pos.re as usize][new_pos.im as usize] == '#' {
                        new_pos = last_pos;
                        current_block = last_block;
                        current_dir = last_dirr;
                        current_pos = new_pos;
                        break;
                    }

                    current_pos = new_pos;
                }
            }
        }
    }

    let r = ((current_block.0 as usize) * block_size) + current_pos.re as usize + 1;
    let c = (current_block.1 as usize) * block_size + current_pos.im as usize + 1;
    let password = (1000 * r) + (4 * c) + current_dir;
    return password;
}

fn main() {
    let test_input = fs::read_to_string(TEST_PATH).expect("Should have been able to read the file");
    let test_input1 =
        fs::read_to_string(TEST_PATH_2).expect("Should have been able to read the file");
    let real_input = fs::read_to_string(REAL_PATH).expect("Should have been able to read the file");

    println!("PART 1");
    use std::time::Instant;
    let now = Instant::now();
    println!("Test: {}", part2(test_input.clone()));
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}\n", elapsed);
    let now = Instant::now();
    println!("Real: {}", part1(real_input.clone()));
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}\n", elapsed);
    let now = Instant::now();

    println!("PART 2");
    println!("Test: {:?}", part2(test_input1.clone()));
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
    let now = Instant::now();
    println!("Real: {}", part2(real_input));
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}
