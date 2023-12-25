use std::time::Instant;

use crate::util::read_file;

pub fn day16_1(filename: String) {
    let mut filename_l = filename;
    if filename_l.is_empty() {
        filename_l = String::from("inputs/day16.txt");
    }   
    let file = read_file(&filename_l);
    
    let time = Instant::now();
    let mut tile_map: Vec<String> = Vec::new();
    for (_, line) in file.lines().enumerate() {
        tile_map.push(String::from(line));
    }

    let mut beams: Vec<(usize, usize, isize, isize)> = Vec::new();
    let mut seen: Vec<(usize, usize, isize, isize)> = Vec::new();
    beams.push((0, 0, 0, 1));
    seen.push((0, 0, 0, 1));

    while !beams.is_empty() {
        let (row, col, mut row_move, mut col_move) = beams.pop().unwrap();
        let curr_tile = tile_map[row].chars().nth(col).unwrap();
        match curr_tile {
            '/' => {
                if row_move == 1 { // we came from the top
                    row_move = 0;
                    col_move = -1;
                }
                else if row_move == -1 { // we came from the bottom
                    row_move = 0;
                    col_move = 1;
                }
                else if col_move == 1 { // we came from the left
                    row_move = -1;
                    col_move = 0;
                }
                else if col_move == -1 { // we came from the right
                    row_move = 1;
                    col_move = 0;
                }
                let new = (row as isize + row_move, col as isize + col_move, row_move, col_move);
                push_if_valid(&tile_map, &mut seen, &mut beams, new);
            },
            '\\' => {
                if row_move == 1 { // we came from the top
                    row_move = 0;
                    col_move = 1;
                }
                else if row_move == -1 { // we came from the bottom
                    row_move = 0;
                    col_move = -1;
                }
                else if col_move == 1 { // we came from the left
                    row_move = 1;
                    col_move = 0;
                }
                else if col_move == -1 { // we came from the right
                    row_move = -1;
                    col_move = 0;
                }
                let new = (row as isize + row_move, col as isize + col_move, row_move, col_move);
                push_if_valid(&tile_map, &mut seen, &mut beams, new);
            },
            '-' => {
                if row_move != 0 { // we came from the top or bottom
                    let new = (row as isize, col as isize - 1, 0, -1);
                    push_if_valid(&tile_map, &mut seen, &mut beams, new);
                    let new = (row as isize, col as isize + 1, 0,  1);
                    push_if_valid(&tile_map, &mut seen, &mut beams, new);
                }
                else if col_move != 0 { // we came from the right or left
                    let new = (row as isize + row_move, col as isize + col_move, row_move, col_move);
                    push_if_valid(&tile_map, &mut seen, &mut beams, new);
                }
            },
            '|' => {
                if row_move != 0 { // we came from the top or bottom
                    let new = (row as isize + row_move, col as isize + col_move, row_move, col_move);
                    push_if_valid(&tile_map, &mut seen, &mut beams, new);
                }
                else if col_move != 0 { // we came from the right or left
                    let new = (row as isize - 1, col as isize, -1, 0);
                    push_if_valid(&tile_map, &mut seen, &mut beams, new);
                    let new = (row as isize + 1, col as isize,  1, 0);
                    push_if_valid(&tile_map, &mut seen, &mut beams, new);
                }
            },
            _ => {
                let new = (row as isize + row_move, col as isize + col_move, row_move, col_move);
                push_if_valid(&tile_map, &mut seen, &mut beams, new);
            }
        }
    }
    let unique = get_unique_positions(&seen);
    let result = unique.len();

    // _print_energized(tile_map, unique);

    print!("the result is {} [time taken {}ms]", result, time.elapsed().as_millis());
}

