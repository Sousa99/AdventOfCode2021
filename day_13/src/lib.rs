use std::fmt;
use std::collections::HashMap;

// ================================================== STRUCTS ==================================================

type CoordinateUnit = u64;

#[derive(PartialEq, Copy, Clone)]
enum PointInfo { Free, Set, Fold }

#[derive(PartialEq, Copy, Clone)]
enum FoldAxis { X, Y }

pub struct Map {
    grid_size:      (CoordinateUnit, CoordinateUnit),
    grid:           HashMap<(CoordinateUnit, CoordinateUnit), PointInfo>,
    folds:          Vec<(FoldAxis, CoordinateUnit)>,
}

// ================================================== AUX FUNCTIONS ==================================================

// ================================================== IMPLEMENTATIONS ==================================================

impl Map {

    pub fn new(points: Vec<(CoordinateUnit, CoordinateUnit)>, folds: Vec<(String, CoordinateUnit)>) -> Map {

        let mut size_x : CoordinateUnit = 0;
        let mut size_y : CoordinateUnit = 0;

        let mut map : HashMap<(CoordinateUnit, CoordinateUnit), PointInfo> = HashMap::new();

        for point in points.into_iter() {

            map.insert(point, PointInfo::Set);
            size_x = std::cmp::max(size_x, point.0);
            size_y = std::cmp::max(size_y, point.1);
        }

        for y_pos in 0..(size_y + 1) {
            for x_pos in 0..(size_x + 1) {
                if !map.contains_key(&(x_pos, y_pos)) {
                    map.insert((x_pos, y_pos), PointInfo::Free);
                }
            }
        }

        let treated_folds : Vec<(FoldAxis, CoordinateUnit)> = folds.into_iter()
            .map(|(axis, value)| {
                if axis == "x" { return (FoldAxis::X, value); }
                else if axis == "y" { return (FoldAxis::Y, value); }
                else { panic!("🚨  Axis not recognized '{}'!", axis); }
            }).rev().collect();

        Map {
            grid_size: (size_x, size_y),
            grid: map,
            folds: treated_folds,
        }
    }

    pub fn number_of_folds(&self) -> usize { self.folds.len() }

    pub fn make_next_fold(&mut self) {

        if self.folds.len() == 0 { return }

        let fold = self.folds.pop().unwrap();
        match fold.0 {

            FoldAxis::X => {

                for y_pos in 0..(self.grid_size.1 + 1) { self.grid.insert((fold.1, y_pos), PointInfo::Fold); }
                let mut still_valid : bool = true;
                let mut x_variation : CoordinateUnit = 1;

                while still_valid {

                    if x_variation > fold.1 || fold.1 + x_variation > self.grid_size.0 {
                        still_valid = false;
                    
                    } else {

                        let left : CoordinateUnit = fold.1 - x_variation;
                        let right : CoordinateUnit = fold.1 + x_variation;
                        for y_pos in 0..(self.grid_size.1 + 1) {
                            let &right_side_value = self.grid.get(&(right, y_pos)).unwrap();
                            if right_side_value == PointInfo::Set {
                                self.grid.insert((left, y_pos), PointInfo::Set);
                            }
                        }
                    }

                    x_variation = x_variation + 1;
                }

                self.grid_size.0 = fold.1 - 1;
            }

            FoldAxis::Y => {

                for x_pos in 0..(self.grid_size.0 + 1) { self.grid.insert((x_pos, fold.1), PointInfo::Fold); }
                let mut still_valid : bool = true;
                let mut y_variation : CoordinateUnit = 1;

                while still_valid {

                    if y_variation > fold.1 || fold.1 + y_variation > self.grid_size.1 {
                        still_valid = false;
                    
                    } else {

                        let top : CoordinateUnit = fold.1 - y_variation;
                        let bottom : CoordinateUnit = fold.1 + y_variation;
                        for x_pos in 0..(self.grid_size.0 + 1) {
                            let &bottom_side_value = self.grid.get(&(x_pos, bottom)).unwrap();
                            if bottom_side_value == PointInfo::Set {
                                self.grid.insert((x_pos, top), PointInfo::Set);
                            }
                        }
                    }

                    y_variation = y_variation + 1;
                }

                self.grid_size.1 = fold.1 - 1;
            }
        }
    }

    pub fn count_set(&self) -> u64 {

        let mut count : u64 = 0;
        for y_value in 0..(self.grid_size.1 + 1) {
            for x_value in 0..(self.grid_size.0 + 1) {

                let &point_info = self.grid.get(&(x_value, y_value)).unwrap();
                if point_info == PointInfo::Set {
                    count = count + 1;
                }
            }
        }

        return count;
    }
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        let mut line : String = String::new();
        for y_value in 0..(self.grid_size.1 + 1) {

            for x_value in 0..(self.grid_size.0 + 1) {

                let &point_info = self.grid.get(&(x_value, y_value)).unwrap();
                if point_info == PointInfo::Free { line = format!("{} .", line) }
                else if point_info == PointInfo::Set { line = format!("{} #", line) }
                else { line = format!("{} -", line) }
            }

            line = format!("{}\n", line);

        }

        return write!(f, "{}", line);
    }
}