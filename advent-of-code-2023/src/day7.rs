use std::time::Instant;

use crate::util::read_file;

pub fn day7_1(filename: String) {
    let mut filename_l = filename;
    if filename_l.is_empty() {
        filename_l = String::from("inputs/day7.txt");
    }   
    let _file = read_file(&filename_l);

    let time = Instant::now();
    print!("the result is {} [time taken {}ms]", 0, time.elapsed().as_millis());
}

pub fn day7_2(filename: String) {
    let mut filename_l = filename;
    if filename_l.is_empty() {
        filename_l = String::from("inputs/day7.txt");
    }   
    let _file = read_file(&filename_l);

    let time = Instant::now();
    print!("the result is {} [time taken {}ms]", 0, time.elapsed().as_millis());
}
