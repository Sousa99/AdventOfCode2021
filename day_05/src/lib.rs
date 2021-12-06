use gcd::Gcd;
use std::fmt;
use std::collections::HashMap;

// ================================================== STRUCTS ==================================================

type CoordinateUnit = i64;

#[derive(Clone)]
struct Point {
    x:      CoordinateUnit,
    y:      CoordinateUnit,
}

#[derive(PartialEq)]
enum LineType { Invalid, Horizontal, Vertical, Diagonal }
#[derive(Clone)]
pub struct Line {
    point1:         Point,
    point2:         Point,
}

type PointInfo = Vec<usize>;
pub struct Map {
    diagonal:       bool,
    lines:          Vec<Line>,
    grid_size:      (usize, usize),
    grid:           HashMap<(usize, usize), PointInfo>
}

// ================================================== AUX FUNCTIONS ==================================================

// ================================================== IMPLEMENTATIONS ==================================================

impl Line {

    pub fn new(x1: CoordinateUnit, y1: CoordinateUnit, x2: CoordinateUnit, y2: CoordinateUnit) -> Line {

        let point1 = Point{x: x1, y: y1};
        let point2 = Point{x: x2, y: y2};

        Line {
            point1: point1,
            point2: point2,
        }
    }

    fn get_type(&self) -> LineType {

        if self.point1.x == self.point2.x && self.point1.y == self.point2.y { return LineType::Invalid; }
        else if self.point1.x != self.point2.x && self.point1.y == self.point2.y { return LineType::Horizontal; }
        else if self.point1.x == self.point2.x && self.point1.y != self.point2.y { return LineType::Vertical; }
        else { return LineType::Diagonal; }
    }

    fn get_points(&self) -> Vec<Point> {

        let line_type : LineType = self.get_type();
        if line_type == LineType::Invalid { return vec!(self.point1.clone()); }

        let mut points : Vec<Point> = Vec::new();

        // Horizontal or Vertical
        if line_type == LineType::Horizontal || line_type == LineType::Vertical {

            if line_type == LineType::Horizontal {

                let min_x = std::cmp::min(self.point1.x, self.point2.x);
                let max_x = std::cmp::max(self.point1.x, self.point2.x);
                for x_value in min_x..(max_x + 1) { points.push(Point{x: x_value, y: self.point1.y}) }
                
            } else if line_type == LineType::Vertical {
                
                let min_y = std::cmp::min(self.point1.y, self.point2.y);
                let max_y = std::cmp::max(self.point1.y, self.point2.y);
                for y_value in min_y..(max_y + 1) { points.push(Point{x: self.point1.x, y: y_value}) }
            }

            return points;

        }

        // Diagonal
        let mut diff : Point = Point{x: self.point2.x - self.point1.x, y: self.point2.y - self.point1.y};
        let gcd = (diff.x.abs() as u64).gcd(diff.y.abs() as u64);
        diff.x = diff.x / gcd as CoordinateUnit;
        diff.y = diff.y / gcd as CoordinateUnit;

        let mut current_point : Point = self.point1.clone();
        points.push(current_point.clone());
        while current_point.x != self.point2.x && current_point.y != self.point2.y {
            
            current_point.x = current_point.x + diff.x;
            current_point.y = current_point.y + diff.y;
            points.push(current_point.clone());
        }

        return points;
    }
}

impl Map {

    pub fn new(lines: Vec<Line>, count_diagonal: bool) -> Map {

        let (mut max_x, mut max_y) = (0, 0);
        for line in lines.iter() {
            max_x = std::cmp::max(max_x, line.point1.x);
            max_x = std::cmp::max(max_x, line.point2.x);
            max_y = std::cmp::max(max_y, line.point1.y);
            max_y = std::cmp::max(max_y, line.point2.y);
        }

        let mut grid : HashMap<(usize, usize), PointInfo> = HashMap::new();
        for x_value in 0..(max_x + 1) {
            for y_value in 0..(max_y + 1) {
                grid.insert((x_value as usize, y_value as usize), PointInfo::new());
            }
        }

        Map {
            diagonal: count_diagonal,
            lines: lines,
            grid_size: (max_x as usize, max_y as usize),
            grid: grid,
        }
    }

    pub fn mark_lines(&mut self) {

        for (line_index, line) in self.lines.iter().enumerate() {

            let line_type = line.get_type();
            if !self.diagonal && (line_type != LineType::Horizontal && line_type != LineType::Vertical) { continue }

            let points = line.get_points();
            for point in points.into_iter() {
                
                let point_info = self.grid.get_mut(&(point.x as usize, point.y as usize)).unwrap();
                point_info.push(line_index);
            }
        }
    }

    pub fn count_positions_with_me(&self, threshold: usize) -> usize {

        let mut count : usize = 0;
        for (_, info) in self.grid.iter() {
            if info.len() >= threshold { count = count + 1 }
        }

        return count;
    }
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        let mut line : String = String::new();
        for y_value in 0..(self.grid_size.1 + 1) {

            for x_value in 0..(self.grid_size.0 + 1) {

                let info = self.grid.get(&(x_value, y_value)).unwrap();
                if info.len() == 0 { line = format!("{} .", line) }
                else { line = format!("{} {}", line, info.len()) }
            }

            line = format!("{}\n", line);

        }

        return write!(f, "{}", line);
    }
}