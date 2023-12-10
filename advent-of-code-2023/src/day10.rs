use std::time::Instant;

use crate::util::read_file;

pub fn day10_1(filename: String) {
    let mut filename_l = filename;
    if filename_l.is_empty() {
        filename_l = String::from("inputs/day10.txt");
    }   
    let file = read_file(&filename_l);
    let mut diagram:Vec<&str> = Vec::new();
    let mut s_position = (0, 0);
    
    let time = Instant::now();
    for (i, line) in file.lines().enumerate() {
        diagram.push(line);
        if line.find('S').is_some() {
            s_position = (i, line.find('S').unwrap())
        }
    }

    // walk through the table finding all the parts of the pipe that are connected
    let mut visited: Vec<(usize, usize)> = Vec::new();
    let mut to_visit: Vec<(usize, usize)> = Vec::new();
    to_visit.push(s_position);

    while !to_visit.is_empty() {
        let current_node = to_visit.pop().unwrap();
        to_visit.extend(find_connected(current_node, &diagram, &visited));
        visited.push(current_node);
    }

    print!("the result is {} [time taken {}ms]", visited.len()/2, time.elapsed().as_millis());
}

fn find_connected(pos: (usize, usize), diagram: &Vec<&str>, visited: &Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    // find all the connected
    let mut positions_to_check: Vec<(isize, isize)> = Vec::new();
    let curr_pipe = diagram.get(pos.0).unwrap().chars().nth(pos.1).unwrap();
    if connects_north(curr_pipe) {
        positions_to_check.push((-1, 0));
    }
    if connects_south(curr_pipe) {
        positions_to_check.push((1, 0));
    }
    if connects_east(curr_pipe) {
        positions_to_check.push((0, 1));
    }
    if connects_west(curr_pipe) {
        positions_to_check.push((0, -1));
    }

    let mut connected_positions: Vec<(usize, usize)> = Vec::new();
    for change in positions_to_check {
        let position_to_check = ((pos.0 as isize + change.0) as usize, (pos.1 as isize + change.1) as usize);
        if position_to_check == pos {
            continue;
        }
        let line = match diagram.get(position_to_check.0) {
            None => continue,
            Some(val) => val
        };
        let pipe = match line.chars().nth(position_to_check.1) {
            None => continue,
            Some(val) => val
        };
        if is_connected(change, pipe) && !visited.contains(&position_to_check) {
            connected_positions.push(position_to_check);
        }

    };
    connected_positions
}

fn find_s_replacement(s_pos: (usize, usize), diagram: &Vec<&str>) -> char {
    // find the pipe that should be at the starting position
    let positions_to_check: Vec<(isize, isize)> = Vec::from([(-1, 0), (1, 0), (0, 1), (0, -1)]);
    let mut south = false;
    let mut north = false;
    let mut east = false;
    let mut west = false;

    for change in positions_to_check {
        let position_to_check = ((s_pos.0 as isize + change.0) as usize, (s_pos.1 as isize + change.1) as usize);
        if position_to_check == s_pos {
            continue;
        }
        let line = match diagram.get(position_to_check.0) {
            None => continue, // we are out of bound so we continue
            Some(val) => val
        };
        let pipe = match line.chars().nth(position_to_check.1) {
            None => continue, // we are out of bound so we continue
            Some(val) => val
        };
        match change {
            (0, 1) => east = connects_west(pipe), // we are west from the symbol
            (0, -1) => west = connects_east(pipe), // we are east from the symbol
            (1, 0) => south = connects_north(pipe), // we are north from the symbol
            (-1, 0) => north = connects_south(pipe), // we are south from the symbol
            _ => panic!("unexpected change to check")
        };
    };
    match (south, north, west, east) {
        (true, true, false, false) => '|',
        (true, false, true, false) => '7',
        (true, false, false, true) => 'F',
        (false, true, false, true) => 'L',
        (false, true, true, false) => 'J',
        (false, false, true, true) => '-',
        (true, true, true, true) => 'S',
        _ => panic!("Unexpected combination {:?}", (south, north, west, east))
    }
}

