use std::hash::Hash;
use std::collections::{HashMap, HashSet};

// ================================================== STRUCTS ==================================================

type PositionUnit = i32;
type Position = (PositionUnit, PositionUnit);

#[derive(PartialEq, Copy, Clone)]
enum AmphipodType { Amber, Bronze, Copper, Desert }

#[derive(Clone)]
struct Amphipod {
    amphipod_type:  AmphipodType,
    position:       Position,
    moved_outside:  bool,
}

#[derive(PartialEq, Copy, Clone)]
enum PositionType { Wall, Hallway, Room }

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
enum RoomType { AmberRoom, BronzeRoom, CopperRoom, DesertRoom }

#[derive(Clone)]
struct Room {
    room_type:  RoomType,
    positions:  Vec<Position>,
}

type Energy = i64;

#[derive(Clone)]
struct Scenario {
    energy_consumed:    Energy,
    rooms:              Vec<Room>,
    amphipods:          Vec<Amphipod>,
}

pub struct Map {
    map_size:           (Position, Position),
    map_positions:      HashMap<Position, PositionType>,
    starting_scenario:  Scenario,
    final_scenario:     Option<Scenario>,
}

// ================================================== AUXILIARY FUNCTIONS ==================================================

fn convert_characther_to_amphipod(characther: char) -> AmphipodType {
    match characther {
        'A' => AmphipodType::Amber,
        'B' => AmphipodType::Bronze,
        'C' => AmphipodType::Copper,
        'D' => AmphipodType::Desert,
        _ => panic!("🚨  Amphipod type not recognized '{}'", characther)
    }
}

fn convert_amphipod_type_to_characther(amphipod_type: &AmphipodType) -> char {
    match amphipod_type {
        AmphipodType::Amber => 'A',
        AmphipodType::Bronze => 'B',
        AmphipodType::Copper => 'C',
        AmphipodType::Desert => 'D',
    }
}

fn convert_room_type_to_characther(room_type: &RoomType) -> char {
    match room_type {
        RoomType::AmberRoom => 'a',
        RoomType::BronzeRoom => 'b',
        RoomType::CopperRoom => 'c',
        RoomType::DesertRoom => 'd',
    }
}

fn convert_position_type_to_characther(position_type: &PositionType) -> char {
    match position_type {
        PositionType::Wall => '#',
        PositionType::Hallway => '.',
        PositionType::Room => 'o',
    }
}

// ================================================== IMPLEMENTATIONS ==================================================

impl Amphipod {

    pub fn new(amphipod_type: AmphipodType, position: Position) -> Amphipod {
        Amphipod {
            amphipod_type: amphipod_type,
            position: position,
            moved_outside: false,
        }
    }

    fn get_movement_cost(&self) -> Energy {
        match self.amphipod_type {
            AmphipodType::Amber => 1, 
            AmphipodType::Bronze => 10, 
            AmphipodType::Copper => 100, 
            AmphipodType::Desert => 1000, 
        }
    }
}

impl Room {

    pub fn new(room_type: RoomType, positions: Vec<Position>) -> Room {
        Room {
            room_type: room_type,
            positions: positions,
        }
    }
}

impl Scenario {

    pub fn new(rooms: Vec<Room>, amphipods: Vec<Amphipod>) -> Scenario {
        Scenario {
            energy_consumed: 0,
            rooms: rooms,
            amphipods: amphipods,
        }
    }

    fn get_amphipod_in_position(&self, position: &Position) -> Option<&Amphipod> {

        let mut amphipod_option : Option<&Amphipod> = None;
        for amphipod in self.amphipods.iter() {
            if amphipod.position == *position {
                amphipod_option = Some(amphipod);
            }
        }

        return amphipod_option;
    }

