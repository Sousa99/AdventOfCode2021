use std::hash::Hash;
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet, BinaryHeap};

// ================================================== STRUCTS ==================================================

type PositionUnit = i32;
type Position = (PositionUnit, PositionUnit);

#[derive(Copy, Clone, PartialEq, Eq)]
enum AmphipodType { Amber, Bronze, Copper, Desert }

#[derive(Clone, PartialEq, Eq)]
struct Amphipod {
    amphipod_type:  AmphipodType,
    position:       Position,
    moved_outside:  bool,
    number_moves:   usize,
}

#[derive(PartialEq, Copy, Clone)]
enum PositionType { Wall, Hallway, Room }

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
enum RoomType { AmberRoom, BronzeRoom, CopperRoom, DesertRoom }

#[derive(Clone)]
struct Room {
    room_type:  RoomType,
    positions:  HashSet<Position>,
}

type Energy = i64;

#[derive(Clone, Eq)]
struct Node {
    energy_consumed:    Energy,
    amphipods:          Vec<Amphipod>,
}

pub struct Map {
    map_size:           (Position, Position),
    map_positions:      HashMap<Position, PositionType>,
    rooms:              HashMap<Position, Room>,
    // Node Graph
    starting_node:      Node,
    final_node:         Option<Node>,
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
            amphipod_type:  amphipod_type,
            position:       position,
            moved_outside:  false,
            number_moves:   0,
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

    fn get_code(&self) -> String {
        let characther : char = convert_amphipod_type_to_characther(&self.amphipod_type);
        return format!("{}:({},{}),{}", characther, self.position.0, self.position.1, self.moved_outside);
    }
}

impl Room {

    pub fn new(room_type: RoomType, positions: HashSet<Position>) -> Room {
        Room {
            room_type: room_type,
            positions: positions,
        }
    }
}

impl Node {

    pub fn new(amphipods: Vec<Amphipod>) -> Node {
        Node {
            energy_consumed: 0,
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

    fn get_code(&self) -> String {

        let mut amphipod_codes : Vec<String> = self.amphipods.iter()
            .map(|amphipod| amphipod.get_code())
            .collect();
        amphipod_codes.sort();

        return format!("{}:({})", self.energy_consumed, amphipod_codes.join("|"));
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

        if amphipod.number_moves >= 2 { return false }

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

    fn movable_position(&self, position_map: &HashMap<Position, PositionType>, rooms: &HashMap<Position, Room>, position: Position, amphipod: &Amphipod) -> bool {

        let position_type = position_map.get(&position);
        let room = rooms.get(&position);
        let amphipod_in_position = self.get_amphipod_in_position(&position);

        // Efficiency
        let position_type_under = position_map.get(&(position.0, position.1 - 1));

        // If not in map not valid
        if position_type.is_none() { return false }
        let position_type = position_type.unwrap();

        // Cannot move through walls
        if *position_type == PositionType::Wall { return false }
        // Cannot move to a place where there is another amphipod
        if amphipod_in_position.is_some() { return false }
        
        // Efficiency, do not block rooms
        if *position_type == PositionType::Hallway && position_type_under.is_some()
            && *position_type_under.unwrap() == PositionType::Room { return false }
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

    fn get_movable_positions(&self, position_map: &HashMap<Position, PositionType>, rooms: &HashMap<Position, Room>, amphipod: &Amphipod) -> Vec<(usize, Position)> {

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


                    let movable_to_position = self.movable_position(position_map, rooms, new_position, amphipod);
                    if movable_to_position { valid_paths.push(new_expanded_path) }
                }
            }
        }

        //println!("Valid Paths: {:?}", valid_paths);

        return valid_paths.into_iter()
            .map(|mut path| (path.len() - 1, path.pop().unwrap()))
            .collect();
    }

    fn finishing_node(&self, rooms: &HashMap<Position, Room>) -> bool {

        for amphipod in self.amphipods.iter() {

            let room = rooms.get(&amphipod.position);
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

    fn create_generated_nodes(&self, position_map: &HashMap<Position, PositionType>, rooms: &HashMap<Position, Room>) -> Vec<Node> {

        let mut new_nodes : Vec<Node> = Vec::new();
        for amphipod in self.amphipods.iter() {

            let amphipod_cost = amphipod.get_movement_cost();
            let movement_possibilities = self.get_movable_positions(position_map, rooms, amphipod);
            for movement_possibility in movement_possibilities.iter() {

                let mut new_node = self.clone();
                let position_type = position_map.get(&movement_possibility.1).unwrap();

                let added_energy = amphipod_cost * movement_possibility.0 as Energy;
                new_node.energy_consumed += added_energy;

                let mut corresponding_amphipod = new_node.get_mut_amphipod_in_position(&amphipod.position).unwrap();
                corresponding_amphipod.position = movement_possibility.1;
                corresponding_amphipod.number_moves += 1;

                if *position_type == PositionType::Hallway { corresponding_amphipod.moved_outside = true; }

                new_nodes.push(new_node);
            }
        }

        return new_nodes;
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.energy_consumed.cmp(&other.energy_consumed).reverse()
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.energy_consumed == other.energy_consumed
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

        let mut rooms : HashMap<Position, Room> = HashMap::new();
        for (room_type, positions) in room_info.into_iter() {
            let new_room : Room = Room::new(room_type, positions);
            for &position in new_room.positions.iter() {
                rooms.insert(position, new_room.clone());
            }
        }

        Map {
            map_size:           ((0, 0), map_limit),
            map_positions:      map_positions,
            rooms:              rooms,
            starting_node:      Node::new(amphipods),
            final_node:         None,
        }
    }

    pub fn reach_final_node(&mut self) {

        let mut active_nodes : BinaryHeap<Node> = BinaryHeap::new();
        active_nodes.push(self.starting_node.clone());
        while active_nodes.len() != 0 {

            let current_scenario = active_nodes.pop().unwrap();
            print!("\r⚙️  Processing {} active scenarios ( minimum = {} ) ...", 
                active_nodes.len(), current_scenario.energy_consumed);

            if self.final_node.is_some() && self.final_node.as_ref().unwrap().energy_consumed < current_scenario.energy_consumed { break }
            
            let nodes = current_scenario.create_generated_nodes(&self.map_positions, &self.rooms);
            for generated_node in nodes.into_iter() {
                if generated_node.finishing_node(&self.rooms) && (
                    self.final_node.is_none() || self.final_node.as_ref().unwrap().energy_consumed > generated_node.energy_consumed
                ) { self.final_node = Some(generated_node.clone()); }
                
                let code_of_generated = generated_node.get_code();
                
                //self.print(&generated_node);
                //println!("{}", code_of_generated);
                active_nodes.push(generated_node);
            }
        }
    }

    pub fn get_energy_of_final_node(&self) -> Energy { self.final_node.as_ref().unwrap().energy_consumed }

    fn print(&self, node: &Node) {

        let mut print_info : String = String::new();
        for y_value in self.map_size.0.1..=self.map_size.1.1 {
            for x_value in self.map_size.0.0..=self.map_size.1.0 {

                let position = (x_value as PositionUnit, y_value as PositionUnit);

                let amphipod = node.get_amphipod_in_position(&position);
                let position_type = self.map_positions.get(&position);

                if amphipod.is_some() {
                    let amphipod_characther = convert_amphipod_type_to_characther(&amphipod.unwrap().amphipod_type);
                    print_info = format!("{}{}", print_info, amphipod_characther);
                } else if position_type.is_some() && *position_type.unwrap() == PositionType::Room {
                    let room_type = self.rooms.get(&position).unwrap();
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