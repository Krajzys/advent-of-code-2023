use std::{collections::HashMap, cmp::Ordering, time::Instant};

use crate::util::read_file;

pub fn day5_1() {
    let file = read_file("inputs/day5.txt");
    let games: Vec<&str> = file.lines().collect();
    
    let mut seeds: Vec<&str> = Vec::new();
    let mut maps: Vec<HashMap<std::ops::Range<i64>, i64>> = Vec::new();
    let mut map_started = false;
    let mut new_map: HashMap<std::ops::Range<i64>, i64> = HashMap::new();
    for (r, line) in games.iter().enumerate() {
        if line.starts_with("seeds:") {
            let seeds_local: Vec<&str> = line.split(": ").collect();
            let seeds_local: Vec<&str> = seeds_local[1].split(" ").collect();
            seeds = seeds_local.clone();
        }
        if line.ends_with("map:") {
            map_started = true;
            new_map = HashMap::new();
        }
        if map_started && !line.ends_with("map:") || r == games.len()-1 {
            if *line == "" {
                maps.push(new_map.clone());
                map_started = false;
            } else {
                let mut values: Vec<i64> = Vec::new();
                for num in line.split(' ') {
                    values.push(num.parse().unwrap());
                };

                let dst_start = values[0];
                let src_start = values[1];
                let range_len = values[2];

                new_map.insert(src_start..src_start+range_len, dst_start-src_start);
            }
            if r == games.len() - 1 {
                maps.push(new_map.clone());
                map_started = false;
            }
        }
    }

    let mut min_seed: i64 = i64::MAX;
    println!("seeds_len {}", seeds.len());
    for seed in seeds {
        // print!("seed {}", seed);
        let mut seed_num: i64 = seed.parse().unwrap();
        for (_, map) in maps.iter().enumerate() {
            for (range, offset) in map {
                if range.contains(&seed_num) {
                    seed_num += offset;
                    break;
                }
            }
            // print!("-[{}]->{}", i, seed_num);
        }
        if seed_num < min_seed {
            min_seed = seed_num;
        }
        // println!();
    }
    print!("the result is {}", min_seed);
}

pub fn day5_2() {
    let file = read_file("inputs/day5.txt");
    let games: Vec<&str> = file.lines().collect();
    
    // parse the input and fillout necessary structures
    let mut seeds: Vec<std::ops::Range<i64>> = Vec::new();
    let mut maps: Vec<HashMap<std::ops::Range<i64>, i64>> = Vec::new();
    let mut map_started = false;
    let mut new_map: HashMap<std::ops::Range<i64>, i64> = HashMap::new();
    for (r, line) in games.iter().enumerate() {
        if line.starts_with("seeds:") {
            let seeds_local: Vec<&str> = line.split(": ").collect();
            let seeds_local: Vec<&str> = seeds_local[1].split(" ").collect();
            let mut range_start: i64 = 0;
            for (i, val) in seeds_local.iter().enumerate() {
                let num = val.parse().unwrap();
                if i % 2 == 0 {
                    range_start = num;
                } else {
                    seeds.push(range_start..range_start+num);
                }
            }
        }
        if line.ends_with("map:") {
            map_started = true;
            new_map = HashMap::new();
        }
        if map_started && !line.ends_with("map:") || r == games.len()-1 {
            if *line == "" {
                maps.push(new_map.clone());
                map_started = false;
            } else {
                let mut values: Vec<i64> = Vec::new();
                for num in line.split(' ') {
                    values.push(num.parse().unwrap());
                };

                let dst_start = values[0];
                let src_start = values[1];
                let range_len = values[2];

                new_map.insert(src_start..src_start+range_len, dst_start-src_start);
            }
            if r == games.len() - 1 {
                maps.push(new_map.clone());
                map_started = false;
            }
        }
    }

    let mut min_seed: i64 = i64::MAX;
    let now = Instant::now();
    // for each input range find the lowest value we can from that range get using a recursive function
    for seed_range in seeds {
        let temp_val = get_min_from_range(seed_range.clone(), &maps, 0);
        if temp_val < min_seed {
            min_seed = temp_val;
        }
    }
    print!("the result is {} [time taken {}ms]", min_seed, now.elapsed().as_millis());
}

fn get_min_from_range(range: std::ops::Range<i64>, maps: &Vec<HashMap<std::ops::Range<i64>, i64>>, index: usize) -> i64 {
    let mut start = range.start;
    let end = range.end;
    if index >= maps.len() {
        return range.start
    }
    let mut val = i64::MAX;
    let map = &maps[index];
    while start < end {
        let mut matched = false;
        let mut min_start = i64::MAX;
        // check if our start falls into one of the ranges if yes we pass the trimmed range to the next map
        for (map_range, offset) in map {
            if map_range.start < min_start && map_range.start > start {
                min_start = map_range.start;
            }
            // if the start falls into one of the ranges, we create a new range that start from start+offset and ends on the end of that range and pass it to the next step
            if map_range.contains(&start) {
                matched = true;
                let new_end = match map_range.end.cmp(&end) {
                    Ordering::Less => map_range.end+offset,
                    Ordering::Equal | Ordering::Greater => end+offset 
                };
                let temp_val = get_min_from_range(start+offset..new_end, maps, index+1);
                // if the end of the range was smaller than the end of our input range we increase the start and continue the loop
                if map_range.end.cmp(&end) == Ordering::Less {
                    start = map_range.end;
                } else {
                    start = end;
                }
                if temp_val < val {
                    val = temp_val;
                }
            }
        }
        // if we didn't find any range that our start could fall into we check if our start is smaller than min_start
        // if yes then we split the range into two ranges, one that starts with start and ends with min_start and pass it unchanged to next map
        // then we pass the range that starts with min_start and ends with end again to the current map
        // if on the other hand out start is bigger than min start we just pass the whole range unchanged to the next map
        if !matched {
            if start < min_start {
                let temp_val = get_min_from_range(start..min_start, maps, index+1);
                if temp_val < val {
                    val = temp_val;
                }
                let temp_val = get_min_from_range(min_start..end, maps, index);
                if temp_val < val {
                    val = temp_val;
                }
                break;
            } else {
                let temp_val = get_min_from_range(range.clone(), maps, index+1);
                if temp_val < val {
                    val = temp_val;
                }
                break;
            }
        }
    }
    val
}
