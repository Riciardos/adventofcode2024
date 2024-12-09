use crate::util::read_lines;
use std::collections::HashMap;

mod day8 {}

pub fn day8() {
    let mut grid = Grid {
        x_max: 0,
        y_max: 0,
        grid_points: vec![],
    };
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
                    grid.grid_points.push(GridPoint {
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

    grid.x_max = x_size ;
    grid.y_max = y_size ;

    let mut antenna_map: HashMap<char, Vec<&GridPoint>> = HashMap::new();
    let grid_clone = grid.grid_points.clone();
    antenna_map = grid_clone.iter().fold(HashMap::new(), |mut acc, point| {
        if point.char != '.' {
            acc.entry(point.char).or_insert(vec![]).push(point);
        }
        return acc;
    });

    mark_anti_nodes(&mut grid, &antenna_map);

    for x in 0..x_size {
        for y in 0..y_size {
            print!(
                "{}",
                if grid.grid_points[x as usize * y_size as usize + y as usize].is_anti_node {
                    '#'
                } else {
                    grid.grid_points[x as usize * y_size as usize + y as usize].char
                }
            )
        }
        println!()
    }

    println!(
        "Day 8 part 2 {:?}",
        grid.grid_points
            .iter()
            .filter(|p| p.is_anti_node || p.is_antenna)
            .count()
    );
}

fn mark_anti_nodes(grid: &mut Grid, antenna_map: &HashMap<char, Vec<&GridPoint>>) {
    antenna_map.iter().for_each(|(antenna_id, antennas)| {
        let x_max = grid.x_max;
        let y_max = grid.y_max;

        for x in 0..antennas.len() - 1 {
            for y in x + 1..antennas.len() {
                let point_one = antennas[x];
                let point_two = antennas[y];
                let x_diff = point_two.x  - point_one.x ;
                let y_diff = point_two.y  - point_one.y ;

                let mut distance_counter = 2;

                let mut point_three_x = point_one.x + distance_counter * x_diff;
                let mut point_three_y = point_one.y  + distance_counter * y_diff;
                let mut point_four_x = point_two.x  - distance_counter * x_diff;
                let mut point_four_y = point_two.y  - distance_counter * y_diff;
                let mut index;
                let mut index_two;

                while !is_coord_out_of_grid(point_three_x, point_three_y, x_max, y_max)
                    || !is_coord_out_of_grid(point_four_x, point_four_y, x_max, y_max)
                {
                    point_three_x = point_one.x  + distance_counter * x_diff;
                    point_three_y = point_one.y  + distance_counter * y_diff;
                    point_four_x = point_two.x  - distance_counter * x_diff;
                    point_four_y = point_two.y  - distance_counter * y_diff;
                    index = (point_three_x - 1) * x_max + point_three_y - 1;
                    index_two = (point_four_x - 1) * x_max + point_four_y - 1;

                    if !is_coord_out_of_grid(point_three_x, point_three_y, x_max, y_max) {
                        grid.grid_points[index as usize].is_anti_node = true;
                    }
                    if !is_coord_out_of_grid(point_four_x, point_four_y, x_max, y_max) {
                        grid.grid_points[index_two as usize].is_anti_node = true;
                    }
                    distance_counter += 1;
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
    x: i32,
    y: i32,
    index: usize,
    is_antenna: bool,
    is_anti_node: bool,
    char: char,
}

#[derive(Debug, Clone)]
struct Grid {
    x_max: i32,
    y_max: i32,
    grid_points: Vec<GridPoint>,
}
