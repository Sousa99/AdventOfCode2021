use std::fmt;
use std::collections::{HashMap, HashSet};

// ================================================== STRUCTS ==================================================

type RiskLevel = u32;
type PositionUnit = u32;
type Position = (PositionUnit, PositionUnit);

#[derive(Clone, Copy)]
struct Node {
    enter_risk:     RiskLevel,
    closed:         bool,
    min_distance:   Option<RiskLevel>,
}

pub struct Map {
    map_start:      Position,
    map_end:        Position,
    map_risks:      HashMap<Position, Node>,
}

// ================================================== AUX FUNCTIONS ==================================================

// ================================================== IMPLEMENTATIONS ==================================================

impl Map {

    pub fn new(values: Vec<Vec<RiskLevel>>, tiling: usize) -> Map {

        let map_start : Position = (0, 0);

        let tile_end_x : PositionUnit = values.iter().map(|line| line.len()).max().unwrap() as PositionUnit;
        let tile_end_y : PositionUnit = values.len() as PositionUnit;

        let map_end_x : PositionUnit = tile_end_x * (tiling as PositionUnit) - 1;
        let map_end_y : PositionUnit = tile_end_y * (tiling as PositionUnit) - 1;
        let map_end : Position = (map_end_x, map_end_y);

        let mut map_risks : HashMap<Position, Node> = HashMap::new();

        // Iterate Values
        for (row_index, row) in values.into_iter().enumerate() {
            for (column_index, value) in row.into_iter().enumerate() {

                // Place values in each tile
                for x_tile in 0..tiling {
                    for y_tile in 0..tiling {

                        let tile_value : RiskLevel = x_tile as RiskLevel + y_tile as RiskLevel;
                        let cell_value : RiskLevel = ( value - 1 + tile_value ) % 9 + 1;

                        let position_x : PositionUnit = (column_index as PositionUnit) + x_tile as PositionUnit * tile_end_x;
                        let position_y : PositionUnit = (row_index as PositionUnit) + y_tile as PositionUnit * tile_end_y;

                        let node : Node = Node{ enter_risk: cell_value, closed: false, min_distance: None };
                        map_risks.insert((position_x, position_y), node);

                    }
                }

            }
        }

        Map {
            map_start: map_start,
            map_end: map_end,
            map_risks: map_risks,
        }

    }

    pub fn get_shortest_path(&mut self) -> Option<RiskLevel> {

        let start_node = self.map_risks.get_mut(&self.map_start).unwrap();
        start_node.min_distance = Some(0);
        let mut possible_points_to_expand : HashSet<Position> = HashSet::new();
        possible_points_to_expand.insert(self.map_start.clone());

        while !self.map_risks.get(&self.map_end).as_ref().unwrap().closed {

            if possible_points_to_expand.len() == 0 { panic!("🚨 No points to expand!") }

            let to_expand = possible_points_to_expand.iter()
                .filter(|&position| self.map_risks.contains_key(position))
                .map(|&position| (position, self.map_risks.get(&position).unwrap()))
                .min_by(|(_, &a_node), (_, &b_node)| a_node.min_distance.unwrap().cmp(&b_node.min_distance.unwrap()))
                .unwrap().0;

            let to_expand_distance : RiskLevel = self.map_risks.get(&to_expand).unwrap().min_distance.unwrap();

            for var_x in -1..=1 {
                for var_y in -1..=1 {

                    // Do not stay in the same spot
                    if var_x == 0 && var_y == 0 { continue }
                    // Do not move diagonally
                    if var_x != 0 && var_y != 0 { continue }

                    let new_x = to_expand.0 as i32 + var_x;
                    let new_y = to_expand.1 as i32 + var_y;
                    if new_x < 0 || new_y < 0 { continue }

                    let new_position : Position = ( new_x as PositionUnit, new_y as PositionUnit );
                    let new_node_option = self.map_risks.get_mut(&new_position);
                    if new_node_option.is_none() { continue }

                    let new_node = new_node_option.unwrap();
                    if new_node.closed { continue }

                    let new_distance = to_expand_distance + new_node.enter_risk;

                    if new_node.min_distance.is_none() { new_node.min_distance = Some(new_distance); }
                    else if new_node.min_distance.unwrap() > new_distance { new_node.min_distance = Some(new_distance); }

                    possible_points_to_expand.insert(new_position);
                }
            }

            let to_expand_node = self.map_risks.get_mut(&to_expand).unwrap();
            to_expand_node.closed = true;
            possible_points_to_expand.remove(&to_expand);
        }

        return self.map_risks.get(&self.map_end).unwrap().min_distance;
    }
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        let mut line : String = String::new();
        for y_value in (self.map_start.1)..=(self.map_end.1) {
            for x_value in (self.map_start.0)..=(self.map_end.0) {

                let point = self.map_risks.get(&(x_value, y_value));
                if point.is_some() { line = format!("{} {}", line, &point.unwrap().enter_risk); }
            }

            line = format!("{}\n", line);

        }

        return write!(f, "{}", line);
    }
}