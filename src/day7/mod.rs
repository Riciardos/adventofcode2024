use crate::day7::Operator::{Add, Multiply};
use crate::util::read_lines;

mod day7 {}

pub fn day7() {
    if let Ok(lines) = read_lines("src/day7/input.txt") {
        let mut correct_results = vec![];
        for string_result in lines {
            if let Ok(line) = string_result {
                let (answer, operands) = line.split_once(": ")
                    .map(|x| {
                        let answer = x.0.parse::<i64>().unwrap();
                        let operands = x.1.split(" ").map(|x| x.parse::<i64>().unwrap()).collect::<Vec<i64>>();
                        return (answer, operands)
                    })
                    .unwrap();

                if traverse_tree(answer, *operands.get(0).unwrap(), Add, &operands, 0 )
                    || traverse_tree(answer, *operands.get(0).unwrap(), Multiply, &operands, 0 ) {
                    correct_results.push(answer);
                }
            }
        }
        print!("Day 7 Part 1: {}\n", correct_results.iter().fold(0, |acc, answer| acc + answer));
    }
}

fn traverse_tree(answer: i64, val: i64, operator: Operator, operands: &Vec<i64>, depth: usize) -> bool {
    if depth == operands.len() {
        return answer == match operator  {
            Add => val + operands.get(depth).unwrap_or(&0),
            Multiply => val * operands.get(depth).unwrap_or(&1)
        }
    }
    let new_val_add = val + operands.get(depth + 1 ).unwrap_or(&0);
    let new_val_mul = val * operands.get(depth + 1).unwrap_or(&1);
    traverse_tree(answer, new_val_add,  Add,  operands, depth + 1) || traverse_tree(answer, new_val_mul, Multiply, operands, depth + 1)
}

enum Operator {
    Multiply,
    Add
}