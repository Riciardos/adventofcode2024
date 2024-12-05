mod util;

mod day4 {}

use super::util::read_lines;
use util::*;

pub fn day4() {
    if let Ok(lines) = read_lines("src/day4/input.txt") {
        let mut x_axis = vec![];
        let mut y_size = 0;
        for line in lines {
            if let Ok(s) = line {
                let y_axis = s.chars().collect::<Vec<char>>();
                y_size = y_axis.len();
                x_axis.push(y_axis);
            }
        }
        let mut grid = vec![];
        for i in 0..x_axis.len() {
            for j in 0..x_axis.get(i).unwrap().len() {
                grid.push(Point {
                    x: i,
                    y: j,
                    letter: *x_axis.get(i).unwrap().get(j).unwrap(),
                    found: false,
                })
            }
        }
        let mut total_found = 0;

        let grid_clone = grid.clone();
        for point in grid.iter_mut(){
            if look_right(&grid_clone, point.x, y_size, point.y) {
                point.found = true;
                total_found += 1
            };
            if look_left(&grid_clone, point.x, y_size, point.y) {
                point.found = true;
                total_found += 1
            };
            if look_down(&grid_clone, point.x, y_size, point.y) {
                point.found = true;
                total_found += 1
            };
            if look_up(&grid_clone, point.x, y_size, point.y) {
                point.found = true;
                total_found += 1
            };
            if look_down_right(&grid_clone, point.x, y_size, point.y) {
                point.found = true;
                total_found += 1
            };
            if look_down_left(&grid_clone, point.x, y_size, point.y) {
                point.found = true;
                total_found += 1
            };
            if look_up_right(&grid_clone, point.x, y_size, point.y) {
                point.found = true;
                total_found += 1
            };
            if look_up_left(&grid_clone, point.x, y_size, point.y) {
                point.found = true;
                total_found += 1
            };
        }

        for i in 0..x_axis.len() {
            for j in 0..y_size {
                let point = grid.get(i * y_size + j).unwrap();
                if point.found {
                    print!("{}", point.letter)
                } else {
                    print!(".")
                }
            }
            println!()
        }
        println!("Day 4 Part 1: {:?}", total_found);
    }
}

// fn mark_found(mut grid: Vec<Point>, y_size: usize) {
//     for i in 0..grid.len() / y_size {
//         for j in 0..y_size {
//
//         }
//     }
//     grid.iter_mut().for_each(|point| {
//         if point.found {
//             let mut point = grid.get().unwrap();
//             *point.found = true;
//
//         }
//     })
// }