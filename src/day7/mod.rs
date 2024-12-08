use crate::util::read_lines;

mod day7 {}

pub fn day7() {
    if let Ok(lines) = read_lines("src/day7/input.txt") {
        let mut correct_results = vec![];
        for string_result in lines {
            if let Ok(line) = string_result {
                let (answer, operands) = line
                    .split_once(": ")
                    .map(|x| {
                        let answer = x.0.parse::<i64>().unwrap();
                        let operands =
                            x.1.split(" ")
                                .map(|x| x.parse::<i64>().unwrap())
                                .collect::<Vec<i64>>();
                        return (answer, operands);
                    })
                    .unwrap();

                if traverse_tree(answer, *operands.get(0).unwrap(), &operands, 0)
                {
                    correct_results.push(answer);
                }
            }
        }
        print!(
            "Day 7 Part 2: {}\n",
            correct_results.iter().fold(0, |acc, answer| acc + answer)
        );
    }
}

fn traverse_tree(
    answer: i64,
    val: i64,
    operands: &Vec<i64>,
    depth: usize,
) -> bool {
    if depth == operands.len() {
        return answer == val;
    }

    let mut new_val_add = val;
    let mut new_val_mul = val;
    let mut new_val_concat = val;

    if let Some(next_operand) = operands.get(depth + 1) {
        new_val_add = val + next_operand;
        new_val_mul = val * next_operand;
        new_val_concat = (val.to_string() + &*next_operand.to_string()).parse::<i64>().unwrap();
    }

    traverse_tree(answer, new_val_add, operands, depth + 1)
        || traverse_tree(answer, new_val_mul, operands, depth + 1)
        || traverse_tree(answer, new_val_concat, operands, depth + 1)
}
