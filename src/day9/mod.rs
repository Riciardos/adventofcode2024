use crate::util::read_lines;
use std::collections::BTreeMap;
use std::ops::Add;

mod day9 {}

pub fn day9() {
    if let Ok(lines) = read_lines("src/day9/input.txt") {
        if let Some(line) = lines.last() {
            let result = decompress(line.unwrap());
            let reordered_result = reorder_data_single_block(result);
            let result_checksum = calculate_checksum(reordered_result);
            println!("Day 9 part 1: {:?}", result_checksum);
        }
    }
}

pub fn day9_part2() {
    if let Ok(lines) = read_lines("src/day9/input.txt") {
        if let Some(line) = lines.last() {
            let result = decompress(line.unwrap());
            let reordered_result = reorder_data_full_block(result.clone());
            let result_checksum = calculate_checksum(reordered_result);
            println!("Day 9 part 2: {}", result_checksum);
        }
    }
}

fn reorder_data_full_block(data: Vec<FileBlock>) -> Vec<FileBlock> {
    let mut result: Vec<FileBlock> = data.clone();

    // set up map of starting points with data
    let starting_point_map =
        data.iter()
            .enumerate()
            .rfold(BTreeMap::<i64, (usize, i64)>::new(), |mut acc, x| {
                if let Some(value) = x.1.value {
                    acc.entry(value)
                        .and_modify(|v| *v = (x.0, x.1.spaces))
                        .or_insert((x.0, x.1.spaces));
                };
                acc
            });

    // move files from end to beginning if there is space
    starting_point_map.iter().rev().for_each(|x| {
        if let Some(starting_point) = is_space_available(&result, x.1.0, x.1.1) {
            move_file(&mut result, starting_point, x.1.0, x.1.1 as usize, *x.0);
        };
    });
    result
}

fn is_space_available(data: &Vec<FileBlock>, original_index: usize, spaces_needed: i64) -> Option<usize> {
    let mut space_counter = 0;

    for i in 0..original_index {
        if !data[i].has_data {
            space_counter += 1;
            if space_counter == spaces_needed {
                return Some(i + 1 - spaces_needed as usize);
            }
            continue
        } else {
            space_counter = 0;
        }
    }

    None
}


fn move_file(data: &mut Vec<FileBlock>, starting_point: usize, original_start_ponint: usize, number_of_spots: usize, file_value: i64) {
    for i in starting_point..starting_point + number_of_spots {
        data[i].value = Some(file_value);
        data[i].has_data = true;
    }
    for i in original_start_ponint..original_start_ponint + number_of_spots {
        data[i].value = None;
        data[i].has_data = false;
    }
}

fn reorder_data_single_block(data: Vec<FileBlock>) -> Vec<FileBlock> {
    let mut result = vec![];
    let length = data.len();
    let mut j = length.clone() - 1;
    for i in 0..length {
        if i >= j {
            break;
        }
        if data[i].has_data {
            result.push(data[i].clone());
        } else {
            while !data[j].has_data {
                j -= 1;
            }
            if i < j {
                result.push(data[j].clone());
                j -= 1;
            }
        }
    }
    result
}

fn decompress(compressed_string: String) -> Vec<FileBlock> {
    let mut decompressed = vec![];

    for i in 0..compressed_string.len() {
        let value = compressed_string
            .chars()
            .nth(i)
            .unwrap()
            .to_digit(10)
            .unwrap();
        for j in 0..value as usize {
            decompressed.push(FileBlock {
                id: i as i64,
                has_data: i % 2 == 0,
                spaces: value as i64,
                value: if i % 2 == 0 {
                    Some((i / 2) as i64)
                } else {
                    None
                },
            })
        }
    }
    decompressed
}

fn calculate_checksum(disk: Vec<FileBlock>) -> i64 {
    let mut sum = 0;
    for x in 0..disk.len() {
        if disk[x].has_data {
            sum += x as i64 * disk[x].value.unwrap();
        }
    }
    sum
}

#[derive(Debug, Clone)]
struct FileBlock {
    id: i64,
    has_data: bool,
    spaces: i64,
    value: Option<i64>,
}