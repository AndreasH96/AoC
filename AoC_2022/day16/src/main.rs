use day16::Valve;
use lazy_static::lazy_static; // 1.4.0
use parking_lot::Mutex;
use rayon::prelude::*;
use std::collections::{HashMap, HashSet};
use std::fs;
use text_io::scan;
const TEST_PATH: &'static str = "./src/test_input.txt";
const REAL_PATH: &'static str = "./src/real_input.txt";

lazy_static! {
    static ref SCORE: Mutex<i32> = Mutex::new(0);
}

fn update_score(new: i32) {
    if new > *SCORE.lock() {
        *SCORE.lock() = new;
    }
}

fn reset_score() {
    *SCORE.lock() = 0;
}

fn extract_valve(in_str: &str) -> Valve {
    let name: String;
    let flow: i32;
    let edges_str: String;
    let parsed_str = in_str
        .replace("tunnels", "tunnel")
        .replace("valves", "valve")
        .replace("leads", "lead");
    scan!(parsed_str.bytes() => "Valve {} has flow rate={}; tunnel lead to valve {}\n",name,flow,edges_str);
    let edges = edges_str.split(",").map(|x| x.trim().to_string()).collect();
    let valve = Valve::new(name, flow, edges, false);
    return valve;
}

// https://en.wikipedia.org/wiki/Floyd%E2%80%93Warshall_algorithm
fn build_distance_map_floyd_warshall(
    valves: &HashMap<String, Valve>,
) -> HashMap<(String, String), i32> {
    let mut distances: HashMap<(String, String), i32> = HashMap::new();
    for valve in valves.keys() {
        let edges = &valves.get(valve).unwrap().edges;
        for edge in edges {
            distances.insert((valve.clone(), edge.clone()), 1);
        }
    }

    for k in valves.keys() {
        for i in valves.keys() {
            for j in valves.keys() {
                let key_ik = (i.to_string(), k.to_string());
                let d_ik = if distances.contains_key(&key_ik) {
                    *distances.get(&key_ik).unwrap()
                } else {
                    (valves.len() * 10) as i32
                };

                let key_kj = (k.to_string(), j.to_string());
                let d_kj = if distances.contains_key(&key_kj) {
                    *distances.get(&key_kj).unwrap()
                } else {
                    (valves.len() * 10) as i32
                };

                let key_ij = (i.to_string(), j.to_string());
                let d_ij = if distances.contains_key(&key_ij) {
                    *distances.get(&key_ij).unwrap()
                } else {
                    (valves.len() * 10) as i32
                };
                if d_ij > (d_ik + d_kj) {
                    if distances.contains_key(&key_ij) {
                        distances.remove(&key_ij);
                    }
                    distances.insert(key_ij, d_ik + d_kj);
                }
            }
        }
    }
    // Remove valves which have 0 flow
    let mut to_remove: Vec<(String, String)> = Vec::new();
    for d in distances.clone() {
        if valves.get(&d.0 .1).unwrap().flow_rate == 0 {
            to_remove.insert(0, d.0)
        }
    }
    for r in to_remove {
        distances.remove(&r);
    }

    return distances;
}

fn format_distances(
    valves: HashMap<String, Valve>,
    distances: HashMap<(String, String), i32>,
) -> HashMap<String, HashSet<(String, i32)>> {
    let mut result: HashMap<String, HashSet<(String, i32)>> = HashMap::new();
    for valve in valves.keys() {
        let mut f: HashSet<(String, i32)> = HashSet::new();
        let valve_distances: HashMap<&(String, String), &i32> =
            distances.iter().filter(|x| x.0 .0 == *valve).collect();

        for v_dist in valve_distances {
            f.insert((v_dist.0 .1.clone(), *v_dist.1));
        }
        result.insert(valve.to_string(), f);
    }
    return result;
}

fn search(
    distances: &HashMap<String, HashSet<(String, i32)>>,
    valves: &HashMap<String, Valve>,
    opened: &HashSet<String>,
    current_pos: &String,
    current_score: i32,
    time: i32,
    max_time: i32,
    part2: bool,
) {
    update_score(current_score);

    let distance = distances.get(current_pos).unwrap();

    distance.par_iter().for_each(|pos| {
        if (!opened.contains(&pos.0)) && ((time + pos.1) < max_time) {
            let mut new_opened = opened.clone();
            new_opened.insert(pos.0.clone());
            let fl = valves.get(&pos.0).unwrap().flow_rate;
            let new_score = current_score + (max_time - time - pos.1) * fl;

            search(
                distances,
                valves,
                &new_opened,
                &pos.0,
                new_score,
                time + pos.1 + 1,
                max_time,
                part2,
            );
        }
    });
    if part2 {
        search(
            distances,
            valves,
            opened,
            &"AA".to_string(),
            current_score,
            0,
            max_time,
            false,
        );
    }
}

fn part2(raw_str: String) -> i32 {
    let valves: HashMap<String, Valve> = HashMap::from_iter(
        raw_str
            .lines()
            .map(|line| (extract_valve(line).name, extract_valve(line)))
            .into_iter(),
    );
    let open_valves: HashSet<String> = HashSet::new();
    let current_pos: String = "AA".to_string();

    let distances: HashMap<String, HashSet<(String, i32)>> =
        format_distances(valves.clone(), build_distance_map_floyd_warshall(&valves));

    search(
        &distances,
        &valves,
        &open_valves,
        &current_pos,
        0,
        0,
        25,
        true,
    );
    let res = SCORE.lock().clone();
    reset_score();
    return res;
}

fn part1(raw_str: String) -> i32 {
    let valves: HashMap<String, Valve> = HashMap::from_iter(
        raw_str
            .lines()
            .map(|line| (extract_valve(line).name, extract_valve(line)))
            .into_iter(),
    );
    let open_valves: HashSet<String> = HashSet::new();
    let current_pos: String = "AA".to_string();

    let distances: HashMap<String, HashSet<(String, i32)>> =
        format_distances(valves.clone(), build_distance_map_floyd_warshall(&valves));

    search(
        &distances,
        &valves,
        &open_valves,
        &current_pos,
        0,
        0,
        29,
        false,
    );
    let res = SCORE.lock().clone();
    reset_score();
    return res;
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
