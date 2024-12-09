use std::collections::HashMap;
use crate::util::read_lines;

mod day8 {}

pub fn day8() {
    let mut grid = vec![];
    let mut index = 0;
    let mut x_size = 0;
    let mut y_size = 0;
    if let Ok(lines) = read_lines("src/day8/input.txt") {
        for line in lines {
            x_size += 1;
            if let Ok(s) = line {
                y_size = 0;
                for c in s.chars() {
                    index += 1;
                    y_size += 1;
                    grid.push(GridPoint {
                        x: x_size,
                        y: y_size,
                        index: index,
                        is_antenna: !c.eq(&'.'),
                        is_anti_node: false,
                        char: c,
                    })
                }
            }
        }
    }

    let mut antenna_map: HashMap::<char, Vec<&GridPoint>> = HashMap::new();
    let grid_clone = grid.clone();
    antenna_map = grid_clone.iter().fold(HashMap::new(), |mut acc, point| {
        if point.char != '.' {
            acc.entry(point.char).or_insert(vec![]).push(point);
        }
        return acc;
    });

    mark_anti_nodes(&mut grid, &antenna_map, x_size, y_size);

    for x in 0..x_size {
        for y in 0..y_size {
            print!("{}", if grid[x*y_size + y].is_anti_node {
                '#'
            } else {
                grid[x*y_size + y].char
            })
        }
        println!()
    }

    println!("Day 8 part 1 {:?}", grid.iter().filter(|p| p.is_anti_node || p.is_antenna).count());
}

fn mark_anti_nodes(grid: &mut Vec<GridPoint>, antenna_map: &HashMap<char, Vec<&GridPoint>>, x_size :usize, y_size :usize) {
    antenna_map.iter().for_each(|(antenna_id, antennas )| {
        let x_max = x_size as i32;
        let y_max = y_size as i32;

        for x in 0..antennas.len() - 1 {
            for y in x+1..antennas.len() {
                let point_one = antennas[x];
                let point_two = antennas[y];
                let x_diff = point_two.x as i32 - point_one.x as i32;
                let y_diff = point_two.y as i32 - point_one.y as i32;

                let mut distance_counter = 2;

                let mut point_three_x = point_one.x as i32 + distance_counter * x_diff;
                let mut point_three_y = point_one.y as i32 + distance_counter * y_diff;
                let mut point_four_x = point_two.x as i32 - distance_counter * x_diff;
                let mut point_four_y = point_two.y as i32 - distance_counter * y_diff;
                let mut index = (point_three_x - 1) * x_max + point_three_y - 1;
                let mut index_two = (point_four_x - 1) * x_max + point_four_y - 1;

                while !is_coord_out_of_grid(point_three_x, point_three_y, x_max, y_max) ||
                    !is_coord_out_of_grid(point_four_x, point_four_y, x_max, y_max) {

                    point_three_x = point_one.x as i32 + distance_counter * x_diff;
                    point_three_y = point_one.y as i32 + distance_counter * y_diff;
                    point_four_x = point_two.x as i32 - distance_counter * x_diff;
                    point_four_y = point_two.y as i32 - distance_counter * y_diff;
                    index = (point_three_x - 1) * x_max + point_three_y - 1;
                    index_two = (point_four_x - 1) * x_max + point_four_y - 1;

                    if !is_coord_out_of_grid(point_three_x, point_three_y, x_max, y_max) {
                        grid[index as usize].is_anti_node = true;
                    }
                    if !is_coord_out_of_grid(point_four_x, point_four_y, x_max, y_max) {
                        grid[index_two as usize].is_anti_node = true;
                    }
                    distance_counter +=1;
                }
            }
        }
    })
}

fn is_coord_out_of_grid(x_coord: i32, y_coord: i32, x_max: i32, y_max: i32) -> bool {
    x_coord <= 0 || y_coord <= 0 || x_coord > x_max || y_coord > y_max
}

#[derive(Debug, Clone)]
struct GridPoint {
    x: usize,
    y: usize,
    index: usize,
    is_antenna: bool,
    is_anti_node: bool,
    char: char
}