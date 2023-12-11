use std::time::Instant;

use crate::util::read_file;

pub fn day11_1(filename: String) {
    let mut filename_l = filename;
    if filename_l.is_empty() {
        filename_l = String::from("inputs/day11.txt");
    }   
    let file = read_file(&filename_l);
    let mut sky_map:Vec<String> = Vec::new();
    let mut galaxies_positions: Vec<(usize, usize)> = Vec::new();
    
    let time = Instant::now();
    // read file into vector and expand rows (if they only have dots then insert one more)
    for (_, line) in file.lines().enumerate() {
        sky_map.push(String::from(line));
        if line.chars().all(|c| c == '.') {
            sky_map.push(String::from(line));
        }
    }

    // expand columns, first find positions to add columns to, then insert dots in every elligible row
    let row_len = sky_map[0].len();
    let mut insert_at: Vec<usize> = Vec::new();
    for col in 0..row_len {
        let mut all_spaces = true;
        for row in 0..sky_map.len() {
            if sky_map[row].chars().nth(col).unwrap() != '.' {
                all_spaces = false;
                break;
            }
        }
        if all_spaces {
            insert_at.push(insert_at.len() + col);
        }
    }
    for col in &insert_at {
        for row in &mut sky_map {
            row.insert(*col, '.');
        }
    }

    // find all the galaxies positions in a expanded sky_map
    for (i, row) in sky_map.iter().enumerate() {
        for (j, char) in row.chars().enumerate() {
            if char == '#' {
                galaxies_positions.push((i, j))
            }
        }
    }

    // calculate sum of distances which is abs(galaxy1 row - galaxy2 row) + abs(galaxy1 col - galaxy2 col)
    let mut sum_of_distances = 0;
    let all_permutations = get_all_pair_permutations(&galaxies_positions);
    for (galaxy1, galaxy2) in all_permutations {
        sum_of_distances += (galaxy1.0 as isize - galaxy2.0 as isize).abs() + (galaxy1.1 as isize - galaxy2.1 as isize).abs()
    }

    print!("the result is {} [time taken {}ms]", sum_of_distances, time.elapsed().as_millis());
}

fn get_all_pair_permutations(galaxies_position: &Vec<(usize, usize)>) -> Vec<((usize, usize), (usize, usize))> {
    let mut all_permut: Vec<((usize, usize), (usize, usize))> = Vec::new();
    for (i, galaxy1) in galaxies_position.iter().enumerate() {
        for galaxy2 in galaxies_position.iter().skip(i+1) {
            all_permut.push((*galaxy1, *galaxy2));
        }
    }
    all_permut
}

pub fn day11_2(filename: String) {
    let mut filename_l = filename;
    if filename_l.is_empty() {
        filename_l = String::from("inputs/day11.txt");
    }   
    let file = read_file(&filename_l);
    let mut sky_map:Vec<String> = Vec::new();
    let mut galaxies_positions: Vec<(usize, usize)> = Vec::new();
    
    let time = Instant::now();
    // read file into vector
    for (_, line) in file.lines().enumerate() {
        sky_map.push(String::from(line));
    }

    // find row indexes where the empty space will expand
    let mut expanded_rows: Vec<usize> = Vec::new();
    for (i, row) in sky_map.iter().enumerate() {
        if row.chars().all(|c| c == '.') {
            expanded_rows.push(i);
        }
    }

    // find column indexes where the empty space will expand
    let row_len = sky_map[0].len();
    let mut expanded_columns: Vec<usize> = Vec::new();
    for col in 0..row_len {
        let mut all_spaces = true;
        for row in 0..sky_map.len() {
            if sky_map[row].chars().nth(col).unwrap() != '.' {
                all_spaces = false;
                break;
            }
        }
        if all_spaces {
            expanded_columns.push(col);
        }
    }

    // find all the galaxies positions in a expanded sky_map
    for (i, row) in sky_map.iter().enumerate() {
        for (j, char) in row.chars().enumerate() {
            if char == '#' {
                galaxies_positions.push((i, j))
            }
        }
    }

    // calculate sum of distances which is abs(galaxy1 row - galaxy2 row) + abs(galaxy1 col - galaxy2 col)
    // also if there are any empty lines or columns between them add appropriate distance to the sum
    const EMPTY_LINES_ADDED: isize = 999999;
    let mut sum_of_distances = 0;
    let all_permutations = get_all_pair_permutations(&galaxies_positions);
    for (galaxy1, galaxy2) in all_permutations {
        let row_range = galaxy1.0.min(galaxy2.0)..galaxy1.0.max(galaxy2.0);
        let col_range = galaxy1.1.min(galaxy2.1)..galaxy1.1.max(galaxy2.1);
        sum_of_distances += (galaxy1.0 as isize - galaxy2.0 as isize).abs() + (galaxy1.1 as isize - galaxy2.1 as isize).abs();
        for expanded_row in &expanded_rows {
            if row_range.contains(&expanded_row) {
                sum_of_distances += EMPTY_LINES_ADDED;
            }
        }
        for expanded_col in &expanded_columns {
            if col_range.contains(&expanded_col) {
                sum_of_distances += EMPTY_LINES_ADDED;
            }
        }
    }

    print!("the result is {} [time taken {}ms]", sum_of_distances, time.elapsed().as_millis());
}