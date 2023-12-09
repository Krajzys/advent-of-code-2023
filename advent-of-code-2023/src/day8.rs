use std::{time::Instant, collections::HashMap};

use crate::util::read_file;

pub fn day8_1(filename: String) {
    let mut filename_l = filename;
    if filename_l.is_empty() {
        filename_l = String::from("inputs/day8.txt");
    }   
    let file = read_file(&filename_l);
    
    // Create steps_list and map to hold the map paths
    let mut steps_list = String::from("");
    let mut map: HashMap<&str, (&str, &str)> = HashMap::new();
    for (i, line) in file.lines().enumerate() {
        if i == 0 {
            steps_list = line.trim().to_string();
        }
        if i > 1 {
            let key_and_paths: Vec<&str> = line.split('=').collect();
            let key = key_and_paths[0].trim();
            let paths: Vec<&str> = key_and_paths[1].trim().trim_matches(|c| c == '(' || c == ')').split(',').collect();
            let paths_tuple = (paths[0].trim(), paths[1].trim());
            map.insert(key, paths_tuple.clone());
        }
    }

    // go thorugh the map according to the steps_list, starting with AAA, until we reach ZZZ
    let mut curr_node = "AAA";
    const END_NODE: &str = "ZZZ";
    let time = Instant::now();

    let mut curr_step: usize = 0;
    while curr_node != END_NODE {
        let paths = map.get(curr_node).unwrap();
        let go = steps_list.chars().nth(curr_step % steps_list.len()).unwrap();
        // print!("{curr_node}-[{go}]->");
        curr_node = match go {
            'L' => paths.0,
            'R' => paths.1,
            _ => panic!("Wrong character")
        };
        curr_step += 1;
    }

    print!("the result is {} [time taken {}ms]", curr_step, time.elapsed().as_millis());
}

pub fn day8_2(filename: String) {
    let mut filename_l = filename;
    if filename_l.is_empty() {
        filename_l = String::from("inputs/day8.txt");
    }   
    let file = read_file(&filename_l);
    
    // Create steps_list and map to hold the map paths
    let mut steps_list = String::from("");
    let mut map: HashMap<&str, (&str, &str)> = HashMap::new();
    for (i, line) in file.lines().enumerate() {
        if i == 0 {
            steps_list = line.trim().to_string();
        }
        if i > 1 {
            let key_and_paths: Vec<&str> = line.split('=').collect();
            let key = key_and_paths[0].trim();
            let paths: Vec<&str> = key_and_paths[1].trim().trim_matches(|c| c == '(' || c == ')').split(',').collect();
            let paths_tuple = (paths[0].trim(), paths[1].trim());
            map.insert(key, paths_tuple.clone());
        }
    }

    // go thorugh the map according to the steps_list, starting with all the nodes that end with A, until we reach ZZZ
    let mut curr_nodes: Vec<&str> = map.clone().into_keys().filter(|key| key.ends_with('A')).collect();
    let time = Instant::now();

    // find the least common multiple of all the loop lengths 
    let mut first_end_occur: Vec<usize> = Vec::new();
    for key in &mut curr_nodes {
        let mut curr_step: usize = 0;
        for go in steps_list.chars().cycle() {
            let paths = map.get(key).unwrap();
            *key = match go {
                'L' => paths.0,
                'R' => paths.1,
                _ => panic!("Wrong character")
            };
            curr_step += 1;
            if key.ends_with('Z') {
                first_end_occur.push(curr_step);
                break;
            }
        }
    }
    let mut current_score = first_end_occur[0];
    for i in 1..first_end_occur.len() {
        current_score = lcm(first_end_occur[i], current_score);
    }
        
    print!("the result is {} [time taken {}ms]", current_score, time.elapsed().as_millis());
}

fn lcm(first: usize, second: usize) -> usize {
    first * second / gcd(first, second)
}

fn gcd(first: usize, second: usize) -> usize {
    let mut bigger = first.max(second);
    let mut smaller = first.min(second);

    loop {
        let rem = bigger % smaller;
        if rem == 0 {
            return smaller;
        }
        (bigger, smaller) = (smaller, rem);
    }
}