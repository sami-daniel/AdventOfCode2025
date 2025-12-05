use std::{collections::HashSet, env::args, fs::File, i32, io::Read, ops::{RangeBounds, RangeInclusive}};

fn main() {
    let args: Vec<String> = args().collect();
    let mut file_buffer = String::new();

    File::open(args.get(1).unwrap()).unwrap().read_to_string(&mut file_buffer).unwrap();
    let total_lines: Vec<&str> = file_buffer[..].split('\n').collect();
    let div_idx = total_lines.iter().position(|l| *l == "").expect("No separation line available");
    let (ranges_str, _) = total_lines.split_at(div_idx);
    let mut ranges = vec![];

    for range_str in ranges_str {
        add_part(&range_str, &mut ranges);
    }

    println!("Total: {}", total)
}

fn add_part(str: &str, ranges: &mut Vec<RangeInclusive<usize>>) {
    let range_parts = str.split('-').map(|p| p.parse::<usize>().unwrap()).take(2).collect::<Vec<usize>>();
    let new_range = range_parts[0]..=range_parts[1];

    for range in ranges.clone() {
        if new_range.start() < range.start() {
            ranges.push(*new_range.start()..=*range.start());
        }

        if new_range.end() < range.end() {
            ranges.push(*new_range.end()..=*range.end());
        }
    }

}