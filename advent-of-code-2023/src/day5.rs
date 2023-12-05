use std::{collections::HashMap, time::Instant};

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
    for seed_range in seeds {
        // print!("seed {}", seed);
        println!("seed_range {}", seed_range.end-seed_range.start);
        for (i, seed) in seed_range.enumerate() {
            if i % 1000000 == 0 {
                println!("processing ... [{i}]");
            }   
            let mut seed_num = seed;
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
        }
        println!("time taken {}", now.elapsed().as_millis()/1000);
        // println!();
    }
    println!("time taken {}", now.elapsed().as_millis()/1000);
    print!("the result is {}", min_seed);
}