    fn get_mut_amphipod_in_position(&mut self, position: &Position) -> Option<&mut Amphipod> {

        let mut amphipod_option : Option<&mut Amphipod> = None;
        for amphipod in self.amphipods.iter_mut() {
            if amphipod.position == *position {
                amphipod_option = Some(amphipod);
            }
        }

        return amphipod_option;
    }

    fn get_room_in_position(&self, position: &Position) -> Option<&Room> {

        let mut room_option : Option<&Room> = None;
        for room in self.rooms.iter() {
            if room.positions.contains(position) {
                room_option = Some(room);
            }
        }

        return room_option;
    }

    fn room_ready_to_receive_amphypod(&self, room: &Room, amphipod: &Amphipod) -> bool {

        // Amphipod with wrong type must not enter
        match (room.room_type, amphipod.amphipod_type) {
            (RoomType::AmberRoom, AmphipodType::Amber) => (),
            (RoomType::BronzeRoom, AmphipodType::Bronze) => (),
            (RoomType::CopperRoom, AmphipodType::Copper) => (),
            (RoomType::DesertRoom, AmphipodType::Desert) => (),
            (_, _) => ( return false )
        }

        // Amphipod must not enter a room with amphipod from wrong type
        let amphipods_in_room : Vec<&Amphipod> = room.positions.iter()
            .map(|position| self.get_amphipod_in_position(position))
            .filter(|amphipod| amphipod.is_some())
            .map(|amphipod| amphipod.unwrap())
            .collect();
        for amphipod_in_room in amphipods_in_room.iter() {
            match (room.room_type, amphipod_in_room.amphipod_type) {
                (RoomType::AmberRoom, AmphipodType::Amber) => (),
                (RoomType::BronzeRoom, AmphipodType::Bronze) => (),
                (RoomType::CopperRoom, AmphipodType::Copper) => (),
                (RoomType::DesertRoom, AmphipodType::Desert) => (),
                (_, _) => ( return false )
            }
        } 

        return true;
    }

    fn transversable(&self, position_map: &HashMap<Position, PositionType>, position: Position, amphipod: &Amphipod) -> bool {

        let position_type = position_map.get(&position);
        let amphipod_in_position = self.get_amphipod_in_position(&position);

        // If not in map not valid
        if position_type.is_none() { return false }
        let position_type = position_type.unwrap();

        // Cannot move through walls
        if *position_type == PositionType::Wall { return false }
        // Cannot move to a place where there is another amphipod
        if amphipod_in_position.is_some() { return false }

        if *position_type == PositionType::Hallway { return true }
        if *position_type == PositionType::Room { return true }

        panic!("🚨  I think it should never reach this position");
    }

    fn movable_position(&self, position_map: &HashMap<Position, PositionType>, position: Position, amphipod: &Amphipod) -> bool {

        let position_type = position_map.get(&position);
        let room = self.get_room_in_position(&position);
        let amphipod_in_position = self.get_amphipod_in_position(&position);

        // If not in map not valid
        if position_type.is_none() { return false }
        let position_type = position_type.unwrap();

        // Cannot move through walls
        if *position_type == PositionType::Wall { return false }
        // Cannot move to a place where there is another amphipod
        if amphipod_in_position.is_some() { return false }
        
        // Cannot move to free spot if already moved to one before
        if *position_type == PositionType::Hallway && amphipod.moved_outside { return false }
        if *position_type == PositionType::Hallway && !amphipod.moved_outside { return true }

        if *position_type == PositionType::Room {

            let room = room.unwrap();
            if self.room_ready_to_receive_amphypod(room, amphipod) { return true }
            else { return false }

        }

        panic!("🚨  I think it should never reach this position");
    }

