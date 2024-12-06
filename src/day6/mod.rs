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

        for x in 0..x_index - 0 {
            for y in 0..y_index - 0 {
                let grid_point = grid.get(x * x_index + y).unwrap();
                if grid_point.is_tree {
                    print!("#");
                } else if grid_point.is_edge {
                    print!("x");
                } else if grid_point.visited {
                    print!("X");
                } else {
                    print!(".");
                }
            }
            println!();
        }
        println!(
            "Day 6 Part 1: {}",
            grid.iter().filter(|p| p.visited).count()
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

enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}