fn is_connected(change: (isize, isize), pipe: char) -> bool {
    match change {
        (0, 1) => connects_west(pipe), // we are west from the symbol
        (0, -1) => connects_east(pipe), // we are east from the symbol
        (1, 0) => connects_north(pipe), // we are north from the symbol
        (-1, 0) => connects_south(pipe), // we are south from the symbol
        _ => panic!("unexpected change to check")
    }
}

fn connects_east(pipe: char) -> bool {
    ['S', '-', 'L', 'F'].contains(&pipe)
}

fn connects_west(pipe: char) -> bool {
    ['S', '-', 'J', '7'].contains(&pipe)
}

fn connects_north(pipe: char) -> bool  {
    ['S', '|', 'J', 'L'].contains(&pipe)
}
fn connects_south(pipe: char) -> bool  {
    ['S', '|', 'F', '7'].contains(&pipe)
}

pub fn day10_2(filename: String) {
    let mut filename_l = filename;
    if filename_l.is_empty() {
        filename_l = String::from("inputs/day10.txt");
    }   
    let file = read_file(&filename_l);
    let mut diagram: Vec<&str> = Vec::new();
    let mut s_position = (0, 0);
    
    let time = Instant::now();
    for (i, line) in file.lines().enumerate() {
        diagram.push(line);
        if line.find('S').is_some() {
            s_position = (i, line.find('S').unwrap())
        }
    }

    let mut visited: Vec<(usize, usize)> = Vec::new();
    let mut to_visit: Vec<(usize, usize)> = Vec::new();
    to_visit.push(s_position);

    while !to_visit.is_empty() {
        let current_node = to_visit.pop().unwrap();
        to_visit.extend(find_connected(current_node, &diagram, &visited));
        visited.push(current_node);
    }

    let new_line = String::from(diagram[s_position.0].replace('S', find_s_replacement(s_position, &diagram).to_string().as_str()));
    diagram[s_position.0] = new_line.as_str();

    let mut restricted_by_pipes: Vec<(usize, usize)> = Vec::new();
    for (row, line) in diagram.iter().enumerate() {
        for (col, _) in line.chars().enumerate() {
            let pos = (row, col);
            if !visited.contains(&pos) {
                if count_cutoffs(pos, &diagram, &visited) % 2 == 1 {
                    restricted_by_pipes.push(pos);
                }
            }
        }
    }

    print!("the result is {} [time taken {}ms]", restricted_by_pipes.len(), time.elapsed().as_millis());    
}

fn count_cutoffs(pos: (usize, usize), diagram: &Vec<&str>, visited: &Vec<(usize, usize)>) -> i32 {
    // count cutoffs to the left of the pos
    let mut cutoffs = 0;
    let line = diagram.get(pos.0).unwrap();
    let mut gap_top = false;
    let mut gap_bottom = false;
    for (i, symbol) in line.chars().enumerate() {
        if i >= pos.1 {
            break;
        }
        if !visited.contains(&(pos.0, i)) {
            gap_top = false;
            gap_bottom = false;
            continue;
        }
        if symbol == '|' {
            cutoffs += 1;
            gap_top = false;
            gap_bottom = false;
        }
        if symbol == 'L' || symbol == 'J' {
            if gap_top {
                gap_top = false;
                cutoffs += 1;
            } else if gap_bottom {
                gap_bottom = false;
            } else {
                gap_bottom = true;
                gap_top = false;
            }
        }
        if symbol == '7' || symbol == 'F' {
            if gap_bottom {
                gap_bottom = false;
                cutoffs += 1;
            } else if gap_top {
                gap_top = false;
            } else {
                gap_top = true;
                gap_bottom = false;
            }
        }
    }
    cutoffs
}