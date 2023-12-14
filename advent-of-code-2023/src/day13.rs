use std::{time::Instant, iter::zip, collections::HashSet};

use crate::util::read_file;

pub fn day13_1(filename: String) {
    let mut filename_l = filename;
    if filename_l.is_empty() {
        filename_l = String::from("inputs/day13.txt");
    }   
    let file = read_file(&filename_l);
    let mut mirror_maps: Vec<Vec<String>> = Vec::new();
    
    let time = Instant::now();
    let mut single_map: Vec<String> = Vec::new();
    // read file into vector
    for (_, line) in file.lines().enumerate() {
        if line.trim().is_empty() {
            mirror_maps.push(single_map.clone());
            single_map.clear();
        } else {
            single_map.push(String::from(line));
        }
    }
    if !single_map.is_empty() {
        mirror_maps.push(single_map.clone());
    }
    
    let mut result = 0;
    for (_, map) in mirror_maps.iter().enumerate() {
        // try to find matching rows
        match find_reflection_line(map) {
            None => {
                // try to find matching columns
                let cols = transpose_string_vector(map);
                match find_reflection_line(&cols) {
                    None => panic!("OoOoo not too good :/"),
                    Some(line) => {
                        result += line.1;
                    }
                }
            },
            Some(line) => {
                result += line.1*100;
            }
        }
    }

    print!("the result is {} [time taken {}ms]", result, time.elapsed().as_millis());
}

fn transpose_string_vector(vector: &Vec<String>) -> Vec<String> {
    let col_count = vector[0].len();
    let row_count = vector.len();
    let mut cols: Vec<String> = Vec::new();
    for col in 0..col_count {
        let mut new_col = String::new();
        for row in 0..row_count {
            new_col.push(vector[row].chars().nth(col).unwrap());
        }
        cols.push(new_col);
    }
    cols
}

fn find_reflection_line(map: &Vec<String>) -> Option<(usize, usize)> {
    let mut start = 0 as usize;
    let mut stop = map.len() - 1;
    let mut match_start = 0 as usize;
    let mut started = false;

    while start < stop {
        if map[start] == map[stop] {
            if !started {
                started = true;
                match_start = start;
            }
        } else if started {
            started = false;
            start = match_start;
            stop = map.len()-1;
        }
        if started {
            stop -= 1;
        }
        start += 1;
    }
    if !started || start == stop { // we didn't find match assuming the last line is the last one mirrored
        start = 0;
        stop = map.len() - 1;
        match_start = 0;
        started = false;
        while start < stop {
            if map[start] == map[stop] {
                if !started {
                    started = true;
                    match_start = stop;
                }
            } else if started {
                started = false;
                start = 0;
                stop = match_start;
            }
            if started {
                start += 1;
            }
            stop -= 1;
        }
        if !started {
            return None;
        } else {
            return Some((stop, stop + 1));
        }
    } else {
        return Some((start - 1, start));
    }
}

pub fn day13_2(filename: String) {
    let mut filename_l = filename;
    if filename_l.is_empty() {
        filename_l = String::from("inputs/day13.txt");
    }   
    let file = read_file(&filename_l);
    let mut mirror_maps: Vec<Vec<String>> = Vec::new();
    
    let time = Instant::now();
    let mut single_map: Vec<String> = Vec::new();
    // read file into vector
    for (_, line) in file.lines().enumerate() {
        if line.trim().is_empty() {
            mirror_maps.push(single_map.clone());
            single_map.clear();
        } else {
            single_map.push(String::from(line));
        }
    }
    if !single_map.is_empty() {
        mirror_maps.push(single_map.clone());
    }
    
    let mut result = 0;
    for (_, map) in mirror_maps.iter().enumerate() {
        // try to find matching rows
        let candidates_row = find_smudge_candidates(map); // create a list of smudge candidates (lines that after changing could create new reflection lines)
        let mut new_reflections = HashSet::new();
        for candidate in candidates_row {
            let mut new_map = map.clone();
            let new_char = if new_map[candidate.0].chars().nth(candidate.1).unwrap() == '.' {String::from('#')} else {String::from('.')};
            new_map[candidate.0].replace_range(candidate.1..=candidate.1, &new_char); // create a map with the replaced line (removed smudge)
            let excluded_reflections = match find_reflection_line(map) {
                None => Vec::new(),
                Some(line) => Vec::from([line])
            };
            new_reflections.extend(find_reflection_line_v2(&new_map, &excluded_reflections)); // if we find any reflection points we add them to a set
        }
        if new_reflections.is_empty() { // try to find matching columns if we didnt find reflection points in rows
            let cols = transpose_string_vector(map);
            let candidates_col = find_smudge_candidates(&cols);
            for candidate in candidates_col {
                let mut new_map = cols.clone();
                let new_char = if new_map[candidate.0].chars().nth(candidate.1).unwrap() == '.' {String::from('#')} else {String::from('.')};
                new_map[candidate.0].replace_range(candidate.1..=candidate.1, &new_char);
                let excluded_reflections = match find_reflection_line(&cols) {
                    None => Vec::new(),
                    Some(line) => Vec::from([line])
                };
                new_reflections.extend(find_reflection_line_v2(&new_map, &excluded_reflections));
            }
            // calculate score for column reflection points
            for refl in new_reflections {
                result += refl.1;
            }
        } else { // calculate score for row reflection points
            for refl in new_reflections {
                result += refl.1*100;
            }
        }
    }

    print!("the result is {} [time taken {}ms]", result, time.elapsed().as_millis());
}

fn find_smudge_candidates(map: &Vec<String>) -> Vec<(usize, usize)> {
    let mut candidates: Vec<(usize, usize)> = Vec::new();
    for line in map.iter().enumerate() {
        for line2 in map.iter().skip(line.0+1).enumerate() {
            match is_smudge_candidate(&line.1, &line2.1) {
                None => {},
                Some(some_line) => {
                    if line2.0 % 2 == 0 {
                        candidates.push((line.0, some_line));
                        candidates.push((line2.0 + line.0 + 1, some_line));
                    }
                }
            }
        }
    }
    candidates
}

fn is_smudge_candidate(string1: &str, string2: &str) -> Option<usize> {
    let mut diff_count = 0;
    let mut diff_index = 0;
    for ((i, ch1), ch2) in zip(string1.chars().enumerate(), string2.chars()) {
        if ch1 != ch2 {
            diff_count += 1;
            diff_index = i;
        }
    }
    if diff_count == 1 {
        Some(diff_index)
    } else {
        None
    }
}

fn find_reflection_line_v2(map: &Vec<String>, excluded_reflections: &Vec<(usize, usize)>) -> HashSet<(usize, usize)> {
    let mut possible_lines: HashSet<(usize, usize)> = HashSet::new();
    for (i, line) in map.iter().take(map.len()-1).enumerate() {
        let next_line = &map[i+1];
        if line == next_line {
            let mut start = i;
            let mut stop = i+1;
            let mut possible_relection = true;
            loop {
                if start > 0 {
                    start -= 1;
                } else {
                    break;
                }
                if stop < map.len()-1 {
                    stop += 1;
                } else {
                    break;
                }
                if map[start] != map[stop] {
                    possible_relection = false;
                    break;
                }
            }
            if possible_relection && !excluded_reflections.contains(&(i, i+1)) {
                possible_lines.insert((i, i+1));
            }
        }
    }
    possible_lines
}