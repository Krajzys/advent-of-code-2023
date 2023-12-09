use std::env;

mod util;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.get(1) {
        None => {
            println!("Usage: {} day-part <filename>\nExample: {} 5-1", args.get(0).unwrap(), args.get(0).unwrap());
            return;
        }
        Some(value) => {
            let filename = match args.get(2) {
                None => String::from(""),
                Some(filename) => String::from(filename)
            };
            match value.as_str() {
                "1-1" => day1::day1_1(),
                "1-2" => day1::day1_2(),
                "2-1" => day2::day2_1(),
                "2-2" => day2::day2_2(),
                "3-1" => day3::day3_1(),
                "3-2" => day3::day3_2(),
                "4-1" => day4::day4_1(),
                "4-2" => day4::day4_2(),
                "5-1" => day5::day5_1(),
                "5-2" => day5::day5_2(),
                "6-1" => day6::day6_1(),
                "6-2" => day6::day6_2(),
                "7-1" => day7::day7_1(filename),
                "7-2" => day7::day7_2(filename),
                "8-1" => day8::day8_1(filename),
                "8-2" => day8::day8_2(filename),
                _ => panic!("unknown argument was provided")
            }
        }
    };
}