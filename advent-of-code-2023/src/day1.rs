use crate::util::read_file;
use regex::{Regex, Match};

pub fn day1_1() {
    let mut sum_of_numbers = 0;
    for line in read_file("inputs/day1.txt").lines() {
        let trimmed = line.trim_matches(char::is_alphabetic);
        let number1 = trimmed.chars().nth(0).unwrap();
        let number2 = trimmed.chars().nth_back(0).unwrap();
        let number_combined = format!("{}{}",number1, number2);
        sum_of_numbers += number_combined.parse::<i32>().unwrap();
    }
    print!("the result is {}", sum_of_numbers)
}

pub fn day1_2() {
    let mut sum_of_numbers = 0;
    let re = Regex::new(r"one|two|three|four|five|six|seven|eight|nine").unwrap();
    for line in read_file("inputs/day1.txt").lines() {
        let fst_digit_byte = match line.find(char::is_numeric) {
            None => line.len() + 1,
            Some(val) => val
        };
        let last_digit_byte = match line.rfind(char::is_numeric) {
            None => 0,
            Some(val) => val
        };
        

        let number1 = match re.find(line) {
            None => line.chars().nth(fst_digit_byte).unwrap(),
            Some(byte) => {
                let byte_i = byte.start();
                if byte_i < fst_digit_byte {
                    word_to_char(byte.as_str())
                } else {
                    line.chars().nth(fst_digit_byte).unwrap()
                }
            },
        };

        let mut last_index = 0;
        let mut prev_val: Option<Match> = None;
        let mut byte_i = 0;
        loop {
            match re.find_at(line, last_index) {
                None => {
                    break;
                },
                Some(val) => {
                    prev_val = Some(val);
                    last_index = val.start() + 1;
                    byte_i = prev_val.unwrap().start();
                }
            }
        }
        let number2 = if byte_i > last_digit_byte {
            word_to_char(prev_val.unwrap().as_str())
        } else {
            line.chars().nth(last_digit_byte).unwrap()
        };

        sum_of_numbers += format!("{}{}", number1, number2).parse::<i32>().unwrap();
    }
    print!("the result is {}", sum_of_numbers)
}

fn word_to_char(word: &str) -> char {
    match word {
        "one" => '1',
        "two" => '2',
        "three" => '3',
        "four" => '4',
        "five" => '5',
        "six" => '6',
        "seven" => '7',
        "eight" => '8',
        "nine" => '9',
        _ => '0',
    }
}