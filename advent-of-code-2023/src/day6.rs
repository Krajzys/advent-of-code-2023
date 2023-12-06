use std::time::Instant;

use crate::util::read_file;

pub fn day6_1() {
    let file = read_file("inputs/day6.txt");
    let games: Vec<&str> = file.lines().collect();
    
    let mut times: Vec<usize> = Vec::new();
    let mut distances: Vec<usize> = Vec::new();
    for line in games.iter() {
        if line.starts_with("Time:") {
            let times_line: Vec<&str> = line.split(":").collect();
            let times_str = times_line[1];
            for val in times_str.trim().replace("  ", " ").split_whitespace() {
                times.push(val.parse().unwrap());
            };
        }
        if line.starts_with("Distance:") {
            let distance_line: Vec<&str> = line.split(":").collect();
            let distance_str = distance_line[1];
            for val in distance_str.trim().replace("  ", " ").split_whitespace() {
                distances.push(val.parse().unwrap());
            };
        }
    }

    let mut total: usize = 1;
    let time = Instant::now();
    for (time, distance) in core::iter::zip(times, distances) {
        let mut wins = 0;
        for pressed in 1..time {
            let curr_distance = pressed*(time-pressed);
            if curr_distance > distance {
                wins += 1;
            }
        }
        total *= wins;
    }

    print!("the result is {} [time taken {}us]", total, time.elapsed().as_micros());
}

pub fn day6_2() {
    let file = read_file("inputs/day6.txt");
    let games: Vec<&str> = file.lines().collect();
    
    let mut times: Vec<usize> = Vec::new();
    let mut distances: Vec<usize> = Vec::new();
    for line in games.iter() {
        if line.starts_with("Time:") {
            let times_line: Vec<&str> = line.split(":").collect();
            let times_str = times_line[1];
            times.push(times_str.trim().replace(" ", "").parse().unwrap());
        }
        if line.starts_with("Distance:") {
            let distance_line: Vec<&str> = line.split(":").collect();
            let distance_str = distance_line[1];
            distances.push(distance_str.trim().replace(" ", "").parse().unwrap());
        }
    }

    let mut total: usize = 1;
    let time = Instant::now();
    for (time, distance) in core::iter::zip(times, distances) {
        let mut wins = 0;
        for pressed in 1..time {
            let curr_distance = pressed*(time-pressed);
            if curr_distance > distance {
                wins = time-(pressed*2)+1;
                break;
            }
        }
        total *= wins;
    }

    print!("the result is {} [time taken {}ms]", total, time.elapsed().as_millis());
}
