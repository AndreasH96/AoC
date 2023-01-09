use num::{
    integer::{lcm},
};
use std::{collections::HashSet, fs,};

const TEST_PATH: &'static str = "./src/test_input.txt";
const REAL_PATH: &'static str = "./src/real_input.txt";

fn insert_cycle_positions(
    (height, width): (usize, usize),
    cycle_positions: &mut Vec<HashSet<(i16, i16)>>,
    start: (i16, i16),
    direction: (i16, i16),
) {
    let mut insertion_vector: Vec<(i16, i16)>;
    let val;
    if direction.0 != 0 {
        insertion_vector = (1..width - 1).map(|x| (x as i16, start.1)).collect();
        val = direction.0;
    } else {
        insertion_vector = (1..height - 1).map(|y| (start.0, y as i16)).collect();
        val = direction.1;
    }
    if val < 0 {
        insertion_vector.reverse();
    }
    while start != insertion_vector[0] {
        insertion_vector.rotate_left(1);
    }
    for (i, insertion) in insertion_vector.iter().enumerate() {
        cycle_positions[i].insert(*insertion);
    }
}

fn create_cycle_states(lines: Vec<&str>) -> (Vec<HashSet<(i16, i16)>>, (usize, usize)) {
    let (height, width) = (lines.len(), lines[0].len());

    let cycle_size = lcm(height - 2, width - 2);
    let mut vertical_cycle: Vec<HashSet<(i16, i16)>> =
        (0..height - 2).map(|_| HashSet::new()).collect();

    let mut horizontal_cycle: Vec<HashSet<(i16, i16)>> =
        (0..width - 2).map(|_| HashSet::new()).collect();
    let mut cycle_positions: Vec<HashSet<(i16, i16)>> =
        (0..cycle_size).map(|_| HashSet::new()).collect();

    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                '>' => insert_cycle_positions(
                    (height, width),
                    &mut horizontal_cycle,
                    (x as i16, y as i16),
                    (1, 0),
                ),
                'v' => insert_cycle_positions(
                    (height, width),
                    &mut vertical_cycle,
                    (x as i16, y as i16),
                    (0, 1),
                ),
                '<' => insert_cycle_positions(
                    (height, width),
                    &mut horizontal_cycle,
                    (x as i16, y as i16),
                    (-1, 0),
                ),
                '^' => insert_cycle_positions(
                    (height, width),
                    &mut vertical_cycle,
                    (x as i16, y as i16),
                    (0, -1),
                ),
                _ => (),
            }
        }
    }
    for i in 0..cycle_size {
        cycle_positions[i].extend(vertical_cycle[i.rem_euclid(vertical_cycle.len())].iter());
        cycle_positions[i].extend(horizontal_cycle[i.rem_euclid(horizontal_cycle.len())].iter());
    }

    return (cycle_positions, (height, width));
}
fn get_possible_moves(
    pos: (i16, i16, usize, usize),
    state: &HashSet<(i16, i16)>,
    cycle_size: usize,
    (height, width): (usize, usize),
    visited: &HashSet<(i16, i16, usize)>,
    start: (i16, i16),
    goal: (i16, i16),
) -> Vec<(i16, i16, usize, usize)> {
    let deltas: Vec<(i16, i16)> = vec![(0, 0), (1, 0), (0, 1), (-1, 0), (0, -1)];
    let mut moves: Vec<(i16, i16, usize, usize)> = Vec::new();
    for (dx, dy) in deltas {
        let (x, y) = (pos.0 + dx, pos.1 + dy);

        if !state.contains(&(x, y))
            && !visited.contains(&(x, y, (pos.3 + 1).rem_euclid(cycle_size)))
        {
            if (x, y) == start
                || (x, y) == goal
                || (x > 0 && x < width as i16 - 1 && y > 0 && y < height as i16 - 1)
            {
                moves.insert(
                    0,
                    (
                        x,
                        y,
                        (pos.2 + 1) as usize,
                        (pos.3 + 1).rem_euclid(cycle_size) as usize,
                    ),
                )
            }
        }
    }
    return moves;
}


fn bfs(
    cycle_states: Vec<HashSet<(i16, i16)>>,
    (height, width): (usize, usize),
    start: (i16, i16),
    goal: (i16, i16),
    start_time: usize,
) -> usize {
    let mut queue: Vec<(i16, i16, usize, usize)> = Vec::new();
    let mut visited: HashSet<(i16, i16, usize)> = HashSet::new();
    let cycle_size = lcm(height - 2, width - 2);
    queue.insert(
        0,
        (
            start.0,
            start.1,
            start_time,
            start_time.rem_euclid(cycle_size),
        ),
    );

    while !queue.is_empty() {
        let current = queue.pop().unwrap();

        if (current.0, current.1) == goal {
            return current.2;
        }

        let moves = get_possible_moves(
            current,
            &cycle_states[(current.3 as usize + 1).rem_euclid(cycle_size)],
            cycle_size,
            (height, width),
            &visited,
            start,
            goal,
        );

        visited.insert((current.0, current.1, current.3));
        for m in moves {
            if !visited.contains(&(m.0, m.1, m.3)) && !queue.contains(&m) {
                queue.insert(0, m);
            }
        }
    }

    return 0;
}
fn part1(text_input: String) -> usize {
    let lines: Vec<&str> = text_input.lines().collect();
    let (cycle_states, (height, width)) = create_cycle_states(lines);
    let start = (1, 0);
    let goal = (width as i16 - 2, height as i16 - 1);
    return bfs(cycle_states, (height, width), start, goal, 0);
}

fn part2(text_input: String) -> usize {
    let lines: Vec<&str> = text_input.lines().collect();
    let ( cycle_states, (height, width)) = create_cycle_states(lines);
    let start = (1, 0);
    let goal = (width as i16 - 2, height as i16 - 1);
    let s1 = bfs(cycle_states.clone(), (height, width), start, goal, 0);
    let s2 = bfs(cycle_states.clone(), (height, width), goal, start, s1);
    return bfs(cycle_states, (height, width), start, goal, s2);
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
    //
    println!("PART 2");
    println!("Test: {:?}", part2(test_input));
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
    let now = Instant::now();
    println!("Real: {}", part2(real_input));
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}
