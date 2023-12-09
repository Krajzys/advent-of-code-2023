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
    // fastest for now
    let mut curr_step: usize = 0;
    for go in steps_list.chars().cycle() {
        for key in &mut curr_nodes {
            let paths = map.get(key).expect("key should be L or R");
            *key = match go {
                'L' => paths.0,
                'R' => paths.1,
                _ => panic!("Wrong character")
            };
        }
        if curr_step % 10000000 == 0 {
            println!("curr step {curr_step}... [time taken {}ms]", time.elapsed().as_millis());
        }
        
        curr_step += 1;
        if curr_nodes.iter().all(|key| key.ends_with('Z')) {
            break;
        }
    }

    // idea v3 - find a loop (such a sequence that after some number of moves we are at the same node and the same step position) for all the starting positions
    // find the least common multiple of all the loop lengths 

    // interesting idea but slow
    // let mut max_iters: Vec<usize> = Vec::from_iter(std::iter::repeat(0).take(curr_nodes.len()));
    // while !max_iters.iter().all(|e| e == max_iters.get(0).unwrap()) || max_iters.iter().all(|e| *e == 0) {
    //     let mut curr_map: usize = 0;
    //     for key in &mut curr_nodes {
    //         let mut curr_max_iter = max_iters[curr_map % max_iters.len()];
    //         // println!("curr max iter {} {} {:?}", curr_max_iter, curr_map % max_iters.len(), max_iters);
    //         let mut go = steps_list.chars().cycle().skip(curr_max_iter);
    //         while !key.ends_with('Z') || max_iters.iter().max().unwrap() > &curr_max_iter {
    //             let paths = map.get(key).unwrap();
    //             *key = match go.next().unwrap() {
    //                 'L' => paths.0,
    //                 'R' => paths.1,
    //                 _ => panic!("Wrong character")
    //             };
    //             curr_max_iter += 1;
    //             if curr_max_iter % 10000000 == 0 {
    //                 println!("curr map {curr_map} curr step {curr_max_iter}... [time taken {}ms]", time.elapsed().as_millis());
    //             }
    //         }
    //         max_iters[curr_map] = curr_max_iter;
    //         curr_map += 1;
    //         // println!("first stop for {curr_map} {key} {} [time taken {}ms]", curr_max_iter, time.elapsed().as_millis());
    //     }
    //     // println!("after one loop {:?}", max_iters)
    // }

    // interesting idea v2
    // let mut max_iters: Vec<usize> = Vec::from_iter(std::iter::repeat(0).take(curr_nodes.len()));
    // let mut curr_map: usize = 0;
    // while curr_map < max_iters.len() {
    //     let key = &mut curr_nodes[curr_map];
    //     let mut curr_max_iter = max_iters[curr_map % max_iters.len()];
    //     // println!("curr max iter {} {} {:?}", curr_max_iter, curr_map % max_iters.len(), max_iters);
    //     let mut go = steps_list.chars().cycle().skip(curr_max_iter);
    //     while !key.ends_with('Z') || max_iters.iter().max().unwrap() > &curr_max_iter {
    //         let paths = map.get(key).unwrap();
    //         *key = match go.next().unwrap() {
    //             'L' => paths.0,
    //             'R' => paths.1,
    //             _ => panic!("Wrong character")
    //         };
    //         curr_max_iter += 1;
    //         if curr_max_iter % 10000000 == 0 {
    //             println!("curr map {curr_map} curr step {curr_max_iter}... [time taken {}ms]", time.elapsed().as_millis());
    //         }
    //     }
    //     max_iters[curr_map] = curr_max_iter;
    //     if curr_map > 0 && max_iters[curr_map-1] != curr_max_iter {
    //         curr_map -= 1;
    //     } else {
    //         curr_map += 1;
    //     }
    //     println!("first stop for {curr_map} {key} {} [time taken {}ms]", curr_max_iter, time.elapsed().as_millis());
    // };
        
    print!("the result is {} [time taken {}ms]", curr_step, time.elapsed().as_millis());
}