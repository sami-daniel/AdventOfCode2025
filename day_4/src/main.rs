use std::{env::args, fs::File, io::Read};

use num_enum::TryFromPrimitive;

#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
enum PossiblePositions {
    Up,
    Down,
    Left,
    Right,
    UpLeft,
    UpRight,
    DownLeft,
    DownRight
}

type Position = (usize, usize);
type Map = Vec<Vec<char>>;

fn main() {
    let args: Vec<String> = args().collect();
    let mut file_buffer = String::new();
    File::open(args.get(1).unwrap()).unwrap().read_to_string(&mut file_buffer).unwrap();
    let map: Map = file_buffer[..].split('\n').map(|l| l.chars().collect()).collect();
    let mut accessibles = 0;
    
    process_map(map, &mut accessibles);

    println!("Accessible: {accessibles}");
}

fn process_map(map: Map, accessibles: &mut i32) {
    let accessibles_copy = *accessibles;
    let mut new_map = Map::new();
    _ = map.iter().map(|_| new_map.push(vec![])).collect::<Vec<_>>();

    for (i, row) in map.iter().enumerate() {
        let column_enumerator = row.iter().enumerate();
        for (j, char) in column_enumerator {
            let pos: Position = (i, j);
            if check_if_accessible(pos, *char, &map) {
                new_map[i].push('.');
                *accessibles += 1;
            } else {
                new_map[i].push(*char);
            }
        }
    }

    if *accessibles == accessibles_copy {
        return;
    }

    process_map(new_map, accessibles);
}

fn check_if_accessible(pos: Position, c: char, map: &[Vec<char>]) -> bool {
    if c != '@' {
        return false;
    }
    
    let mut total_rolls_around = 0;
    for i in 0..8 {
        let element = get_element_at(&pos, PossiblePositions::try_from(i).unwrap(), map);
        match element {
            Some('@') => total_rolls_around += 1,
            _ => {}
        };
    }

    if total_rolls_around >= 4 {
        return false;
    }

    return true;
}

fn get_element_at(actual_pos: &Position, expected_pos: PossiblePositions, map: &[Vec<char>]) -> Option<char> {
    fn get_at(row: usize, column: usize, map: &[Vec<char>]) -> Option<char> {
        let row = map.get(row)?;
        Some(*row.get(column)?)
    }

    let (x, y) = *actual_pos;
    match expected_pos {
        PossiblePositions::Up => {
            if x == 0 {
                return None;
            }
            get_at(x - 1, y, map)
        },
        PossiblePositions::Down => get_at(x + 1, y, map),
        PossiblePositions::Left => {
            if y == 0 {
                return None;
            }

            get_at(x, y - 1, map)
        },
        PossiblePositions::Right => get_at(x, y + 1, map),
        PossiblePositions::UpLeft => {
            if x == 0 || y == 0 {
                return None;
            }

            get_at(x - 1, y - 1, map)
        },
        PossiblePositions::UpRight => {
            if x == 0 {
                return None;
            }

            get_at(x - 1, y + 1, map)
        },
        PossiblePositions::DownLeft => {
            if y == 0 {
                return None;
            }

            get_at(x + 1, y - 1, map)
        },
        PossiblePositions::DownRight => get_at(x + 1, y + 1, map),
    }
}