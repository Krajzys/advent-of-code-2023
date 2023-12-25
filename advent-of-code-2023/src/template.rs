use std::time::Instant;

use crate::util::read_file;

pub fn dayXX_1(filename: String) {
    let mut filename_l = filename;
    if filename_l.is_empty() {
        filename_l = String::from("inputs/dayXX.txt");
    }
    let file = read_file(&filename_l);

    let time = Instant::now();
    let result = 0;
    for line in file.lines() {
        // TODO: Process file lines
    }

    print!("the result is {} [time taken {}ms]", result, time.elapsed().as_millis());
}

pub fn dayXX_2(filename: String) {
    let mut filename_l = filename;
    if filename_l.is_empty() {
        filename_l = String::from("inputs/dayXX.txt");
    }
    let file = read_file(&filename_l);

    let time = Instant::now();
    let result = 0;
    for line in file.lines() {
        // TODO: Process file lines
    }

    print!("the result is {} [time taken {}ms]", result, time.elapsed().as_millis());
}