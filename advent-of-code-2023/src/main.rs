use std::env;

mod util;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.get(1) {
        None => panic!("no arguments provided"),
        Some(value) => {
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
                _ => panic!("unknown argument was provided")
            }
        }
    };
}