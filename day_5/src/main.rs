use std::{env::args, fs::File, io::Read, ops::RangeInclusive};

fn main() {
    let args: Vec<String> = args().collect();
    let mut file_buffer = String::new();

    File::open(args.get(1).unwrap()).unwrap().read_to_string(&mut file_buffer).unwrap();
    let total_lines: Vec<&str> = file_buffer[..].split('\n').filter(|l| !l.is_empty()).collect();
    let mut ranges = vec![];

    for range_str in total_lines {
        add_part(&range_str, &mut ranges);
    }
    
    let total: usize = ranges.iter().map(|r| r.end() - r.start() + 1).sum();
    println!("Total: {}", total);
}

fn add_part(str: &str, ranges: &mut Vec<RangeInclusive<usize>>) {
    let range_parts = str.split('-').map(|p| p.parse::<usize>().unwrap()).take(2).collect::<Vec<usize>>();
    let new_start = range_parts[0];
    let new_end = range_parts[1];

    if ranges.is_empty() {
        ranges.push(new_start..=new_end);
        return;
    }

    let mut to_add = vec![(new_start, new_end)];
    
    for existing in ranges.iter() {
        let mut next_to_add = vec![];
        
        for (start, end) in to_add {
            let ex_start = *existing.start();
            let ex_end = *existing.end();

            if end < ex_start || start > ex_end {
                next_to_add.push((start, end));
            } else {
                if start < ex_start {
                    next_to_add.push((start, ex_start - 1));
                }
                
                if end > ex_end {
                    next_to_add.push((ex_end + 1, end));
                }
            }
        }
        
        to_add = next_to_add;
    }
    
    for (start, end) in &to_add {
        ranges.push(*start..=*end);
    }
}