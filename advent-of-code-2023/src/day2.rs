use std::collections::HashMap;

use crate::util::read_file;

pub fn day2_1() {
    let mut sum_of_game_ids = 0;
    for line in read_file("inputs/day2.txt").lines() {
        let game_id: u32 = line
            .split(" ")
            .nth(1)
            .expect("bad input string format")
            .replace(":", "")
            .parse()
            .expect("error converting to integer");

        let games_data = &line[line.find(":").expect("bad input string format")+2..];
        let mut game_possible = true;

        for game in games_data.split(";") {
            game_possible = true;
            let game = game.trim();
            for cubes in game.split(",") {
                let cubes = cubes.trim();
                let cubes_count = cubes.split(" ").nth(0).expect("bad input string format");
                let cubes_color = cubes.split(" ").nth(1).expect("bad input string format");
                if !is_possible(cubes_color, cubes_count) {
                    game_possible = false;
                    break;
                }
            }
            if !game_possible {
                break;
            }
        }
        if game_possible {
            sum_of_game_ids += game_id;
        }
    }
    print!("the result is {}", sum_of_game_ids)
}

fn is_possible(cube_color: &str, cube_count: &str) -> bool {
    let cube_count: u32 = cube_count.parse().expect("error converting to integer");
    match cube_color {
        "red" => cube_count <= 12,
        "green" => cube_count <= 13,
        "blue" => cube_count <= 14,
        _ => false
    }
}

pub fn day2_2() {
    let mut sum_of_powers = 0;
    for line in read_file("inputs/day2.txt").lines() {
        let games_data = &line[line.find(":").expect("bad input string format")+2..];
        let mut min_cube_count: HashMap<&str, u32> = HashMap::new();

        for game in games_data.split(";") {
            let game = game.trim();
            for cubes in game.split(",") {
                let cubes = cubes.trim();
                let cubes_count: u32 = cubes.split(" ").nth(0).expect("bad input string format").parse().expect("error converting to integer");
                let cubes_color = cubes.split(" ").nth(1).expect("bad input string format");
                let zero = 0;
                let curr_max = min_cube_count.get(cubes_color).unwrap_or_else(|| &zero);
                if &cubes_count > curr_max {
                    min_cube_count.insert(cubes_color, cubes_count);
                }
            }
        }
        let mut powers = 1;
        for val in min_cube_count.values() {
            powers *= val;
        }
        sum_of_powers += powers;
    }
    print!("the result is {}", sum_of_powers)
}