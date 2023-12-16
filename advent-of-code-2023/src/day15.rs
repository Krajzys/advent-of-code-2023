use std::time::Instant;

use crate::util::read_file;

pub fn day15_1(filename: String) {
    let mut filename_l = filename;
    if filename_l.is_empty() {
        filename_l = String::from("inputs/day15.txt");
    }   
    let file = read_file(&filename_l);
    
    let time = Instant::now();
    let mut result = 0;
    for (_, line) in file.lines().enumerate() {
        for step in line.split(',') {
            let mut val = 0;
            for ch in step.chars() {
                val += ch as i32;
                val *= 17;
                val %= 256;
            }
            result += val;
        }
    }

    print!("the result is {} [time taken {}ms]", result, time.elapsed().as_millis());
}

pub fn day15_2(filename: String) {
    let mut filename_l = filename;
    if filename_l.is_empty() {
        filename_l = String::from("inputs/day15.txt");
    }   
    let file = read_file(&filename_l);
    
    let time = Instant::now();
    let mut result = 0;
    let mut boxes: Vec<Vec<(String, usize)>> = Vec::new();
    for _ in 0..256 {
        boxes.push(Vec::new());
    }

    for (_, line) in file.lines().enumerate() {
        for step in line.split(',') {
            let mut box_no: usize = 0;
            let op_pos = step.find(|c| c == '=' || c == '-').unwrap();
            for ch in step.chars().take(op_pos) {
                box_no += ch as usize;
                box_no *= 17;
                box_no %= 256;
            }
            match step.chars().nth(op_pos).unwrap() {
                '=' => {
                    let mut label_lens_str = step.split('=');
                    let label = label_lens_str.next().unwrap();
                    let lens_len: usize = label_lens_str.next().unwrap().parse().unwrap();
                    match boxes[box_no].iter().position(|el| el.0 == label) {
                        Some(pos) => {
                            let lens = boxes[box_no].get_mut(pos).unwrap();
                            *lens = (String::from(label), lens_len);
                        },
                        None => {
                            boxes[box_no].push((String::from(label), lens_len));
                        }
                    }
                },
                '-' => {
                    let mut label = step.split('-');
                    let label = label.next().unwrap();
                    match boxes[box_no].iter().position(|el| el.0 == label) {
                        Some(pos) => {
                            boxes[box_no].remove(pos);
                        },
                        None => {}
                    };
                },
                _ => {}
            }
        }
    }


    for (i, single_box) in boxes.iter().enumerate() {
        for (j, lens) in single_box.iter().enumerate() {
            result += (i+1)*(j+1)*lens.1;
        }
    }

    print!("the result is {} [time taken {}ms]", result, time.elapsed().as_millis());
}

fn _show_boxes(boxes: &Vec<Vec<(String, usize)>>) {
    for (i, a_box) in boxes.iter().enumerate() {
        if a_box.is_empty() {
            continue;
        }
        print!("Box {i}: ");
        for pair in a_box {
            print!("[{} {}] ", pair.0, pair.1);
        }
        println!();
    }
}