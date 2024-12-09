use crate::util::read_lines;

mod day9 {}

pub fn day9() {
    if let Ok(lines) = read_lines("src/day9/input.txt") {
        if let Some(line) = lines.last() {
            let result = decompress(line.unwrap());

            let result = reorder_data(result);

            let result_checksum = calculate_checksum(result);
            println!("Day 9 part 1: {:?}", result_checksum);
        }
    }
}


fn reorder_data(data : Vec<FileBlock>) -> Vec<FileBlock> {
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
        let value = compressed_string.chars().nth(i).unwrap().to_digit(10).unwrap();
        for j in 0..value as usize {
            decompressed.push(FileBlock {
                id: i as i64,
                has_data: i % 2 == 0,
                value: if i%2 == 0 { Some((i / 2) as i64) } else { None } ,
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
    value: Option<i64>,
}