use std::time::Instant;

use crate::util::read_file;

pub fn day14_1(filename: String) {
    let mut filename_l = filename;
    if filename_l.is_empty() {
        filename_l = String::from("inputs/day14.txt");
    }   
    let file = read_file(&filename_l);
    let mut rock_map: Vec<String> = Vec::new();
    
    let time = Instant::now();
    // read file into vector
    for (_, line) in file.lines().enumerate() {
        rock_map.push(String::from(line));
    }

    let mut rock_columns = transpose_string_vector(&rock_map); // transpose so it's easier to work on columns
    rock_columns = tilt_platform(&rock_columns); // tilt north (initial up direction)

    let mut result = 0;
    for col in rock_columns {
        for (pos, char) in col.chars().enumerate() {
            if char == 'O' {
                result += col.len() - pos;
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

fn rotate_right(vector: &Vec<String>) -> Vec<String> {
    let col_count = vector[0].len();
    let row_count = vector.len();
    let mut cols: Vec<String> = Vec::new();
    for col in (0..col_count).rev() {
        let mut new_col = String::new();
        for row in 0..row_count {
            new_col.push(vector[row].chars().nth(col).unwrap());
        }
        cols.push(new_col);
    }
    cols
}

pub fn day14_2(filename: String) {
    let mut filename_l = filename;
    if filename_l.is_empty() {
        filename_l = String::from("inputs/day14.txt");
    }   
    let file = read_file(&filename_l);
    let mut rock_map: Vec<String> = Vec::new();
    
    let time = Instant::now();
    // read file into vector
    for (_, line) in file.lines().enumerate() {
        rock_map.push(String::from(line));
    }
    const CYCLE_COUNT: usize = 1000000000;

    let mut seen: Vec<Vec<String>> = Vec::new();
    let mut rock_columns = transpose_string_vector(&rock_map); // transpose so it's easier to work on columns
    seen.push(rock_columns.clone());

    let mut loop_after = 0;
    
    for i in 0..CYCLE_COUNT {
        if i % 4 == 0 {
            if seen.contains(&rock_columns) && i != 0 {
                loop_after = i;
                break;
            } else {
                seen.push(rock_columns.clone());
            }
        }
        rock_columns = tilt_platform(&rock_columns); // tilt up (whatever direction it is)
        rock_columns = rotate_right(&rock_columns); // rotate the vector so that the previous left direction is now in rows
    }
    println!("loops after {loop_after}");

    for _ in 0..(CYCLE_COUNT%loop_after) {
        rock_columns = tilt_platform(&mut rock_columns); // tilt north (initial up direction)
        rock_columns = rotate_right(&rock_columns); // rotate the vector so that the previous left direction is now in rows
    }

    let mut result = 0;
    for col in rock_columns {
        for (pos, char) in col.chars().enumerate() {
            if char == 'O' {
                result += col.len() - pos;
            }
        }
    }
    
    print!("the result is {} [time taken {}ms]", result, time.elapsed().as_millis());
}

fn tilt_platform(rock_columns: &Vec<String>) -> Vec<String> {
    let mut new_rock_columns = rock_columns.clone();
    for col in &mut new_rock_columns {
        let mut last_free_index = 0;
        for i in 0..col.len() {
            let curr_char = col.chars().nth(i).unwrap();
            if curr_char == '#' {
                last_free_index = i + 1;
            } else if curr_char == 'O' && last_free_index < i {
                col.remove(last_free_index);
                col.insert(last_free_index, 'O');
                col.remove(i);
                col.insert(i, '.');
                last_free_index += 1;
            } else if curr_char == 'O' {
                last_free_index = i + 1;
            }
        }
    }
    new_rock_columns
}