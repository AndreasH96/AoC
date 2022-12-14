use std::collections::HashSet;
use std::{fs, ops::Neg};
const TEST_PATH: &'static str = "./src/test_input.txt";
const REAL_PATH: &'static str = "./src/real_input.txt";

fn generate_line(line_corners: Vec<(i32, i32)>) -> Vec<(i32, i32)> {
    let mut occupied_points: Vec<(i32, i32)> = Vec::new();
    for i in 0..line_corners.len() - 1 {
        let (p1, p2) = (line_corners[i], line_corners[i + 1]);
        let mut new_dots: Vec<(i32, i32)> = Vec::new();
        if p1.0 == p2.0 {
            new_dots = (p1.1.min(p2.1)..=p1.1.max(p2.1))
                .map(|x| (p1.0, x))
                .collect();
        } else {
            new_dots = (p1.0.min(p2.0)..=p1.0.max(p2.0))
                .map(|x| (x, p1.1))
                .collect();
        }
        occupied_points.append(&mut new_dots);
    }

    return occupied_points;
}

fn part1(lines: Vec<Vec<(i32, i32)>>) -> usize {
    let mut rock_positions: Vec<(i32, i32)> = lines
        .iter()
        .map(|line| generate_line(line.clone()))
        .flatten()
        .collect();

    rock_positions.sort_by(|a, b| b.1.cmp(&a.1));
    let lowest_rock_y = rock_positions[0].1;
    let rock_set:HashSet<(i32,i32)> = HashSet::from_iter(rock_positions.clone());
    rock_positions.clear();
    let sand_start = (500, 0);
    let mut sand_positions: HashSet<(i32, i32)> = HashSet::new();
    let mut active_sand = (500, 0);
    let mut sand_falls = true;

    let sand_deltas: Vec<(i32, i32)> = vec![(0, 1), (1.neg(), 1), (1, 1)];
    while sand_falls {
        // Let active sand try to take step
        let position_choices: Vec<(i32, i32)> = sand_deltas
            .iter()
            .map(|x| (x.0 + active_sand.0, x.1 + active_sand.1))
            .filter(|x| x.0 > 0 && x.1 > 0)
            .collect();
        let possible_positions: Vec<&(i32, i32)> = position_choices
            .iter()
            .filter(|x| !rock_set.contains(&x) && !sand_positions.contains(&x))
            .collect();
        if possible_positions.len() == 0 {
            sand_positions.insert(active_sand.clone());
            active_sand = sand_start.clone();
        } else {
            active_sand = **possible_positions.get(0).unwrap();
            if active_sand.1 == lowest_rock_y {
                sand_falls = false;
            }
        }
    }
    return sand_positions.len();
}


fn part2(lines: Vec<Vec<(i32, i32)>>) -> usize {
    let mut rock_positions: Vec<(i32, i32)> = lines
        .iter()
        .map(|line| generate_line(line.clone()))
        .flatten()
        .collect();
    rock_positions.sort_by(|a, b| b.1.cmp(&a.1));
    let rock_set:HashSet<(i32,i32)> = HashSet::from_iter(rock_positions.clone());
    let lowest_rock_y = rock_positions[0].1 +2;

    let sand_start = (500, 0);
    let mut sand_positions: HashSet<(i32, i32)> = HashSet::new();
    let mut active_sand = (500, 0);
    let mut sand_falls = true;

    let sand_deltas: Vec<(i32, i32)> = vec![(0, 1), (1.neg(), 1), (1, 1)];

    while sand_falls {
        // Let active sand try to take step
        let position_choices: Vec<(i32, i32)> = sand_deltas
            .iter()
            .map(|x| (x.0 + active_sand.0, x.1 + active_sand.1))
            .filter(|x| x.1 < lowest_rock_y && x.1 > 0)
            .collect();
        let possible_positions: Vec<&(i32, i32)> = position_choices
            .iter()
            .filter(|x| !rock_set.contains(&x) && !sand_positions.contains(&x))
            .collect();
        if possible_positions.len() == 0 {
            sand_positions.insert(active_sand.clone());
            if active_sand == sand_start{
                sand_falls = false;
            }
            active_sand = sand_start.clone();
 
        } else {
            active_sand = **possible_positions.get(0).unwrap();
        }
    }
    return sand_positions.len();
}

fn main() {
    let test_input = fs::read_to_string(TEST_PATH).expect("Should have been able to read the file");
    let real_input = fs::read_to_string(REAL_PATH).expect("Should have been able to read the file");
    let test_lines: Vec<Vec<(i32, i32)>> = test_input
        .split("\n")
        .map(|x| {
            x.split("->")
                .map(|y| {
                    (
                        y.split(",").collect::<Vec<&str>>()[0]
                            .trim()
                            .parse()
                            .unwrap(),
                        y.split(",").collect::<Vec<&str>>()[1]
                            .trim()
                            .parse()
                            .unwrap(),
                    )
                })
                .collect()
        })
        .collect();

    let real_lines: Vec<Vec<(i32, i32)>> = real_input
        .split("\n")
        .map(|x| {
            x.split("->")
                .map(|y| {
                    (
                        y.split(",").collect::<Vec<&str>>()[0]
                            .trim()
                            .parse()
                            .unwrap(),
                        y.split(",").collect::<Vec<&str>>()[1]
                            .trim()
                            .parse()
                            .unwrap(),
                    )
                })
                .collect()
        })
        .collect();

    println!("PART 1");
    println!("Test: {}", part1(test_lines.clone()));
    println!("Real: {}\n", part1(real_lines.clone()));

    println!("PART 2");
    println!("Test: {}", part2(test_lines));
    println!("Real: {}", part2(real_lines));
}
