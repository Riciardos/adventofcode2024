use std::cmp::PartialEq;
use std::collections::HashMap;
use crate::util::read_lines;

use crate::day6::Direction::{DOWN, LEFT, RIGHT, UP};

mod day6 {}

pub fn day6() {
    if let Ok(lines) = read_lines("src/day6/input.txt") {
        let mut grid = vec![];
        let mut x_index = 0;
        let mut y_index = 0;
        let x_size = 132;
        let y_size = 132;
        let guard_start_x = 44;
        let guard_start_y = 80;

        for line in lines {
            y_index = 0;
            for ch in line.unwrap().chars() {
                grid.push(GridPoint {
                    x: x_index,
                    y: y_index,
                    is_tree: ch == '#',
                    is_edge: ch == 'x',
                    visited: false,
                    index: grid.len(),
                    point_up: if x_index > 0 && x_index < x_size {
                        (x_index - 1) * x_size + y_index
                    } else {
                        0
                    },
                    point_down: if x_index > 0 && x_index < x_size {
                        (x_index + 1) * x_size + y_index
                    } else {
                        0
                    },
                    point_left: if y_index > 0 && y_index < y_size {
                        x_index * x_size + y_index - 1
                    } else {
                        0
                    },
                    point_right: if y_index > 0 && y_index < y_size {
                        x_index * x_size + y_index + 1
                    } else {
                        0
                    },
                });
                y_index += 1;
            }
            x_index += 1;
        }

        let mut guard = Guard {
            x: guard_start_x,
            y: guard_start_y,
            current_grid_point: guard_start_x * x_size + guard_start_y,
            direction: UP,
            found_edge: false,
        };

        let mut current_gridpoint_index = guard.current_grid_point;
        grid.get_mut(current_gridpoint_index).unwrap().visited = true;

        while !guard.found_edge {
            let next_gridpoint_index = match guard.direction {
                UP => grid.get(current_gridpoint_index).unwrap().point_up,
                DOWN => grid.get(current_gridpoint_index).unwrap().point_down,
                LEFT => grid.get(current_gridpoint_index).unwrap().point_left,
                RIGHT => grid.get(current_gridpoint_index).unwrap().point_right,
            };
            let next_gridpoint = grid.get_mut(next_gridpoint_index).unwrap();
            if next_gridpoint.is_edge {
                guard.found_edge = true;
                break;
            }
            if next_gridpoint.is_tree {
                guard.direction = get_next_direction(guard.direction);
                continue;
            }
            next_gridpoint.visited = true;
            current_gridpoint_index = next_gridpoint_index;
        }

        // for x in 0..x_index - 0 {
        //     for y in 0..y_index - 0 {
        //         let grid_point = grid.get(x * x_index + y).unwrap();
        //         if grid_point.is_tree {
        //             print!("#");
        //         } else if grid_point.is_edge {
        //             print!("x");
        //         } else if grid_point.visited {
        //             print!("X");
        //         } else {
        //             print!(".");
        //         }
        //     }
        //     println!();
        // }
        println!(
            "Day 6 Part 1: {}",
            grid.iter().filter(|p| p.visited).count()
        );
    }
}

pub fn day6_part2() {
    if let Ok(lines) = read_lines("src/day6/input.txt") {
        let mut grid = vec![];
        let mut x_index = 0;
        let mut y_index = 0;
        let x_size = 132;
        let y_size = 132;
        let guard_start_x = 44;
        let guard_start_y = 80;

        for line in lines {
            y_index = 0;
            for ch in line.unwrap().chars() {
                grid.push(GridPoint {
                    x: x_index,
                    y: y_index,
                    is_tree: ch == '#',
                    is_edge: ch == 'x',
                    visited: false,
                    index: grid.len(),
                    point_up: if x_index > 0 && x_index < x_size {
                        (x_index - 1) * x_size + y_index
                    } else {
                        0
                    },
                    point_down: if x_index > 0 && x_index < x_size {
                        (x_index + 1) * x_size + y_index
                    } else {
                        0
                    },
                    point_left: if y_index > 0 && y_index < y_size {
                        x_index * x_size + y_index - 1
                    } else {
                        0
                    },
                    point_right: if y_index > 0 && y_index < y_size {
                        x_index * x_size + y_index + 1
                    } else {
                        0
                    },
                });
                y_index += 1;
            }
            x_index += 1;
        }

        let mut loops_found = 0;

        for grid_point in &grid {

            let mut visited_tree_points:HashMap<usize, Vec<Direction>> = HashMap::new();
            let mut guard = Guard {
                x: guard_start_x,
                y: guard_start_y,
                current_grid_point: guard_start_x * x_size + guard_start_y,
                direction: UP,
                found_edge: false,
            };
            // get grid lone and set new 'block' tree
            let mut grid_clone = grid.clone();
            grid_clone.get_mut(grid_point.index).unwrap().is_tree = true;

            let mut current_gridpoint_index = guard.current_grid_point;
            grid_clone.get_mut(current_gridpoint_index).unwrap().visited = true;

            let mut loop_found = false;
            while !guard.found_edge && !loop_found {
                // get next gridpoint
                let next_gridpoint_index = match guard.direction {
                    UP => grid_clone.get(current_gridpoint_index).unwrap().point_up,
                    DOWN => grid_clone.get(current_gridpoint_index).unwrap().point_down,
                    LEFT => grid_clone.get(current_gridpoint_index).unwrap().point_left,
                    RIGHT => grid_clone.get(current_gridpoint_index).unwrap().point_right,
                };

                let next_gridpoint = grid_clone.get_mut(next_gridpoint_index).unwrap();

                if next_gridpoint.is_edge {
                    guard.found_edge = true;
                    break;
                }
                if next_gridpoint.is_tree {
                    if let Some(visited_tree) = visited_tree_points.get(&next_gridpoint.index) {
                        if visited_tree.iter().filter(|dir| **dir == guard.direction).count() == 1 {
                            loop_found = true;
                            loops_found += 1;
                        } else {
                            visited_tree_points.get_mut(&next_gridpoint.index).unwrap().push(guard.direction.clone());
                        }
                    } else {
                        visited_tree_points.insert(next_gridpoint.index.clone(), vec![]);
                    }
                    guard.direction = get_next_direction(guard.direction);
                } else {
                    next_gridpoint.visited = true;
                    current_gridpoint_index = next_gridpoint_index;
                }
            }
        }

        println!(
            "Day 6 Part 2: {}",
            loops_found
        );
    }
}


fn get_next_direction(current_direction: Direction) -> Direction {
    match current_direction {
        UP => RIGHT,
        RIGHT => DOWN,
        DOWN => LEFT,
        LEFT => UP,
    }
}

#[derive(Debug, Clone)]
struct GridPoint {
    x: usize,
    y: usize,
    is_tree: bool,
    is_edge: bool,
    visited: bool,
    index: usize,
    point_up: usize,
    point_down: usize,
    point_left: usize,
    point_right: usize,
}

struct Guard {
    x: usize,
    y: usize,
    current_grid_point: usize,
    direction: Direction,
    found_edge: bool,
}

#[derive(PartialEq, Copy, Clone, Debug)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}
