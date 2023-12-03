use std::collections::HashSet;

use crate::util::read_file;

pub fn day3_1() {
    let file = read_file("inputs/day3.txt");
    let instruction: Vec<&str> = file.lines().collect();
    let mut sum_of_nums = 0;
    for (r, line) in instruction.iter().enumerate() {
        for (c, ch) in line.chars().enumerate() {
            if !ch.is_numeric() && ch != '.' {
                sum_of_nums += get_adjacent_sum(&instruction, r, c);
                println!("");
            }
        }
    }
    print!("the result is {}", sum_of_nums);
}

fn get_adjacent_sum(instruction: &Vec<&str>, r: usize, c: usize) -> i32 {
    // let used_nums = std::collections::HashSet::new();
    print!("{r}-{c}: ");
    let mut used_indexes: HashSet<String> = HashSet::new();
    let mut sum_of_nums  = 0;
    for r2 in -1..=1 {
        if r == 0 && r2 == -1 || r == instruction.len()-1 && r2 == 1 {
            continue;
        }
        for c2 in -1..=1 {
            if r2 == 0 && c2 == 0 {
                continue;
            }
            let r3 = (r as i32 + r2) as usize;
            if c == 0 && c2 == -1 || c == instruction[r3].len()-1 && c2 == 1 {
                continue;
            }
            let c3 = (c as i32 + c2) as usize;
            // print!("{}", instruction[r3].chars().nth(c3).unwrap());

            let index = format!("{}-{}", r3,c3);
            if instruction[r3].chars().nth(c3).unwrap().is_numeric() && !used_indexes.contains(&index) {
                let num = find_num(instruction[r3], c3, r3, &mut used_indexes);
                sum_of_nums += num.parse::<i32>().unwrap();
                print!("{},", num)
            }
        }
    }
    sum_of_nums
}

fn find_num(line: &str, pos: usize, row: usize, used_indexes: &mut HashSet<String>) -> String {
    let mut pos = pos;
    while pos > 0 && line.chars().nth(pos-1).unwrap().is_numeric() {
        pos -= 1;
    }
    used_indexes.insert(format!("{}-{}", row, pos));
    let mut num = format!("{}", line.chars().nth(pos).unwrap());
    pos += 1;
    
    while pos < line.len() && line.chars().nth(pos).unwrap().is_numeric() {
        used_indexes.insert(format!("{}-{}", row, pos));
        num = format!("{}{}",num, line.chars().nth(pos).unwrap());
        pos +=1;
    }
    num
}

pub fn day3_2() {
    let file = read_file("inputs/day3.txt");
    let instruction: Vec<&str> = file.lines().collect();
    let mut sum_of_nums = 0;
    for (r, line) in instruction.iter().enumerate() {
        for (c, ch) in line.chars().enumerate() {
            if !ch.is_numeric() && ch == '*' {
                sum_of_nums += get_adjacent_sum_two(&instruction, r, c);
                println!("");
            }
        }
    }
    print!("the result is {}", sum_of_nums);
}

fn get_adjacent_sum_two(instruction: &Vec<&str>, r: usize, c: usize) -> i32 {
    // let used_nums = std::collections::HashSet::new();
    print!("{r}-{c}: ");
    let mut used_indexes: HashSet<String> = HashSet::new();
    let mut mul_of_nums  = 1;
    let mut nums_found = 0;
    for r2 in -1..=1 {
        if r == 0 && r2 == -1 || r == instruction.len()-1 && r2 == 1 {
            continue;
        }
        for c2 in -1..=1 {
            if r2 == 0 && c2 == 0 {
                continue;
            }
            let r3 = (r as i32 + r2) as usize;
            if c == 0 && c2 == -1 || c == instruction[r3].len()-1 && c2 == 1 {
                continue;
            }
            let c3 = (c as i32 + c2) as usize;
            // print!("{}", instruction[r3].chars().nth(c3).unwrap());
            
            let index = format!("{}-{}", r3,c3);
            if instruction[r3].chars().nth(c3).unwrap().is_numeric() && !used_indexes.contains(&index) {
                nums_found += 1;
                if nums_found > 2 {
                    print!("NULL");
                    return 0;
                }
                let num = find_num(instruction[r3], c3, r3, &mut used_indexes);
                mul_of_nums *= num.parse::<i32>().unwrap();
                print!("{},", num)
            }
        }
    }
    if nums_found < 2 {
        print!("NULL");
        0
    } else {
        mul_of_nums
    }
}