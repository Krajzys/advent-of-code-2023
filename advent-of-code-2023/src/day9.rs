use std::time::Instant;

use crate::util::read_file;

pub fn day9_1(filename: String) {
    let mut filename_l = filename;
    if filename_l.is_empty() {
        filename_l = String::from("inputs/day9.txt");
    }   
    let file = read_file(&filename_l);
    
    let time = Instant::now();
    let mut total_added = 0;
    for (_, line) in file.lines().enumerate() {
        // get the inital sequence from the line and push it to a new sequences vector
        let mut sequences: Vec<Vec<i32>> = Vec::new();
        let sequence: Vec<i32> = line.split(' ').map(|el| el.parse().unwrap()).collect();
        sequences.push(sequence.clone());

        // add first diff vector to the sequence vector then do it on subsequent diff vectors until we get all 0 vector
        let mut sequence_diff = get_diff_vector(sequence.clone());
        sequences.push(sequence_diff.clone());
        loop {
            sequence_diff = get_diff_vector(sequence_diff.clone());
            sequences.push(sequence_diff.clone());
            if sequence_diff.iter().all(|el| *el == 0) {
                break;
            }
        }

        // reverse the sequences vector and push change to the back (the change is last element plus the last change)
        let mut change = 0;
        sequences.reverse();
        for seq in &mut sequences {
            seq.push(seq.last().unwrap() + change);
            change = *seq.last().unwrap();
        }
        total_added += change;
    }

    print!("the result is {} [time taken {}ms]", total_added, time.elapsed().as_millis());
}

fn get_diff_vector(sequence: Vec<i32>) -> Vec<i32> {
    let mut diff_sequence: Vec<i32> = Vec::new();
    for i in 1..sequence.len() {
        diff_sequence.push(sequence[i] - sequence[i-1]);
    }
    diff_sequence
}

pub fn day9_2(filename: String) {
    let mut filename_l = filename;
    if filename_l.is_empty() {
        filename_l = String::from("inputs/day9.txt");
    }   
    let file = read_file(&filename_l);
    
    let time = Instant::now();
    let mut total_added = 0;
    for (_, line) in file.lines().enumerate() {
        // get the inital sequence from the line and push it to a new sequences vector
        let mut sequences: Vec<Vec<i32>> = Vec::new();
        let sequence: Vec<i32> = line.split(' ').map(|el| el.parse().unwrap()).collect();
        sequences.push(sequence.clone());

        // add first diff vector to the sequence vector then do it on subsequent diff vectors until we get all 0 vector
        let mut sequence_diff = get_diff_vector(sequence.clone());
        sequences.push(sequence_diff.clone());
        loop {
            sequence_diff = get_diff_vector(sequence_diff.clone());
            sequences.push(sequence_diff.clone());
            if sequence_diff.iter().all(|el| *el == 0) {
                break;
            }
        }

        // reverse the sequences vector and insert change in the front (the change is the first element minus the last change)
        let mut change = 0;
        sequences.reverse();
        for seq in &mut sequences {
            seq.insert(0, seq.first().unwrap() - change);
            change = *seq.first().unwrap();
        }
        total_added += change;
    }

    print!("the result is {} [time taken {}ms]", total_added, time.elapsed().as_millis());
}