use day19::Blueprint;

use rayon::prelude::{IntoParallelIterator, IntoParallelRefIterator, ParallelIterator};
use std::{collections::HashMap, fs};
use text_io::scan;
const TEST_PATH: &'static str = "./src/test_input.txt";
const REAL_PATH: &'static str = "./src/real_input.txt";

fn parse_input(input_string: String) -> Blueprint {
    let blueprint_id: usize;
    let ore_robot_cost: u32;
    let clay_robot_cost: u32;
    let obsidian_robot_cost_ore: u32;
    let obsidian_robot_cost_clay: u32;
    let geode_robot_cost_ore: u32;
    let geode_robot_cost_obsidian: u32;
    scan!(input_string.bytes() => "Blueprint {}: Each ore robot costs {} ore. Each clay robot costs {} ore. Each obsidian robot costs {} ore and {} clay. Each geode robot costs {} ore and {} obsidian.",
    blueprint_id,ore_robot_cost,clay_robot_cost,obsidian_robot_cost_ore,obsidian_robot_cost_clay,geode_robot_cost_ore,geode_robot_cost_obsidian);

    let mut price_list: HashMap<usize, HashMap<usize, u32>> = HashMap::new();

    let robot_costs = [
        (0, ore_robot_cost, 0, 0),
        (1, clay_robot_cost, 0, 0),
        (2, obsidian_robot_cost_ore, obsidian_robot_cost_clay, 0),
        (3, geode_robot_cost_ore, 0, geode_robot_cost_obsidian),
    ];
    for rc in robot_costs {
        let mut prices: HashMap<usize, u32> = HashMap::new();
        prices.insert(0, rc.1);
        prices.insert(1, rc.2);
        prices.insert(2, rc.3);
        prices.insert(3, 0);
        price_list.insert(rc.0, prices);
    }

    let blueprint: Blueprint = Blueprint::new(blueprint_id, price_list.clone());
    return blueprint;
}

fn step(
    time: u32,
    max_time: u32,
    current_bots: &Vec<u32>,
    current_materials: &Vec<u32>,
    bot_to_build: usize,
    bots_to_build: Vec<usize>,
    blueprint: &mut Blueprint,
) -> u32 {
    let mut new_bot = bot_to_build;
    for i in 0..3 {
        new_bot = if blueprint.price_list.get(&3).unwrap()[&i] <= current_materials[i] {
            3
        } else {
            bot_to_build
        };
    }

    let costs = blueprint.price_list.get(&new_bot).unwrap();
    let mut to_build: Vec<usize> = bots_to_build;
    let mut materials = current_materials.clone();
    let mut bots = current_bots.clone();

    if time >= max_time {
        return materials[3];
    }
    if time == max_time -1{
        return bots[3] + materials[3];
    }
    for i in 0..4 {
        if costs[&i] > 0 && bots[i] == 0 {
            return materials[3];
        }
    }

    let mut time_delta = 1 + materials
        .iter()
        .enumerate()
        .map(|m| {
            0.max(*costs.get(&m.0).unwrap() as i32 - *m.1 as i32) as u32
                / (bots.get(m.0).unwrap().max(&1))
                + if *bots.get(m.0).unwrap() != 0
                    && (0.max(*costs.get(&m.0).unwrap() as i32 - *m.1 as i32) as u32
                        % bots.get(m.0).unwrap())
                        != 0
                {
                    1
                } else {
                    0
                }
        })
        .max()
        .unwrap();

    time_delta = time_delta.min(max_time - time);
    let new_time = time_delta + time;
    for i in 0..4 {
        materials[i] += bots[i] * time_delta;
    }

    if new_time < max_time {
        for i in 0..3 {
            materials[i] -= costs[&i];
        }
    }
    
    bots[new_bot] += 1;
   
    if to_build.len() > 1 {
        to_build = to_build
            .iter()
            .filter(|x| {
                (bots.get(**x).unwrap() * (max_time - new_time)) + materials.get(**x).unwrap()
                    < ((max_time - new_time) * blueprint.max_costs.get(**x).unwrap())
                    || **x == 3
            })
            .map(|x| *x)
            .collect();
    }

    return to_build
        .iter()
        .map(|x| {
            step(
                new_time,
                max_time,
                &bots,
                &materials,
                *x,
                to_build.clone(),
                blueprint,
            )
        })
        .max()
        .unwrap();
}

fn part1(text_input: String) -> u32 {
    let blueprints: Vec<Blueprint> = text_input
        .lines()
        .map(|x| parse_input(x.to_string()))
        .collect();
    let time_limit = 24;
    let current_materials: Vec<u32> = vec![0, 0, 0, 0];
    let current_bots: Vec<u32> = vec![1, 0, 0, 0];

    let result: Vec<(u32, u32)> = blueprints
        .par_iter()
        .map(|x: &Blueprint| {
            let k = (0..2)
                .into_par_iter()
                .map(|i| {
                    step(
                        0,
                        time_limit,
                        &current_bots.clone(),
                        &current_materials.clone(),
                        i,
                        vec![0, 1, 2, 3],
                        &mut x.clone(),
                    )
                })
                .max()
                .unwrap();

            return (x.id as u32, k);
        })
        .collect();
    return result.iter().map(|x| x.0 * x.1).sum();
}
fn part2(text_input: String, test: bool) -> u32 {
    let mut blueprints: Vec<Blueprint> = text_input
        .lines()
        .map(|x| parse_input(x.to_string()))
        .collect();

    let time_limit = 32;
    let current_materials: Vec<u32> = vec![0, 0, 0, 0];
    let current_bots: Vec<u32> = vec![1, 0, 0, 0];

    blueprints = if test {
        blueprints
    } else {
        blueprints[0..3].to_vec()
    };

    let result: Vec<u32> = blueprints
        .par_iter()
        .map(|x: &Blueprint| {
            let k = (0..2)
                .into_par_iter()
                .map(|i| {
                    step(
                        0,
                        time_limit,
                        &current_bots.clone(),
                        &current_materials.clone(),
                        i,
                        vec![0, 1, 2, 3],
                        &mut x.clone(),
                    )
                })
                .max()
                .unwrap();
            return k;
        })
        .collect();
    if test {
        return result.get(0).unwrap() * result.get(1).unwrap();
    }
    return result.get(0).unwrap() * result.get(1).unwrap() * result.get(2).unwrap();
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
   /*  println!("Test: {:?}", part2(test_input, true));
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed); */
    let now = Instant::now();
    println!("Real: {}", part2(real_input, false));
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}