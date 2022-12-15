use itertools::any;
use std::collections::{HashMap, HashSet};
use std::fs;
use text_io::scan;
const TEST_PATH: &'static str = "./src/test_input.txt";
const REAL_PATH: &'static str = "./src/real_input.txt";

fn extract_sensors_and_beacons(in_str: &str) -> Vec<(i32, i32)> {
    let s_x: i32;
    let s_y: i32;
    let b_x: i32;
    let b_y: i32;
    scan!(in_str.bytes() => "Sensor at x={}, y={}: closest beacon is at x={}, y={}",s_x,s_y,b_x,b_y);
    return vec![(s_x, s_y), (b_x, b_y)];
}

fn manhattan_dist(s: (i32, i32), b: (i32, i32)) -> i32 {
    return (s.0 - b.0).abs() + (s.1 - b.1).abs();
}

fn part1(raw_input: &str, y: i32) -> i32 {
    let test = raw_input.lines().map(|x| extract_sensors_and_beacons(x));

    let mut sensor_map: HashMap<(i32, i32), (i32, i32, i32)> = HashMap::new();
    let mut beacon_set: HashSet<(i32, i32)> = HashSet::new();
    test.for_each(|r| {
        beacon_set.insert(r[1]);
        sensor_map.insert(r[0], (r[1].0, r[1].1, manhattan_dist(r[0], r[1])));
    });

    let min_x: i32 = sensor_map
        .keys()
        .map(|x| x.0 - sensor_map.get(x).unwrap().2)
        .min()
        .unwrap();
    let max_x: i32 = sensor_map
        .keys()
        .map(|x| x.0 + sensor_map.get(x).unwrap().2)
        .max()
        .unwrap();
    let mut result = 0;
    for x in min_x..=max_x {
        if beacon_set.contains(&(x, y)) {
            continue;
        } else {
            for s in sensor_map.keys() {
                if manhattan_dist(*s, (x, y)) <= sensor_map.get(s).unwrap().2 {
                    result += 1;
                    break;
                }
            }
        }
    }
    return result;
}


fn part2(raw_input: &str, max_min: (i32, i32)) -> i128 {
    let test = raw_input.lines().map(|x| extract_sensors_and_beacons(x));

    let mut sensor_map: HashMap<(i32, i32), (i32, i32, i32)> = HashMap::new();
    let mut beacon_set: HashSet<(i32, i32)> = HashSet::new();
    test.for_each(|r| {
        beacon_set.insert(r[1]);
        sensor_map.insert(r[0], (r[1].0, r[1].1, manhattan_dist(r[0], r[1])));
    });

    for y in max_min.0..=max_min.1 {

        let mut edges: Vec<(i32,i32)> = Vec::new();
        for s in sensor_map.keys() { 
            if (s.1 - y).abs() <= sensor_map.get(&s).unwrap().2 {
                let left_edge = s.0 - sensor_map.get(&s).unwrap().2 + (s.1 - y).abs();
                let right_edge = s.0 + (sensor_map.get(&s).unwrap().2 - (s.1 - y).abs()).abs();
                edges.insert(0, (max_min.0.max(left_edge),max_min.1.min(right_edge)));
            }
        }  
        edges.sort_by(|a,b| a.1.cmp(&b.1));

        for edge in edges.clone(){
            
            if (!any(edges.clone(), |x| (x.0..=x.1).contains(&(edge.0-1))))  && edge.0>0{
                return ((edge.0 as i128-1) *4000000) +y as i128 ;
            }
        }


    }
    return 0;
}

fn main() {
    let test_input = fs::read_to_string(TEST_PATH).expect("Should have been able to read the file");
    let real_input = fs::read_to_string(REAL_PATH).expect("Should have been able to read the file");

    println!("PART 1");
    use std::time::Instant;
    let now = Instant::now();
    println!("Test: {}", part1(&test_input, 10));
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
    let now = Instant::now();
    println!("Real: {}", part1(&real_input,2000000));
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}\n", elapsed);
    let now = Instant::now();
    //
    println!("PART 2");
    println!("Test: {:?}", part2(&test_input, (0, 20)));
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
    let now = Instant::now();
    println!("Real: {}",  part2(&real_input, (0, 4000000)));
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
    //let now = Instant::now();
}

// Create two sets, one for sensor positions and one for beacon positions *
// Create map with distance between sensors and nearest beacon, key = sensor pos, val = beacon pos and manhattan distance pos *

// Find X range of beacon ranges for sensors, extract min X and max X from this
// Go to row Y, loop through X positions, check if it's distance to any Sensor is less than their beacon
// If less than or equal, cannot be placed there
// Else, can be placed there
