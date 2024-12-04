mod day2 {}

use super::util::read_lines;


pub fn day2() {
    if let Ok(lines) = read_lines("src/day2/input.csv") {

        let mut valid = 0;
        for line in lines {
            if let Ok(data) = line {
                let split = data.split_whitespace();
                let mut array = Vec::<i32>::new();
                for num in split {
                    array.push(num.parse::<i32>().unwrap());
                }

                let mut is_valid_up = true;
                let mut is_valid_down = true;
                for i in 1..array.len() {
                    if array[i] < array[i-1] || ((array[i] - array[i -1]).abs() < 1 || (array[i] - array[i-1]).abs() > 3) {
                        is_valid_up = false;
                        break;
                    }
                }
                for i in 1..array.len() {
                    if array[i] > array[i-1] || ((array[i] - array[i -1 ]).abs() < 1 || (array[i] - array[i-1]).abs() > 3) {
                        is_valid_down = false;
                        break;
                    }
                }

                if (is_valid_up) || (is_valid_down) {
                    valid += 1;
                }
            }
        }
        println!("Day 2: {}", valid);
    }
}