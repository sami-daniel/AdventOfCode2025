use std::{fs::File, i32, io::Read, path::{Path, PathBuf}};

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    let mut file_buffer = String::new();
    File::open(args.get(1).expect("Invalid input path.")).unwrap().read_to_string(&mut file_buffer).unwrap();
    let mut lines: Vec<&str> = file_buffer[..].split('\n').collect();

    if lines.last() == Some(&"") {
        lines.pop();
    }

    let mut iter = lines.iter();
    let mut dial = 50;
    let mut hit = 0; 
    while let Some(line) = iter.next() {
        let value = parse_val(line);
        count_zero_crossings(value, &mut dial, &mut hit);
    }
    println!("Hits: {}", hit)
}

fn count_zero_crossings(val: i32, dial: &mut i32, hit: &mut i32) {
    let start = *dial;
    let end = (*dial + val).rem_euclid(100);
    let mut pos = start;
    let step = if val > 0 { 1 } else { -1 };
    let steps = val.abs();
    for _ in 0..steps {
        pos = (pos + step).rem_euclid(100);
        if pos == 0 {
            add_hit(hit);
        }
    }
    *dial = end;
}

fn parse_val(val: &str) -> i32 {
    let chars = val.chars().collect::<Vec<char>>();
    let number = &chars[1..].iter().collect::<String>()[..].parse::<i32>().unwrap();
    let letter = &chars[0];
    match letter {
        'L' => {
            *number * -1
        },
        'R' => {
            *number
        },
        _ => panic!("Unrecognized value on parse_val")
    }
}

fn add_hit(hit: &mut i32) {
    *hit += 1;
}