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
mod day9;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;

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
                "9-1" => day9::day9_1(filename),
                "9-2" => day9::day9_2(filename),
                "10-1" => day10::day10_1(filename),
                "10-2" => day10::day10_2(filename),
                "11-1" => day11::day11_1(filename),
                "11-2" => day11::day11_2(filename),
                "12-1" => day12::day12_1(filename),
                "12-2" => day12::day12_2(filename),
                "13-1" => day13::day13_1(filename),
                "13-2" => day13::day13_2(filename),
                "14-1" => day14::day14_1(filename),
                "14-2" => day14::day14_2(filename),
                "15-1" => day15::day15_1(filename),
                "15-2" => day15::day15_2(filename),
                "16-1" => day16::day16_1(filename),
                "16-2" => day16::day16_2(filename),
                _ => panic!("unknown argument was provided")
            }
        }
    };
}