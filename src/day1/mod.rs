mod day1 {}

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