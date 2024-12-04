mod day3 {}

use regex::Regex;

use super::util::read_lines;

pub fn day3() {

    if let Ok(lines) =  read_lines("src/day3/input.txt") {

        let regex = Regex::new(r"(mul\((\d{1,3}),(\d{1,3})\))").unwrap();
        let mut total = 0;
        for line in lines {
            if let Ok(s) = line {
                let mut results = vec![];
                for (_, [mul, left, right]) in regex.captures_iter(&*s).map(|x| x.extract()) {
                    results.push([mul, left, right]);
                }
                for result in results {
                    total +=  result[1].parse::<i32>().unwrap() * result[2].parse::<i32>().unwrap();
                }
            }
        }
        println!("Day 3 : {:?}", total);
    }

}

pub fn day3_part2() {

    if let Ok(lines) =  read_lines("src/day3/input.txt") {

        let regex = Regex::new(r"(mul\(\d{1,3},\d{1,3}\))|(do\(\))|(don't\(\))").unwrap();
        let mut total = 0;
        let mut should_add = true;

        for line in lines {
            if let Ok(s) = line {
                let mut results = vec![];
                for (_, [instruction]) in regex.captures_iter(&*s).map(|x| x.extract()) {
                    results.push(instruction);
                }
                let regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
                for result in results {
                    if result == "don't()" {
                        should_add = false;
                        continue;
                    }
                    if result == "do()" {
                        should_add = true;
                        continue;
                    }
                    if should_add {
                        let caps = regex.captures(result).unwrap();
                        total += caps.get(1).unwrap().as_str().parse::<i32>().unwrap() * caps.get(2).unwrap().as_str().parse::<i32>().unwrap();

                    }
                }

            }
        }
        println!("Day 3 part 2: {:?}", total);
    }

}