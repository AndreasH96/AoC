use std::{
    collections::{HashMap, HashSet},
    fs,
    ops::Neg,
    vec,
};


const TEST_PATH: &'static str = "./src/test_input.txt";
const REAL_PATH: &'static str = "./src/real_input.txt";

const LINE_WIDTH: usize = 7;

fn rotate_shape(
    inp: &Vec<String>,
    left: bool,
    active_shape_pos: (usize, usize),
    current_game_board: &Vec<String>,
) -> usize {
    // Check if would be out of map
    if (left && active_shape_pos.0 == 0)
        || (!left && (active_shape_pos.0 + inp[0].len()) == LINE_WIDTH)
    {
        return active_shape_pos.0;
    }

    // Check for collisions
    // Checks whether a move of the block would make any rock disappear.
    let gm: Vec<Vec<String>> = current_game_board
        [active_shape_pos.1..active_shape_pos.1 + inp.len()]
        .iter()
        .map(|x| x.chars().map(|y| y.to_string()).collect())
        .collect();

    let d: i32 = if left { 1.neg() } else { 1 };

    for i in 0..gm.len() {
        for j in active_shape_pos.0..active_shape_pos.0 + inp[0].len() {
            if gm[i][(j as i32 + d) as usize] == "#"
                && inp[i]
                    .get(j - active_shape_pos.0..=j - active_shape_pos.0)
                    .unwrap()
                    == "@"
            {
                // Collision, perform no move
                return active_shape_pos.0;
            }
        }
    }

    // If no collisions, perform move
    if left {
        return active_shape_pos.0 - 1;
    } else {
        return active_shape_pos.0 + 1;
    }
}

fn solve(input_string: String, rocks: usize) -> u128 {
    let shapes: Vec<Vec<String>> = Vec::from(vec![
        vec!["@@@@".to_string()],
        vec![".@.".to_string(), "@@@".to_string(), ".@.".to_string()],
        vec!["..@".to_string(), "..@".to_string(), "@@@".to_string()],
        vec![
            "@".to_string(),
            "@".to_string(),
            "@".to_string(),
            "@".to_string(),
        ],
        vec!["@@".to_string(), "@@".to_string()],
    ]);
    let mut fallen_blocks = 0;
    let mut counter = 0;
    let mut active_shape: &Vec<String> = &shapes.get(0).unwrap();

    let empty_row: String = ".......".to_string();

    let mut game_board: Vec<String> = Vec::from(vec![
        ".......".to_string(),
        ".......".to_string(),
        ".......".to_string(),
        ".......".to_string(),
    ]);
    let mut active_shape_x: usize = 2;
    let mut active_shape_y: i32 = 0;
    let mut height: u128 = 0;
    let mut prev_index = 0;
    let input_chars: Vec<char> = input_string.chars().collect();

    let mut pattern_indexes: HashMap<usize, (usize, u128)> = HashMap::new();
    let mut found_repeated = false;

    while fallen_blocks < rocks {
        let mv = input_chars[counter % input_chars.len()];
        counter += 1;
        active_shape_x = rotate_shape(
            &active_shape,
            mv == '<',
            (active_shape_x, active_shape_y as usize),
            &game_board,
        );
        // If reached bottom of map
        if active_shape_y as usize + active_shape.len() == game_board.len() {
            fallen_blocks += 1;
            for r in active_shape.clone() {
                let insert_string = format!(
                    "{}{}{}",
                    ".".repeat(active_shape_x),
                    r,
                    ".".repeat(7 - (active_shape_x + active_shape[0].len()))
                );
                game_board.insert(game_board.len(), insert_string.replace("@", "#"));
            }
            active_shape = shapes.get(fallen_blocks % shapes.len()).unwrap();
            active_shape_x = 2;
            active_shape_y = 0;

            while game_board[0..3].iter().any(|x| x.contains("#")) {
                game_board.insert(0, empty_row.clone());
            }
            for _ in 0..active_shape.len() - 1 {
                game_board.insert(0, empty_row.clone())
            }
        } else {
            for i in 0..active_shape.len() {
                let row = active_shape.get(i).unwrap();
                let row_match: HashSet<usize> = row.match_indices("@").map(|x| x.0).collect();

                let map_row_below = game_board.get(active_shape_y as usize + i + 1).unwrap();
                let map_match: HashSet<usize> = map_row_below
                    [active_shape_x..active_shape_x + row.len()]
                    .match_indices("#")
                    .map(|x| x.0)
                    .collect();

                if row_match.iter().any(|x| map_match.contains(&x)) {
                    fallen_blocks += 1;
                    for (r, row) in active_shape.iter().enumerate() {
                        let mut row_vec: Vec<String> = game_board[r + active_shape_y as usize]
                            .chars()
                            .map(|x| x.to_string())
                            .collect();
                        for (c, col) in row.chars().enumerate() {
                            if col == '@' {
                                row_vec[c + active_shape_x] = "#".to_string();
                            }
                        }
                        game_board[r + active_shape_y as usize] = row_vec.join("");
                    }

                    active_shape = shapes.get(fallen_blocks % shapes.len()).unwrap();
                    active_shape_x = 2;
                    active_shape_y = -1;

                    let contains_rock: Vec<&String> =
                        game_board.iter().filter(|x| x.contains("#")).collect();
                    let new_index = contains_rock.len();
                    height += new_index as u128 - prev_index as u128;
                    prev_index = new_index;

                    if (fallen_blocks - 1) % shapes.len() == 0 && !found_repeated {
                        if pattern_indexes.len() > 1
                            && *pattern_indexes.keys().min().unwrap() > counter % input_chars.len()
                        {
                            pattern_indexes.clear();
                        }

                        if pattern_indexes.contains_key(&(&counter % input_chars.len())) {
                            found_repeated = true;
                            let curr = pattern_indexes
                                .get(&(&counter % input_chars.len()))
                                .unwrap();

                           
                            let m = fallen_blocks - curr.0;
                            let k = height - curr.1;


                            height += k * ((rocks as u128 - fallen_blocks as u128) / m as u128);
                            fallen_blocks += ((rocks - fallen_blocks) / m) * m;

                        } else {
                            pattern_indexes
                                .insert(&counter % input_chars.len(), (fallen_blocks, height));
                        }
                    }

                    while !game_board[0].contains("#") {
                        game_board.remove(0);
                    }

                    while game_board[0..3].iter().any(|x| x.contains("#")) {
                        game_board.insert(0, empty_row.clone());
                    }
                    for _ in 0..active_shape.len() {
                        game_board.insert(0, empty_row.clone());
                    }

                    while game_board.len() > 200 {
                        prev_index -= 1;
                        game_board.pop();
                    }

                    break;
                }
            }
            //println!("{:?}",rocks-fallen_blocks);
            active_shape_y += 1;
        }
    }

    return height;
}

fn part1(input_string: String) -> u128 {
    return solve(input_string, 2022);
}

fn part2(input_string: String) -> u128 {
    return solve(input_string, 1000000000000);
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
