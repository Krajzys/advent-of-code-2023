use std::time::Instant;

use crate::util::read_file;

pub fn day12_1(filename: String) {
    let mut filename_l = filename;
    if filename_l.is_empty() {
        filename_l = String::from("inputs/day12.txt");
    }   
    let file = read_file(&filename_l);
    let mut springs_conditions:Vec<(&str, Vec<usize>)> = Vec::new();
    
    let time = Instant::now();
    // read file into vector
    for (_, line) in file.lines().enumerate() {
        let record = line.split(' ').collect::<Vec<&str>>()[0];
        let damaged: Vec<usize> = line.split(' ').collect::<Vec<&str>>()[1]
                                      .split(',').map(|c| c.parse::<usize>().unwrap()).collect();
        springs_conditions.push((record, damaged.clone()));
    }

    let mut possiblities_count = 0;
    for (record, damaged) in springs_conditions {
        let question_mark_count = record.chars().filter(|c| *c == '?').count();
        let broken_count = record.chars().filter(|c| *c == '#').count();
        let possibilities = get_n_len_permutations(question_mark_count, damaged.iter().sum::<usize>() - broken_count);
        for possiblity in possibilities {
            let mut new_str = String::from(record);
            for char in possiblity.chars() {
                new_str = new_str.replacen('?', String::from(char).as_str(), 1);
            }
            if record_matches_damaged(&new_str, &damaged) {
                possiblities_count += 1;
            }
        }
    }

    print!("the result is {} [time taken {}ms]", possiblities_count, time.elapsed().as_millis());
}

fn record_matches_damaged(record_no_questions: &str, damaged: &Vec<usize>) -> bool {
    let replaced = record_no_questions.split('.').filter(|c| !c.is_empty()).map(|el| el.len()).collect::<Vec<usize>>();
    return replaced.len() == damaged.len() && replaced.iter().zip(damaged).all(|(el1, el2)| el1 == el2);
}

fn get_all_len_permutations(question_mark_count: usize) -> Vec<String> {
    let mut possible_permutations: Vec<String> = Vec::new();
    for num_of_broken in 0..=question_mark_count {
        let mut positions: Vec<usize> = Vec::from_iter(0..num_of_broken);
        let ending_string = String::from_iter(std::iter::repeat('#').take(num_of_broken));
        loop {
            let mut inital_string = String::new();
            for j in 0..question_mark_count {
                if positions.contains(&j) {
                    inital_string.push('#');
                }  else {
                    inital_string.push('.');
                }
            }
            possible_permutations.push(inital_string.clone());
            if num_of_broken == 0 {
                break;
            }
            let mut curr_increase = num_of_broken-1;
            if curr_increase >= positions.len() || inital_string.ends_with(&ending_string) {
                break;
            }
            while positions[curr_increase] >= question_mark_count-(positions.len()-1-curr_increase)-1 {
                curr_increase -= 1;
                let mut curr_increase_val = positions[curr_increase] + 1;
                for num in &mut positions[curr_increase+1..] {
                    *num = curr_increase_val+1;
                    curr_increase_val += 1;
                }
            }
            positions[curr_increase] += 1;
        }
    }
    possible_permutations
}

fn get_n_len_permutations(question_mark_count: usize, n: usize) -> Vec<String> {
    let mut possible_permutations: Vec<String> = Vec::new();
    let num_of_broken = n;
    let mut positions: Vec<usize> = Vec::from_iter(0..num_of_broken);
    let ending_string = String::from_iter(std::iter::repeat('#').take(num_of_broken));
    loop {
        let mut inital_string = String::new();
        for j in 0..question_mark_count {
            if positions.contains(&j) {
                inital_string.push('#');
            }  else {
                inital_string.push('.');
            }
        }
        possible_permutations.push(inital_string.clone());
        if num_of_broken == 0 {
            break;
        }
        let mut curr_increase = num_of_broken-1;
        if curr_increase >= positions.len() || inital_string.ends_with(&ending_string) {
            break;
        }
        while positions[curr_increase] >= question_mark_count-(positions.len()-1-curr_increase)-1 {
            curr_increase -= 1;
            let mut curr_increase_val = positions[curr_increase] + 1;
            for num in &mut positions[curr_increase+1..] {
                *num = curr_increase_val+1;
                curr_increase_val += 1;
            }
        }
        positions[curr_increase] += 1;
    }
    possible_permutations
}

pub fn day12_2(filename: String) {
    let mut filename_l = filename;
    if filename_l.is_empty() {
        filename_l = String::from("inputs/day12.txt");
    }   
    let file = read_file(&filename_l);
    let mut springs_conditions:Vec<(String, Vec<usize>)> = Vec::new();
    
    let time = Instant::now();
    // read file into vector
    for (_, line) in file.lines().enumerate() {
        let record = line.split(' ').collect::<Vec<&str>>()[0];
        let damaged: Vec<usize> = line.split(' ').collect::<Vec<&str>>()[1]
                                      .split(',').map(|c| c.parse::<usize>().unwrap()).collect();
        let mut new_record = String::from(record);
        new_record.push('?');
        springs_conditions.push((new_record.repeat(5), damaged.repeat(5)));
    }

    
    // try to setup correct values at the start and at the end e.g. for ??.?? 1,1 - see how many times you can start the string with one # - 2 times, end with one # - 2 times, result 4 possibilities
    // multiply the total by the amount of times you can correctly assign those values
    let possiblities_count = 0;
    // for (record, damaged) in springs_conditions {
    //     let question_mark_count = record.chars().filter(|c| *c == '?').count();
    //     let broken_count = record.chars().filter(|c| *c == '#').count();
    //     let possibilities = get_n_len_permutations(question_mark_count, damaged.iter().sum::<usize>() - broken_count);
    //     println!("curr -> {record}");
    //     for possiblity in possibilities {
    //         let mut new_str = record.clone();
    //         for char in possiblity.chars() {
    //             new_str = new_str.replacen('?', String::from(char).as_str(), 1);
    //         }
    //         if record_matches_damaged(&new_str, &damaged) {
    //             possiblities_count += 1;
    //         }
    //     }
    // }

    print!("the result is {} [time taken {}ms]", possiblities_count, time.elapsed().as_millis());
}