fn _print_energized(tile_map: Vec<String>, unique: Vec<(usize, usize)>) {
    for i in 0..tile_map.len() {
        for j in 0..tile_map[0].len() {
            let temp_pos = (i, j);
            if unique.contains(&temp_pos) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn push_if_valid(map: &Vec<String>, seen: &mut Vec<(usize, usize, isize, isize)>, beams: &mut Vec<(usize, usize, isize, isize)>, new: (isize, isize, isize, isize)) {
    if new.2 != 0 {
        let new_row = new.0 as isize;
        if new_row < map.len() as isize && new_row >= 0 {
            let new_rec = (new.0 as usize, new.1 as usize, new.2, new.3);
            if !seen.contains(&new_rec) {
                seen.push(new_rec);
                beams.push(new_rec);
            }
        }
    } else if new.3 != 0 {
        let new_col = new.1 as isize;
        if new_col < map[new.0 as usize].len() as isize && new_col >= 0 {
            let new_rec = (new.0 as usize, new.1 as usize, new.2, new.3);
            if !seen.contains(&new_rec) {
                seen.push(new_rec);
                beams.push(new_rec);
            }
        }
    }
}

fn get_unique_positions(seen: &Vec<(usize, usize, isize, isize)>) -> Vec<(usize, usize)> {
    let mut unique: Vec<(usize, usize)> = Vec::new();
    for position in seen {
        let new_position = (position.0, position.1);
        if !unique.contains(&new_position) {
            unique.push(new_position);
        }
    }
    unique
}

pub fn day16_2(filename: String) {
    let mut filename_l = filename;
    if filename_l.is_empty() {
        filename_l = String::from("inputs/day16.txt");
    }   
    let file = read_file(&filename_l);
    
    let time = Instant::now();
    let mut result = 0;
    let mut tile_map: Vec<String> = Vec::new();
    for (_, line) in file.lines().enumerate() {
        tile_map.push(String::from(line));
    }
    let mut starting_positions: Vec<(usize, usize, isize, isize)> = Vec::new();
    for i in 0..tile_map.len() { // add initial positions starting from rows (left and right)
        starting_positions.push((i, 0, 0, 1));
        starting_positions.push((i, tile_map[0].len()-1, 0, -1));
    }
    for i in 0..tile_map[0].len() { // add initial positions starting from cols (up and down)
        starting_positions.push((0, i, 1, 0));
        starting_positions.push((tile_map.len()-1, i, -1, 0));
    }

    println!("starting positions {:?}", starting_positions);

    while !starting_positions.is_empty() {
        println!("positions left {}", starting_positions.len());
        let mut beams: Vec<(usize, usize, isize, isize)> = Vec::new();
        let mut seen: Vec<(usize, usize, isize, isize)> = Vec::new();
        let curr_starting_pos = starting_positions.pop().unwrap();
        beams.push(curr_starting_pos);
        seen.push(curr_starting_pos);

        while !beams.is_empty() {
            let (row, col, mut row_move, mut col_move) = beams.pop().unwrap();
            let curr_tile = tile_map[row].chars().nth(col).unwrap();
            match curr_tile {
                '/' => {
                    if row_move == 1 { // we came from the top
                        row_move = 0;
                        col_move = -1;
                    }
                    else if row_move == -1 { // we came from the bottom
                        row_move = 0;
                        col_move = 1;
                    }
                    else if col_move == 1 { // we came from the left
                        row_move = -1;
                        col_move = 0;
                    }
                    else if col_move == -1 { // we came from the right
                        row_move = 1;
                        col_move = 0;
                    }
                    let new = (row as isize + row_move, col as isize + col_move, row_move, col_move);
                    push_if_valid(&tile_map, &mut seen, &mut beams, new);
                },
                '\\' => {
                    if row_move == 1 { // we came from the top
                        row_move = 0;
                        col_move = 1;
                    }
                    else if row_move == -1 { // we came from the bottom
                        row_move = 0;
                        col_move = -1;
                    }
                    else if col_move == 1 { // we came from the left
                        row_move = 1;
                        col_move = 0;
                    }
                    else if col_move == -1 { // we came from the right
                        row_move = -1;
                        col_move = 0;
                    }
                    let new = (row as isize + row_move, col as isize + col_move, row_move, col_move);
                    push_if_valid(&tile_map, &mut seen, &mut beams, new);
                },
                '-' => {
                    if row_move != 0 { // we came from the top or bottom
                        let new = (row as isize, col as isize - 1, 0, -1);
                        push_if_valid(&tile_map, &mut seen, &mut beams, new);
                        let new = (row as isize, col as isize + 1, 0,  1);
                        push_if_valid(&tile_map, &mut seen, &mut beams, new);
                    }
                    else if col_move != 0 { // we came from the right or left
                        let new = (row as isize + row_move, col as isize + col_move, row_move, col_move);
                        push_if_valid(&tile_map, &mut seen, &mut beams, new);
                    }
                },
                '|' => {
                    if row_move != 0 { // we came from the top or bottom
                        let new = (row as isize + row_move, col as isize + col_move, row_move, col_move);
                        push_if_valid(&tile_map, &mut seen, &mut beams, new);
                    }
                    else if col_move != 0 { // we came from the right or left
                        let new = (row as isize - 1, col as isize, -1, 0);
                        push_if_valid(&tile_map, &mut seen, &mut beams, new);
                        let new = (row as isize + 1, col as isize,  1, 0);
                        push_if_valid(&tile_map, &mut seen, &mut beams, new);
                    }
                },
                _ => {
                    let new = (row as isize + row_move, col as isize + col_move, row_move, col_move);
                    push_if_valid(&tile_map, &mut seen, &mut beams, new);
                }
            }
        }
        let unique = get_unique_positions(&seen);
        if unique.len() > result {
            result = unique.len();
        }
    }
    // _print_energized(tile_map, unique);

    print!("the result is {} [time taken {}ms]", result, time.elapsed().as_millis());
}