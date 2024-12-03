mod day1 {}

use std::collections::HashMap;
use super::util::read_lines;

pub fn day1() {
    // read line by line
    if let Ok(lines) = read_lines("src/day1/input.csv") {

        let mut total_distance = 0;
        let mut left_array = Vec::<i32>::new();
        let mut right_array = Vec::<i32>::new();

        let numbers = lines;
        for line in numbers {
            if let Ok(data) = line {
                let mut split = data.split_whitespace();
                left_array.push(split.next().unwrap().parse::<i32>().unwrap());
                right_array.push(split.next().unwrap().parse::<i32>().unwrap());
            }
        }

        left_array.sort();
        right_array.sort();

        for i in 0..left_array.len() {
            total_distance +=  (right_array[i] - left_array[i]).abs();
        }

        println!("Part 1: {}", total_distance);
    }
}

pub fn day1_star2() {
    // read line by line
    if let Ok(lines) = read_lines("src/day1/input.csv") {

        let mut total_similarity = 0;
        let mut left_array = Vec::<i32>::new();
        let mut right_array = Vec::<i32>::new();

        let numbers = lines;
        for line in numbers {
            if let Ok(data) = line {
                let mut split = data.split_whitespace();
                left_array.push(split.next().unwrap().parse::<i32>().unwrap());
                right_array.push(split.next().unwrap().parse::<i32>().unwrap());
            }
        }

        let occurences = right_array.iter().fold(HashMap::new(), |mut acc, x| {
            *acc.entry(x).or_insert(0) += 1;
            acc
        });

        for i in 0..left_array.len() {
            total_similarity +=  left_array[i] * occurences.get(&(left_array[i])).unwrap_or(&0);
        }

        println!("Day 1 part 2: {}", total_similarity);
    }
}