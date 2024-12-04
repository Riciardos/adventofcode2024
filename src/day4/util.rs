mod util {}

pub fn look_right(grid: &Vec<Point>, x_coord: usize, y_size: usize, y_coord: usize) -> bool {
    if let Some(point) = grid.get(x_coord * y_size + y_coord) {
        if point.letter == 'X' {
            if let Some(point) = grid.get(x_coord * y_size + y_coord + 1) {
                if point.letter == 'M' {
                    if let Some(point) = grid.get(x_coord * y_size + y_coord + 2) {
                        if point.letter == 'A' {
                            if let Some(point) = grid.get(x_coord * y_size + y_coord + 3) {
                                if point.letter == 'S' {
                                    return true;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    false
}

pub fn look_left(grid: &Vec<Point>, x_coord: usize, y_size: usize, y_coord: usize) -> bool {
    if let Some(point) = grid.get(x_coord * y_size + y_coord) {
        if point.letter == 'X' {
            if let Some(point) = grid.get(x_coord * y_size + y_coord - 1) {
                if point.letter == 'M' {
                    if let Some(point) = grid.get(x_coord * y_size + y_coord - 2) {
                        if point.letter == 'A' {
                            if let Some(point) = grid.get(x_coord * y_size + y_coord - 3) {
                                if point.letter == 'S' {
                                    return true;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    false
}

pub fn look_down(grid: &Vec<Point>, x_coord: usize, y_size: usize, y_coord: usize) -> bool {
    if let Some(point) = grid.get(x_coord * y_size + y_coord) {
        if point.letter == 'X' {
            if let Some(point) = grid.get((x_coord + 1) * y_size + y_coord) {
                if point.letter == 'M' {
                    if let Some(point) = grid.get((x_coord + 2) * y_size + y_coord) {
                        if point.letter == 'A' {
                            if let Some(point) = grid.get((x_coord + 3) * y_size + y_coord) {
                                if point.letter == 'S' {
                                    return true;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    false
}

pub fn look_up(grid: &Vec<Point>, x_coord: usize, y_size: usize, y_coord: usize) -> bool {
    if x_coord < 3 {
        return false;
    }
    if let Some(point) = grid.get(x_coord * y_size + y_coord) {
        if point.letter == 'X' {
            if let Some(point) = grid.get((x_coord - 1) * y_size + y_coord) {
                if point.letter == 'M' {
                    if let Some(point) = grid.get((x_coord - 2) * y_size + y_coord) {
                        if point.letter == 'A' {
                            if let Some(point) = grid.get((x_coord - 3) * y_size + y_coord) {
                                if point.letter == 'S' {
                                    return true;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    false
}

pub fn look_up_right(grid: &Vec<Point>, x_coord: usize, y_size: usize, y_coord: usize) -> bool {
    if x_coord < 3 {
        return false;
    }
    if let Some(point) = grid.get(x_coord * y_size + y_coord) {
        if point.letter == 'X' {
            if let Some(point) = grid.get((x_coord - 1) * y_size + y_coord + 1) {
                if point.letter == 'M' {
                    if let Some(point) = grid.get((x_coord - 2) * y_size + y_coord + 2) {
                        if point.letter == 'A' {
                            if let Some(point) = grid.get((x_coord - 3) * y_size + y_coord + 3) {
                                if point.letter == 'S' {
                                    return true;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    false
}

pub fn look_up_left(grid: &Vec<Point>, x_coord: usize, y_size: usize, y_coord: usize) -> bool {
    if x_coord < 3 || y_coord < 3 {
        return false;
    }
    if let Some(point) = grid.get(x_coord * y_size + y_coord) {
        if point.letter == 'X' {
            if let Some(point_m) = grid.get((x_coord - 1) * y_size + y_coord - 1) {
                if point_m.letter == 'M' {
                    if let Some(point_a) = grid.get((x_coord - 2) * y_size + y_coord - 2) {
                        if point_a.letter == 'A' {
                            if let Some(point_s) = grid.get((x_coord - 3) * y_size + y_coord - 3) {
                                if point_s.letter == 'S' {
                                    return true;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    false
}

pub fn look_down_right(grid: &Vec<Point>, x_coord: usize, y_size: usize, y_coord: usize) -> bool {
    if let Some(point) = grid.get(x_coord * y_size + y_coord) {
        if point.letter == 'X' {
            if let Some(point) = grid.get((x_coord + 1) * y_size + y_coord + 1) {
                if point.letter == 'M' {
                    if let Some(point) = grid.get((x_coord + 2) * y_size + y_coord + 2) {
                        if point.letter == 'A' {
                            if let Some(point) = grid.get((x_coord + 3) * y_size + y_coord + 3) {
                                if point.letter == 'S' {
                                    return true;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    false
}

pub fn look_down_left(grid: &Vec<Point>, x_coord: usize, y_size: usize, y_coord: usize) -> bool {
    if let Some(original_point) = grid.get(x_coord * y_size + y_coord) {
        if original_point.letter == 'X' {
            if let Some(point_m) = grid.get((x_coord + 1) * y_size + y_coord - 1) {
                if point_m.letter == 'M' {
                    if let Some(point_a) = grid.get((x_coord + 2) * y_size + y_coord - 2) {
                        if point_a.letter == 'A' {
                            if let Some(point_s) = grid.get((x_coord + 3) * y_size + y_coord - 3) {
                                if point_s.letter == 'S' {
                                    return true;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    false
}

#[derive(Debug)]
pub struct Point {
    pub(crate) x: usize,
    pub(crate) y: usize,
    pub(crate) letter: char,
    pub(crate) found: bool,
}

impl Point {
    pub fn set_found(&mut self, found: bool) {
        self.found = found;
    }
}