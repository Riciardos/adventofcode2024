mod util {}

pub fn look_right(grid: &Vec<Point>, x_coord: usize, y_size: usize, y_coord: usize) -> bool {
    let Some(point_x) = grid.get(x_coord * y_size + y_coord) else {
        return false;
    };
    let Some(point_m) = grid.get(x_coord * y_size + y_coord + 1) else {
        return false;
    };
    let Some(point_a) = grid.get(x_coord * y_size + y_coord + 2) else {
        return false;
    };
    let Some(point_s) = grid.get(x_coord * y_size + y_coord + 3) else {
        return false;
    };

    if point_x.letter == 'X'
        && point_m.letter == 'M'
        && point_a.letter == 'A'
        && point_s.letter == 'S'
    {
        return true;
    }
    false
}

pub fn look_left(grid: &Vec<Point>, x_coord: usize, y_size: usize, y_coord: usize) -> bool {
    if y_coord < 3 {
        return false;
    }
    let Some(point_x) = grid.get(x_coord * y_size + y_coord) else {
        return false;
    };
    let Some(point_m) = grid.get(x_coord * y_size + y_coord - 1) else {
        return false;
    };
    let Some(point_a) = grid.get(x_coord * y_size + y_coord - 2) else {
        return false;
    };
    let Some(point_s) = grid.get(x_coord * y_size + y_coord - 3) else {
        return false;
    };

    if point_x.letter == 'X'
        && point_m.letter == 'M'
        && point_a.letter == 'A'
        && point_s.letter == 'S'
    {
        return true;
    }
    false
}

pub fn look_down(grid: &Vec<Point>, x_coord: usize, y_size: usize, y_coord: usize) -> bool {
    let Some(point_x) = grid.get(x_coord * y_size + y_coord) else {
        return false;
    };
    let Some(point_m) = grid.get((x_coord + 1) * y_size + y_coord) else {
        return false;
    };
    let Some(point_a) = grid.get((x_coord + 2) * y_size + y_coord) else {
        return false;
    };
    let Some(point_s) = grid.get((x_coord + 3) * y_size + y_coord) else {
        return false;
    };

    if point_x.letter == 'X'
        && point_m.letter == 'M'
        && point_a.letter == 'A'
        && point_s.letter == 'S'
    {
        return true;
    }
    false
}

pub fn look_up(grid: &Vec<Point>, x_coord: usize, y_size: usize, y_coord: usize) -> bool {
    if x_coord < 3 {
        return false;
    }

    let Some(point_x) = grid.get(x_coord * y_size + y_coord) else {
        return false;
    };
    let Some(point_m) = grid.get((x_coord - 1) * y_size + y_coord) else {
        return false;
    };
    let Some(point_a) = grid.get((x_coord - 2) * y_size + y_coord) else {
        return false;
    };
    let Some(point_s) = grid.get((x_coord - 3) * y_size + y_coord) else {
        return false;
    };

    if point_x.letter == 'X'
        && point_m.letter == 'M'
        && point_a.letter == 'A'
        && point_s.letter == 'S'
    {
        return true;
    }
    false
}

pub fn look_up_right(grid: &Vec<Point>, x_coord: usize, y_size: usize, y_coord: usize) -> bool {
    if x_coord < 3 {
        return false;
    }

    let Some(point_x) = grid.get(x_coord * y_size + y_coord) else {
        return false;
    };
    let Some(point_m) = grid.get((x_coord - 1) * y_size + y_coord + 1) else {
        return false;
    };
    let Some(point_a) = grid.get((x_coord - 2) * y_size + y_coord + 2) else {
        return false;
    };
    let Some(point_s) = grid.get((x_coord - 3) * y_size + y_coord + 3) else {
        return false;
    };

    if point_x.letter == 'X'
        && point_m.letter == 'M'
        && point_a.letter == 'A'
        && point_s.letter == 'S'
    {
        return true;
    }
    false
}

pub fn look_up_left(grid: &Vec<Point>, x_coord: usize, y_size: usize, y_coord: usize) -> bool {
    if x_coord < 3 || y_coord < 3 {
        return false;
    }

    let Some(point_x) = grid.get(x_coord * y_size + y_coord) else {
        return false;
    };
    let Some(point_m) = grid.get((x_coord - 1) * y_size + y_coord - 1) else {
        return false;
    };
    let Some(point_a) = grid.get((x_coord - 2) * y_size + y_coord - 2) else {
        return false;
    };
    let Some(point_s) = grid.get((x_coord - 3) * y_size + y_coord - 3) else {
        return false;
    };

    if point_x.letter == 'X'
        && point_m.letter == 'M'
        && point_a.letter == 'A'
        && point_s.letter == 'S'
    {
        return true;
    }
    false
}

pub fn look_down_right(grid: &Vec<Point>, x_coord: usize, y_size: usize, y_coord: usize) -> bool {
    let Some(point_x) = grid.get(x_coord * y_size + y_coord) else {
        return false;
    };
    let Some(point_m) = grid.get((x_coord + 1) * y_size + y_coord + 1) else {
        return false;
    };
    let Some(point_a) = grid.get((x_coord + 2) * y_size + y_coord + 2) else {
        return false;
    };
    let Some(point_s) = grid.get((x_coord + 3) * y_size + y_coord + 3) else {
        return false;
    };

    if point_x.letter == 'X'
        && point_m.letter == 'M'
        && point_a.letter == 'A'
        && point_s.letter == 'S'
    {
        return true;
    }
    false
}

pub fn look_down_left(grid: &Vec<Point>, x_coord: usize, y_size: usize, y_coord: usize) -> bool {
    let Some(point_x) = grid.get(x_coord * y_size + y_coord) else {
        return false;
    };
    let Some(point_m) = grid.get((x_coord + 1) * y_size + y_coord - 1) else {
        return false;
    };
    let Some(point_a) = grid.get((x_coord + 2) * y_size + y_coord - 2) else {
        return false;
    };
    let Some(point_s) = grid.get((x_coord + 3) * y_size + y_coord - 3) else {
        return false;
    };

    if point_x.letter == 'X'
        && point_m.letter == 'M'
        && point_a.letter == 'A'
        && point_s.letter == 'S'
    {
        return true;
    }
    false
}

#[derive(Debug, Clone)]
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