    fn get_movable_positions(&self, position_map: &HashMap<Position, PositionType>, amphipod: &Amphipod) -> Vec<(usize, Position)> {

        type Path = Vec<Position>;

        let mut valid_paths : Vec<Path> = Vec::new();
        let mut active_paths : Vec<Path> = vec!(vec!(amphipod.position));

        while active_paths.len() != 0 {

            let expand_path = active_paths.pop().unwrap();
            let last_position = expand_path.last().unwrap();
            for x_var in -1..=1 {
                for y_var in -1..=1 {

                    if x_var == 0 && y_var == 0 { continue }
                    if x_var != 0 && y_var != 0 { continue }

                    let new_position = (last_position.0 + x_var, last_position.1 + y_var);
                    if expand_path.contains(&new_position) { continue }

                    let transversable = self.transversable(position_map, new_position, amphipod);
                    if !transversable { continue }

                    let mut new_expanded_path = expand_path.clone();
                    new_expanded_path.push(new_position);
                    active_paths.push(new_expanded_path.clone());


                    let movable_to_position = self.movable_position(position_map, new_position, amphipod);
                    if movable_to_position { valid_paths.push(new_expanded_path) }
                }
            }
        }

        //println!("Valid Paths: {:?}", valid_paths);

        return valid_paths.into_iter()
            .map(|mut path| (path.len() - 1, path.pop().unwrap()))
            .collect();
    }

    fn scenario_finished(&self) -> bool {

        for amphipod in self.amphipods.iter() {

            let room = self.get_room_in_position(&amphipod.position);
            if room.is_none() { return false }
            let room = room.unwrap();

            match (room.room_type, amphipod.amphipod_type) {
                (RoomType::AmberRoom, AmphipodType::Amber) => (),
                (RoomType::BronzeRoom, AmphipodType::Bronze) => (),
                (RoomType::CopperRoom, AmphipodType::Copper) => (),
                (RoomType::DesertRoom, AmphipodType::Desert) => (),
                (_, _) => ( return false )
            }
        }

        return true;
    }

    fn create_scenarios_variation(&self, position_map: &HashMap<Position, PositionType>) -> Vec<Scenario> {

        let mut new_scenarios : Vec<Scenario> = Vec::new();
        for amphipod in self.amphipods.iter() {

            let amphipod_cost = amphipod.get_movement_cost();
            let movement_possibilities = self.get_movable_positions(position_map, amphipod);
            for movement_possibility in movement_possibilities.iter() {

                let mut new_scenario = self.clone();
                let position_type = position_map.get(&movement_possibility.1).unwrap();

                new_scenario.energy_consumed += amphipod_cost * movement_possibility.0 as Energy;
                let mut corresponding_amphipod = new_scenario.get_mut_amphipod_in_position(&amphipod.position).unwrap();
                corresponding_amphipod.position = movement_possibility.1;
                if *position_type == PositionType::Hallway { corresponding_amphipod.moved_outside = true; }

                new_scenarios.push(new_scenario);
            }
        }

        return new_scenarios;
    }
}

impl Map {

