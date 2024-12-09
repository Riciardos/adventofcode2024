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

    mark_anti_nodes(&mut grid, &antenna_map);

    // for x in 0..x_size {
    //     for y in 0..y_size {
    //         print!("{}", if grid[x*y_size + y].is_anti_node {
    //             '#'
    //         } else {
    //             grid[x*y_size + y].char
    //         })
    //     }
    //     println!()
    // }

    println!("Day 8 part 1 {:?}", grid.iter().filter(|p| p.is_anti_node).count());
}

fn mark_anti_nodes(grid: &mut Vec<GridPoint>, antenna_map: &HashMap<char, Vec<&GridPoint>>) {
    antenna_map.iter().for_each(|(antenna_id, antennas )| {
        for x in 0..antennas.len() - 1 {
            for y in x+1..antennas.len() {
                let point_one = antennas[x];
                let point_two = antennas[y];
                let x_diff = point_two.x as i32 - point_one.x as i32;
                let y_diff = point_two.y as i32 - point_one.y as i32;

                let point_three_x = point_one.x as i32 + 2 * x_diff;
                let point_three_y = point_one.y as i32 + 2 * y_diff;
                let point_four_x = point_two.x as i32 - 2 * x_diff;
                let point_four_y = point_two.y as i32 - 2 * y_diff;
                let index = (point_three_x - 1)* 50 + point_three_y - 1;
                let index_two = (point_four_x - 1) * 50 + point_four_y - 1;

                if point_three_x > 0 && point_three_x <= 50 && point_three_y > 0 && point_three_y <= 50 {
                    grid[index as usize].is_anti_node = true;
                }
                if point_four_x > 0 && point_four_x <= 50 && point_four_y > 0 && point_four_y <= 50 {
                    grid[index_two as usize].is_anti_node = true;
                }
            }
        }
    })
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