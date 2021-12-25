use std::fmt;
use std::collections::{HashMap};

// ================================================== STRUCTS ==================================================

type PositionUnit = i64;
type Position = (PositionUnit, PositionUnit);

#[derive(PartialEq, Copy, Clone)]
enum PositionType { Free, EastFacingCucumber, SouthFacingCucumber }

pub struct Map {
    iteration:  usize,
    map_start:  Position,
    map_end:    Position,
    map:        HashMap<Position, PositionType>,
}

// ================================================== AUX FUNCTIONS ==================================================

fn convert_characther_position_type(position_characther: char) -> PositionType {
    match position_characther {
        '.' => PositionType::Free,
        '>' => PositionType::EastFacingCucumber,
        'v' => PositionType::SouthFacingCucumber,
        _ => panic!("🚨  Characther not recognized as a position type")
    }
}

fn convert_position_type_characther(position_type: &PositionType) -> char {
    match position_type {
        PositionType::Free => '.',
        PositionType::EastFacingCucumber => '>',
        PositionType::SouthFacingCucumber => 'v',
    }
}

// ================================================== IMPLEMENTATIONS ==================================================

impl Map {

    pub fn new(info: Vec<Vec<char>>) -> Map {

        let map_end_x = info.iter().map(|line| line.len()).max().unwrap() as PositionUnit - 1;
        let map_end_y = info.len() as PositionUnit - 1;

        let mut map : HashMap<Position, PositionType> = HashMap::new();
        for (line_index, line) in info.into_iter().enumerate() {
            for (characther_index, characther) in line.into_iter().enumerate() {
                
                let position = (characther_index as PositionUnit, line_index as PositionUnit);
                let position_type = convert_characther_position_type(characther);
                map.insert(position, position_type);
            }
        }

        Map {
            iteration: 0,
            map_start: (0, 0),
            map_end: (map_end_x, map_end_y),
            map: map,
        }
    }

    fn check_position_free(&self, position: Position) -> bool {
        let position_type_option = self.map.get(&position);
        return position_type_option.is_some() && *position_type_option.unwrap() == PositionType::Free;
    }
    
    fn get_position_to_move(&self, position: Position) -> Position {

        let position_type = *self.map.get(&position).unwrap();
        let new_position = match position_type {
            PositionType::Free => position,
            PositionType::EastFacingCucumber => (position.0 + 1, position.1),
            PositionType::SouthFacingCucumber => (position.0, position.1 + 1),
        };

        return (new_position.0 % (self.map_end.0 + 1) + self.map_start.0, new_position.1 % (self.map_end.1 + 1) + self.map_start.1);
    }

    fn iterate_type(&mut self, type_to_iterate: PositionType) -> bool {

        let target_points : Vec<(Position, Position)> = self.map.iter()
            .filter(|(_, &position_type)| position_type == type_to_iterate)
            .map(|(&position, _)| (position, self.get_position_to_move(position)))
            .filter(|&(_, position_to)| self.check_position_free(position_to))
            .collect();

        let count_changes : usize = target_points.len();

        for (position_from, position_to) in target_points.into_iter() {
            self.map.insert(position_from, PositionType::Free);
            self.map.insert(position_to, type_to_iterate);
        }

        return count_changes != 0;
    }

    pub fn iterate(&mut self) -> bool {

        let east_changes = self.iterate_type(PositionType::EastFacingCucumber);
        let south_changes = self.iterate_type(PositionType::SouthFacingCucumber);

        if !(east_changes || south_changes) { return false }
        self.iteration = self.iteration + 1;
        return true;
    }

    pub fn get_final_iteration(&self) -> usize { self.iteration + 1 }
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        let mut info_printed : String = format!("Iteration {}:\n", self.iteration);
        for y_value in self.map_start.1..=self.map_end.1 {
            for x_value in self.map_start.0..=self.map_end.0 {

                let position : Position = (x_value as PositionUnit, y_value as PositionUnit);
                let position_type : &PositionType = self.map.get(&position).unwrap();
                let position_characther = convert_position_type_characther(position_type);
                info_printed = format!("{}{}", info_printed, position_characther);
            }

            info_printed = format!("{}\n", info_printed);
        }

        return write!(f, "{}", info_printed);
    }
}