    pub fn new(info_lines: Vec<String>) -> Map {

        let map_limit_x = info_lines.iter().map(|line| line.len()).max().unwrap() as PositionUnit - 1;
        let map_limit_y = info_lines.len() as PositionUnit - 1;
        let map_limit : Position = (map_limit_x, map_limit_y);
        
        let room_type_order : Vec<RoomType> = vec!(RoomType::AmberRoom, RoomType::BronzeRoom,
            RoomType::CopperRoom, RoomType::DesertRoom);
        
        let mut map_positions : HashMap<Position, PositionType> = HashMap::new();

        let mut amphipods : Vec<Amphipod> = Vec::new();
        let mut room_info : HashMap<RoomType, HashSet<Position>> = HashMap::new();
        for &room_type in room_type_order.iter() { room_info.insert(room_type, HashSet::new()); }

        for (line_index, line) in info_lines.into_iter().enumerate() {

            let mut current_room_index : usize = 0;
            for (char_index, characther) in line.chars().into_iter().enumerate() {

                let current_position : Position = (char_index as PositionUnit, line_index as PositionUnit);
                match (line_index, characther) {
                    (_, ' ') => {},
                    (_, '#') => { map_positions.insert(current_position, PositionType::Wall); },
                    (line_index, '.') if line_index <= 1 => { map_positions.insert(current_position, PositionType::Hallway); },
                    (_, characther) => {
                        
                        map_positions.insert(current_position, PositionType::Room);
                        // Add to rooms
                        let room_type = room_type_order.get(current_room_index).unwrap();
                        let room_type_info = room_info.get_mut(room_type).unwrap();
                        room_type_info.insert(current_position);
                        // Add to Amphipod if justifiable
                        if characther != '.' {
                            let amphipod_type = convert_characther_to_amphipod(characther);
                            let amphipod : Amphipod = Amphipod::new(amphipod_type, current_position);
                            amphipods.push(amphipod);
                        }

                        current_room_index = current_room_index + 1;
                    },
                };
            }
        }

        let rooms : Vec<Room> = room_info.into_iter()
            .map(|(room_type, positions)| Room::new(room_type, positions.into_iter()
                .collect::<Vec<Position>>()))
            .collect();

        Map {
            map_size:           ((0, 0), map_limit),
            map_positions:      map_positions,
            starting_scenario:  Scenario::new(rooms, amphipods),
            final_scenario:     None,
        }
    }

    pub fn reach_final_scenario(&mut self) {

        let mut active_scenarios : Vec<Scenario> = vec!(self.starting_scenario.clone());
        while active_scenarios.len() != 0 {

            let current_scenario = active_scenarios.pop().unwrap();
            print!("\r⚙️  Processing {} active scenarios ( minimum = {} ) ...", 
                active_scenarios.len(), current_scenario.energy_consumed);

            //if self.final_scenario.is_some() && self.final_scenario.as_ref().unwrap().energy_consumed < current_scenario.energy_consumed { break }

            let mut generated_scenarios = current_scenario.create_scenarios_variation(&self.map_positions);
            //println!("Generated Scenarios: {}", generated_scenarios.len());
            for generated_scenario in generated_scenarios.iter() {
                if generated_scenario.scenario_finished() && (
                    self.final_scenario.is_none() || self.final_scenario.as_ref().unwrap().energy_consumed > generated_scenario.energy_consumed
                ) { self.final_scenario = Some(generated_scenario.clone()); }

                self.print(generated_scenario);
            }

            active_scenarios.append(&mut generated_scenarios);
            //active_scenarios.sort_by_key(|scenario| scenario.energy_consumed);
            //active_scenarios.reverse();
        }
    }

    pub fn get_energy_of_final_scenario(&self) -> Energy { self.final_scenario.as_ref().unwrap().energy_consumed }

    fn print(&self, scenario: &Scenario) {

        let mut print_info : String = String::new();
        for y_value in self.map_size.0.1..=self.map_size.1.1 {
            for x_value in self.map_size.0.0..=self.map_size.1.0 {

                let position = (x_value as PositionUnit, y_value as PositionUnit);

                let amphipod = scenario.get_amphipod_in_position(&position);
                let position_type = self.map_positions.get(&position);

                if amphipod.is_some() {
                    let amphipod_characther = convert_amphipod_type_to_characther(&amphipod.unwrap().amphipod_type);
                    print_info = format!("{}{}", print_info, amphipod_characther);
                } else if position_type.is_some() && *position_type.unwrap() == PositionType::Room {
                    let room_type = scenario.get_room_in_position(&position).unwrap();
                    let room_type_characther = convert_room_type_to_characther(&room_type.room_type);
                    print_info = format!("{}{}", print_info, room_type_characther);
                } else if position_type.is_some() {
                    let position_type_characther = convert_position_type_to_characther(position_type.unwrap());
                    print_info = format!("{}{}", print_info, position_type_characther);
                } else { print_info = format!("{} ", print_info) }
            }

            print_info = format!("{}\n", print_info);
        }

        println!("{}", print_info);
    }
}