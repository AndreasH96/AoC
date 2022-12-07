mod lib;
use lib::{Folder, FolderFile};
use std::{collections::HashMap, fs, str};
const TEST_PATH: &'static str = "./src/test_input.txt";
const REAL_PATH: &'static str = "./src/real_input.txt";

fn create_folder_structure(raw_data: &str) -> HashMap<String, Folder> {
    let mut path_commands: Vec<Vec<&str>> = raw_data
        .split("$ cd")
        .map(|x| {
            x.split("\n")
                .map(|y| y.trim())
                .filter(|r| r.len() > 0 && !r.contains("$ ls"))
                .collect()
        })
        .collect();
    path_commands.remove(0);

    let mut current_path: Vec<String> = vec![];
    let mut folders: HashMap<String, Folder> = HashMap::new();

    for commands in path_commands {
        if commands[0] == ".." {
            current_path.pop();
        } else {
            current_path.insert(current_path.len(), (&commands[0]).to_string());

            let sub_folders: Vec<String> = commands
                .iter()
                .filter(|x| x.contains("dir"))
                .map(|x| vec![current_path.join(":"), x[4..].to_string()].join(":"))
                .collect();

            let file_strings = commands
                .iter()
                .filter(|x| (!x.contains("dir")) && x.contains(" "));

            let files: Vec<FolderFile> = file_strings
                .map(|f| FolderFile::new(f.to_string()))
                .collect();
            folders.insert(
                current_path.join(":"),
                Folder::new(current_path.join(":"), sub_folders, files),
            );
        }
    }
    return folders;
}

fn calc_folder_sizes(raw_data: &str) -> Vec<(String, usize)> {

    let folders = create_folder_structure(raw_data);

    let sizes: Vec<(String, usize)> = folders
        .iter()
        .map(|x| (x.0.clone(), x.1.get_size_recur(&folders)))
        .collect();

    return sizes;
}

fn solve_part1(raw_data: &str) -> usize {
    return calc_folder_sizes(&raw_data)
        .iter()
        .filter(|x| x.1 <= 100000)
        .map(|x| x.1)
        .sum();
}

fn solve_part2(raw_data: &str) -> (String, usize) {
    let goal = 40000000;
    let sizes = calc_folder_sizes(raw_data);
    let total_space_used: usize = sizes.clone().iter().map(|x| x.1).max().unwrap();
    let mut candidates: Vec<&(String, usize)> = sizes
        .iter()
        .filter(|x| (total_space_used - x.1) < goal)
        .collect();
    candidates.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

    return candidates[0].clone();
}

fn main() {
    let test_data = fs::read_to_string(TEST_PATH).expect("Should have been able to read the file");
    let real_data = fs::read_to_string(REAL_PATH).expect("Should have been able to read the file");

    println!("PART 1");
    println!("Test: {:?}", solve_part1(&test_data));
    println!("Real: {:?}", solve_part1(&real_data));

    println!("PART 2");
    println!("Test: {:?}", solve_part2(&test_data));
    println!("Real: {:?}", solve_part2(&real_data));
}
