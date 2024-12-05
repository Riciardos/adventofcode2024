use crate::util::read_lines;
use std::cmp::Ordering;

mod day5 {}

pub fn day5() {
    if let Ok(rules) = read_lines("src/day5/input_rules.txt") {
        if let Ok(updates) = read_lines("src/day5/input_updates.txt") {
            let rules_vec: Vec<(i32, i32)> = rules
                .into_iter()
                .map(|x| x.unwrap())
                .map(|x| {
                    let y = x.split_once("|").unwrap().clone();
                    return (y.0.parse::<i32>().unwrap(), y.1.parse::<i32>().unwrap());
                })
                .collect();
            let updates_vec: Vec<Vec<i32>> = updates
                .into_iter()
                .map(|x| {
                    x.unwrap()
                        .split(",")
                        .map(|x| x.parse::<i32>().unwrap())
                        .collect()
                })
                .collect();

            let mut correct_updates = vec![];

            updates_vec.iter().for_each(|update| {
                if check_rules(update, &rules_vec) {
                    correct_updates.push(update.clone());
                }
            });

            let total = correct_updates.iter().fold(0, |acc, update| {
                let middle_number = *update.get(update.len() / 2).unwrap();
                acc + middle_number
            });

            println!("Day 5 Part 1 : {}", total);
        }
    }
}

pub fn day5_star2() {
    if let Ok(rules) = read_lines("src/day5/input_rules.txt") {
        if let Ok(updates) = read_lines("src/day5/input_updates.txt") {
            let rules_vec: Vec<(i32, i32)> = rules
                .into_iter()
                .map(|x| x.unwrap())
                .map(|x| {
                    let y = x.split_once("|").unwrap().clone();
                    return (y.0.parse::<i32>().unwrap(), y.1.parse::<i32>().unwrap());
                })
                .collect();
            let updates_vec: Vec<Vec<i32>> = updates
                .into_iter()
                .map(|x| {
                    x.unwrap()
                        .split(",")
                        .map(|x| x.parse::<i32>().unwrap())
                        .collect()
                })
                .collect();

            let mut incorrect_updates = vec![];

            updates_vec.iter().for_each(|update| {
                if !check_rules(update, &rules_vec) {
                    incorrect_updates.push(update.clone());
                }
            });

            incorrect_updates.iter_mut().for_each(|update| {
                bubble_sort_till_correct(update, &rules_vec);
            });

            let total = incorrect_updates.iter().fold(0, |acc, update| {
                let middle_number = *update.get(update.len() / 2).unwrap();
                acc + middle_number
            });

            println!("Day 5 Part 2 : {}", total);
        }
    }
}

fn bubble_sort_till_correct(update: &mut Vec<i32>, rules: &Vec<(i32, i32)>) {
    update.sort_by(|a, b| {
        if rules.iter().filter(|x| a.eq(&x.0) && b.eq(&x.1)).count() == 1 {
            return Ordering::Less;
        }
        if rules.iter().filter(|x| b.eq(&x.0) && a.eq(&x.1)).count() == 1 {
            return Ordering::Greater;
        }
        return Ordering::Equal;
    });
}

fn check_rules(update: &Vec<i32>, rules: &Vec<(i32, i32)>) -> bool {
    // we assume the update is correct unless a rule_n2 is found before rule_n1
    for i in 0..update.len() {
        let number = update[i];
        let rules_that_apply = rules
            .into_iter()
            .filter(|x| x.0 == number)
            .collect::<Vec<&(i32, i32)>>();
        for x in 0..rules_that_apply.len() {
            for j in 0..i {
                if rules_that_apply[x].1 == update[j] {
                    return false;
                }
            }
        }
    }
    true
}
