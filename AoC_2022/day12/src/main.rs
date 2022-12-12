use std::{
    collections::{HashMap, HashSet},
    fs,
    ops::Neg,
};

const ALPHABET: &str = " abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
const TEST_PATH: &'static str = "./src/test_input.txt";
const REAL_PATH: &'static str = "./src/real_input.txt";

fn find_goal(matrix: Vec<Vec<i8>>) -> (usize, usize) {
    for (i, row) in matrix.iter().enumerate() {
        for (j, val) in row.iter().enumerate() {
            if *val == 31 {
                return (i, j);
            }
        }
    }
    return (0, 0);
}

fn get_possible_neighbors(
    matrix: Vec<Vec<i8>>,
    current_node: (usize, usize, usize),
    goal: (usize, usize),
) -> Vec<(usize, usize, usize)> {
    let movements: [[i32; 2]; 4] = [[1, 0], [1.neg(), 0], [0, 1], [0, 1.neg()]];
    let mut possible_neighbors: Vec<(usize, usize, usize)> = Vec::new();

    for mov in movements {
        let mut new_pos = (
            current_node.0 as i32 + mov[0],
            current_node.1 as i32 + mov[1],
            0,
        );
        if new_pos.0 >= 0
            && new_pos.0 < (matrix.len() as i32)
            && new_pos.1 >= 0
            && (new_pos.1 < matrix[0].len() as i32)
        {
            if matrix[new_pos.0 as usize][new_pos.1 as usize]
                <= matrix[current_node.0][current_node.1] + 1
            {
                new_pos.2 = f(
                    (new_pos.0 as usize, new_pos.1 as usize, current_node.2 + 1),
                    goal,
                );

                possible_neighbors.insert(
                    0,
                    (new_pos.0 as usize, new_pos.1 as usize, new_pos.2 as usize),
                );
            }
        }
    }
    return possible_neighbors;
}

fn f(x: (usize, usize, usize), y: (usize, usize)) -> usize {
    return x.2 + ((y.0 as i32 - x.0 as i32).abs() + (y.1 as i32 - x.1 as i32).abs()) as usize;
}

fn solve(start: (usize, usize), in_mat: Vec<Vec<i8>>) -> usize {
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut path: HashMap<(usize, usize), (usize, usize)> = HashMap::new();
    let mut costs: HashMap<(usize, usize), usize> = HashMap::new();
    let mut queue: Vec<(usize, usize, usize)> = Vec::new();
    let mut matrix = in_mat.clone();

    let goal = find_goal(matrix.clone());

    visited.insert(start);
    path.insert(start, start);
    costs.insert(start, 0);
    matrix[goal.0][goal.1] = ALPHABET.find("z").unwrap() as i8;
    queue.insert(0, (start.0, start.1, 0));
    while !queue.is_empty() {
        queue.sort_by(|a, b| b.2.cmp(&a.2));

        let q = queue.pop().unwrap();

        let neighbors = get_possible_neighbors(matrix.clone(), q, goal);
        for neighbor in neighbors {
            if (neighbor.0, neighbor.1) == goal {
                path.insert((neighbor.0, neighbor.1), (q.0, q.1));
                queue.clear();
                break;
            } else {
                if visited.contains(&(neighbor.0, neighbor.1)) {
                    if costs.get(&(neighbor.0, neighbor.1)).unwrap() > &neighbor.2 {
                        costs.remove(&(neighbor.0, neighbor.1));
                        costs.insert((neighbor.0, neighbor.1), neighbor.2);
                        path.remove(&(neighbor.0, neighbor.1));
                        path.insert((neighbor.0, neighbor.1), (q.0, q.1));
                        queue.insert(queue.len(), neighbor);
                    }
                } else {
                    visited.insert((neighbor.0, neighbor.1));
                    costs.insert((neighbor.0, neighbor.1), neighbor.2);
                    path.insert((neighbor.0, neighbor.1), (q.0, q.1));
                    queue.insert(queue.len(), neighbor);
                }
            }
        }
    }

    let mut c = goal.clone();

    let mut test: Vec<(usize, usize)> = Vec::new();

    if path.contains_key(&(c.0, c.1)) {
        while path.get(&(c.0, c.1)).unwrap() != &c {
            test.insert(0, c.clone());
            c = *path.get(&(c.0, c.1)).unwrap();
        }
        return test.len();
    }
    return usize::MAX;
}

fn find_start_positions(matrix: Vec<Vec<i8>>) -> Vec<(usize, usize)> {
    let mut result: Vec<(usize, usize)> = Vec::new();

    for (i, x) in matrix.iter().enumerate() {
        if x[0] == 1 {
            result.insert(0, (i, 0));
        }
    }
    return result;
}

fn part1(matrix: Vec<Vec<i8>>) -> usize {
    let start_position = (0, 0);
    let res = solve(start_position, matrix.clone());
    return res;
}

fn part2(matrix: Vec<Vec<i8>>) -> usize {
    let mut start_positions: Vec<(usize, usize)> = find_start_positions(matrix.clone());
    start_positions = start_positions
        .iter()
        .filter(|x| get_possible_neighbors(matrix.clone(), (x.0, x.1, 0), (0, 0)).len() > 0)
        .map(|x| *x)
        .collect::<Vec<(usize, usize)>>();

    let mut min_start = usize::MAX;

    for start in start_positions {
        let res = solve(start, matrix.clone());
        if res < min_start {
            min_start = res;
        }
    }
    return min_start;
}

fn main() {
    let test_data = fs::read_to_string(TEST_PATH).expect("Should have been able to read the file");
    let real_data = fs::read_to_string(REAL_PATH).expect("Should have been able to read the file");
    let test_matrix: Vec<Vec<i8>> = test_data
        .lines()
        .map(|line| {
            line.split("")
                .map(|x| ALPHABET.find(x).unwrap() as i8)
                .filter(|x| *x != 0)
                .collect()
        })
        .collect();

    let real_matrix: Vec<Vec<i8>> = real_data
        .lines()
        .map(|line| {
            line.split("")
                .map(|x| ALPHABET.find(x).unwrap() as i8)
                .filter(|x| *x != 0)
                .collect()
        })
        .collect();

    println!("PART 1");
    println!("Test data: {}", part1(test_matrix.clone()));
    println!("Real data: {}\n", part1(real_matrix.clone()));

    println!("PART 2");
    println!("Test data: {}", part2(test_matrix.clone()));
    println!("Real data: {}", part2(real_matrix.clone()));
}
