use std::env;

mod util;
mod day1;

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.get(1) {
        None => panic!("no arguments provided"),
        Some(value) => {
            match value.as_str() {
                "1-1" => day1::day1_1(),
                "1-2" => day1::day1_2(),
                _ => panic!("unknown argument was provided")
            }
        }
    };
}