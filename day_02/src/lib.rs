use std::collections::HashMap;

// ================================================== STRUCTS ==================================================

type CoordinateUnit = i64;

#[derive(Copy, Clone)]
pub struct Position {
    pub horizontal:     CoordinateUnit,
    pub depth:          CoordinateUnit,
}

#[derive(Copy, Clone)]
enum MovementCode { Up, Forward, Down }
pub struct Submarine {
    start_position:     Position,
    positions_traveled: Vec<Position>
}

pub struct AimedSubmarine {
    start_position:     Position,
    aim:                CoordinateUnit,
    positions_traveled: Vec<Position>
}

// ================================================== AUX FUNCTIONS ==================================================

fn convert_str_code(code_str: &str) -> MovementCode {

    let mut map_str_codes: HashMap<&str, MovementCode> = HashMap::new();
    map_str_codes.insert("up",          MovementCode::Up);
    map_str_codes.insert("down",        MovementCode::Down);
    map_str_codes.insert("forward",     MovementCode::Forward);

    let code_converted = map_str_codes.get(code_str).unwrap();
    return code_converted.clone();
}

// ================================================== IMPLEMENTATIONS ==================================================

impl Submarine {
    pub fn new() -> Submarine {
        Submarine {
            start_position:     Position{horizontal: 0, depth: 0},
            positions_traveled: Vec::new(),
        }
    }

    pub fn update_chart(&mut self, lines: Vec<String>) {

        self.positions_traveled = vec!(self.start_position);
        
        for line in lines.into_iter() {

            let line_split: Vec<&str> = line.split(' ').collect();
            let code = line_split[0];
            let value: CoordinateUnit = line_split[1].parse().unwrap();

            let last_position = self.positions_traveled.last().unwrap();

            let code_converted = convert_str_code(code);
            let new_position = match code_converted {
                MovementCode::Up => Position{horizontal: last_position.horizontal, depth: last_position.depth - value},
                MovementCode::Forward => Position{horizontal: last_position.horizontal + value, depth: last_position.depth},
                MovementCode::Down => Position{horizontal: last_position.horizontal, depth: last_position.depth + value},
            };

            self.positions_traveled.push(new_position);
        }
    }

    pub fn get_last_position(&self) -> Option<&Position> {
        return self.positions_traveled.last();
    }
}

impl AimedSubmarine {
    pub fn new() -> AimedSubmarine {
        AimedSubmarine {
            start_position:     Position{horizontal: 0, depth: 0},
            aim:                0,
            positions_traveled: Vec::new(),
        }
    }

    pub fn update_chart(&mut self, lines: Vec<String>) {

        self.positions_traveled = vec!(self.start_position);
        
        for line in lines.into_iter() {

            let line_split: Vec<&str> = line.split(' ').collect();
            let code = line_split[0];
            let value: CoordinateUnit = line_split[1].parse().unwrap();

            let last_position = self.positions_traveled.last().unwrap();

            let code_converted = convert_str_code(code);

            match code_converted {
                MovementCode::Up => self.aim = self.aim - value,
                MovementCode::Down => self.aim = self.aim + value,
                MovementCode::Forward => {
                    let new_position = Position{horizontal: last_position.horizontal + value, depth: last_position.depth + self.aim * value};
                    self.positions_traveled.push(new_position)
                },
            };
        }
    }

    pub fn get_last_position(&self) -> Option<&Position> {
        return self.positions_traveled.last();
    }
}