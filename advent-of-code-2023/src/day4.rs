use std::collections::HashMap;

use crate::util::read_file;

pub fn day4_1() {
    let file = read_file("inputs/day4.txt");
    let games: Vec<&str> = file.lines().collect();
    let mut total = 0;
    for (_, line) in games.iter().enumerate() {
        let line_cut = String::from(&line[line.find(':').unwrap()+2..]);
        let nums: Vec<&str> = line_cut.split('|').collect();
        let winning_nums = nums[0].trim().replace("  ", " ");
        let winning_nums: Vec<&str> = winning_nums.split(' ').collect();
        let our_nums = nums[1].trim().replace("  ", " ");
        let our_nums = our_nums.split(' ');

        let mut score = 0;
        for num in our_nums {
            if winning_nums.contains(&num) {
                score = if score == 0 {1} else {score*2};
            }
        }
        total += score;
    }
    print!("the result is {}", total);
}

pub fn day4_2() {
    let file = read_file("inputs/day4.txt");
    let games: Vec<&str> = file.lines().collect();
    let mut total = 0;
    let mut cards_dict: HashMap<usize, usize> = HashMap::new();

    for i in 0..games.len() {
        cards_dict.insert(i, 1);
    }
    
    for (r, line) in games.iter().enumerate() {
        for _ in 0..cards_dict[&r] {
            let line_cut = String::from(&line[line.find(':').unwrap()+2..]);
            let nums: Vec<&str> = line_cut.split('|').collect();
            let winning_nums = nums[0].trim().replace("  ", " ");
            let winning_nums: Vec<&str> = winning_nums.split(' ').collect();
            let our_nums = nums[1].trim().replace("  ", " ");
            let our_nums = our_nums.split(' ');
            let mut wins = 0;

            for num in our_nums {
                if winning_nums.contains(&num) {
                    wins += 1;
                }
            }
            for i in 1..=wins {
                let game_no = r+i;
                if game_no < cards_dict.len() {
                    cards_dict.insert(game_no, cards_dict[&game_no]+1);
                }
            }
        }
    }
    for v in cards_dict.values() {
        total += v;
    }
    print!("the result is {}", total);
}
