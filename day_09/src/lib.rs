use std::collections::HashMap;

// ================================================== STRUCTS ==================================================

type Digit = u32;

#[derive(PartialEq, Copy, Clone)]
enum CellType { LowPoint, Other }
struct Cell {
    value:      Digit,
    cell_type:  Option<CellType>,
}

struct Basin {
    center:         (usize, usize),
    basin_members:  Vec<(usize, usize)>
}

pub struct Map {
    size:       (usize, usize),
    cell_map:   HashMap<(i64, i64), Cell>,
}

// ================================================== AUX FUNCTIONS ==================================================

// ================================================== IMPLEMENTATIONS ==================================================

impl Cell {

    fn new(value: Digit) -> Cell {
        Cell {
            value: value,
            cell_type: None,
        }
    }

    fn get_value(&self) -> Digit { self.value }

    fn set_cell_type(&mut self, neighbours: Vec<Digit>) {

        let mut lower_than_all : bool = true;
        for neighbour_value in neighbours.into_iter() {
            if self.value >= neighbour_value {
                lower_than_all = false;
            }
        }

        if lower_than_all { self.cell_type = Some(CellType::LowPoint) }
        else { self.cell_type = Some(CellType::Other) }
    }

    fn get_risk_level(&self) -> Option<u32> {

        if self.cell_type.is_none() { return None; }
        if self.cell_type.unwrap() == CellType::LowPoint { return Some(self.value + 1); }
        else { return None; }
    }
}

impl Map {

    pub fn new(cell_values: Vec<Vec<Digit>>) -> Map {

        let mut size : (usize, usize) = (0, 0);
        let mut map : HashMap<(i64, i64), Cell> = HashMap::new();

        size.0 = cell_values.len();
        for (index_row, cell_row) in cell_values.into_iter().enumerate() {

            size.1 = std::cmp::max(size.1, cell_row.len());
            for (index_col, cell_value) in cell_row.into_iter().enumerate() {

                let new_cell : Cell = Cell::new(cell_value);
                map.insert((index_row as i64, index_col as i64), new_cell);
            }
        }

        Map {
            size: size,
            cell_map: map,
        }
    }

    pub fn find_low_points(&mut self) {

        for row_index in 0..self.size.0 {
            for column_index in 0..self.size.1 {

                let mut cell_values : Vec<Digit> = Vec::new();
                // Check neighbours
                for row_variation in vec![-1, 1] {
                    let neighbour_cell = self.cell_map.get(&(row_index as i64 + row_variation, column_index as i64));
                    if neighbour_cell.is_some() { cell_values.push(neighbour_cell.unwrap().get_value()) }
                }
                for col_variation in vec![-1, 1] {
                    let neighbour_cell = self.cell_map.get(&(row_index as i64, column_index as i64 + col_variation));
                    if neighbour_cell.is_some() { cell_values.push(neighbour_cell.unwrap().get_value()) }
                }

                // Update current cell
                let cell_option = self.cell_map.get_mut(&(row_index as i64, column_index as i64));
                if cell_option.is_none() { continue }

                let cell = cell_option.unwrap();
                cell.set_cell_type(cell_values);

            }
        }
    }

    pub fn find_sum_risk_levels(&self) -> u32 {

        let mut sum_risk : u32 = 0;
        for (_, cell) in self.cell_map.iter() {
            
            let risk_level = cell.get_risk_level();
            if risk_level.is_some() { sum_risk = sum_risk + risk_level.unwrap(); }
        }

        return sum_risk;
    }

    pub fn find_basins_value(&self) -> u32 {

        let mut basins : Vec<Basin> = Vec::new();
        for (&(row_index, col_index), cell) in self.cell_map.iter() {

            if cell.cell_type.unwrap() == CellType::Other { continue; }

            let initial_position = (row_index as usize, col_index as usize);
            let mut basin : Basin = Basin{center: initial_position, basin_members: vec!(initial_position)};
            let cell_value = cell.get_value();

            let mut still_to_check = vec!(initial_position);
            while still_to_check.len() > 0 {

                let check_position = still_to_check.pop().unwrap();

                // Check neighbours
                for row_variation in vec![-1, 1] {
                    let neighbour_position = (check_position.0 as i64 + row_variation, check_position.1 as i64);
                    let neighbour_cell = self.cell_map.get(&neighbour_position);
                    if neighbour_cell.is_some() {

                        let neighbour_position_corrected = (neighbour_position.0 as usize, neighbour_position.1 as usize);
                        let neighbour_value = neighbour_cell.unwrap().get_value();
                        if neighbour_value > cell_value && neighbour_value != 9 && !basin.basin_members.contains(&neighbour_position_corrected) {

                            still_to_check.push(neighbour_position_corrected);
                            basin.basin_members.push(neighbour_position_corrected);
                        }
                    }
                }
                for col_variation in vec![-1, 1] {
                    let neighbour_position = (check_position.0 as i64, check_position.1 as i64 + col_variation);
                    let neighbour_cell = self.cell_map.get(&neighbour_position);
                    if neighbour_cell.is_some() {

                        let neighbour_position_corrected = (neighbour_position.0 as usize, neighbour_position.1 as usize);
                        let neighbour_value = neighbour_cell.unwrap().get_value();
                        if neighbour_value > cell_value && neighbour_value != 9 && !basin.basin_members.contains(&neighbour_position_corrected) {

                            still_to_check.push(neighbour_position_corrected);
                            basin.basin_members.push(neighbour_position_corrected);
                        }
                    }
                }
            }

            //println!("Basin center '({}, {})' has size of '{}'", basin.center.0, basin.center.1, basin.basin_members.len());
            basins.push(basin);
        }

        basins.sort_by_key(|basin| basin.basin_members.len());
        basins.reverse();

        return basins[0..3].iter().fold(1u32, |value, basin| value * basin.basin_members.len() as u32);
    }
}