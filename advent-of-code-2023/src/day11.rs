use std::time::Instant;

use crate::util::read_file;

pub fn day11_1(filename: String) {
    let mut filename_l = filename;
    if filename_l.is_empty() {
        filename_l = String::from("inputs/day10.txt");
    }   
    let file = read_file(&filename_l);
    let mut diagram:Vec<&str> = Vec::new();
    
    let time = Instant::now();
    for (_, line) in file.lines().enumerate() {
        diagram.push(line);
    }

    print!("the result is {} [time taken {}ms]", 0, time.elapsed().as_millis());
}

pub fn day11_2(filename: String) {
    let mut filename_l = filename;
    if filename_l.is_empty() {
        filename_l = String::from("inputs/day11.txt");
    }   
    let file = read_file(&filename_l);
    let mut diagram: Vec<&str> = Vec::new();
    
    let time = Instant::now();
    for (_, line) in file.lines().enumerate() {
        diagram.push(line);
    }

    print!("the result is {} [time taken {}ms]", 0, time.elapsed().as_millis());    
}