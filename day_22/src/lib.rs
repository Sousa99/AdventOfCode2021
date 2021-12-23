use std::fmt;

use std::collections::{HashMap};
use std::io::{Write, stdout};

// ================================================== STRUCTS ==================================================

#[derive(PartialEq, Copy, Clone)]
pub enum CubeState { Off, On }

struct Cube {
    state:  CubeState,
}

type PositionUnit = i64;
type Position = (PositionUnit, PositionUnit, PositionUnit);

struct Rule {
    to_state:   CubeState,
    x_range:    (PositionUnit, PositionUnit),
    y_range:    (PositionUnit, PositionUnit),
    z_range:    (PositionUnit, PositionUnit),
}

pub struct LimitedMap {
    iteration:  usize,
    rules:      Vec<Rule>,
    cube_map:   HashMap<Position, Cube>,
    limit_x:    Option<(PositionUnit, PositionUnit)>,
    limit_y:    Option<(PositionUnit, PositionUnit)>,
    limit_z:    Option<(PositionUnit, PositionUnit)>
}

#[derive(Copy, Clone)]
struct Region {
    start_position: Position,
    end_position:   Position
}

pub struct UnlimitedMap {
    iteration:          usize,
    rules:              Vec<Rule>,
    current_regions:    Vec<Region>
}

// ================================================== AUX FUNCTIONS ==================================================

fn convert_string_to_state(state_str: &str) -> CubeState {
    match state_str {
        "off" => CubeState::Off,
        "on" => CubeState::On,
        _ => panic!("🚨  Unrecognized state '{}'", state_str),
    }
}

fn limit_range(range: (PositionUnit, PositionUnit), limit: (PositionUnit, PositionUnit)) -> (PositionUnit, PositionUnit) {

    let range_start_limitted = std::cmp::max(range.0, limit.0);
    let range_end_limitted = std::cmp::min(range.1, limit.1);
    
    return (range_start_limitted, range_end_limitted);
}

// ================================================== IMPLEMENTATIONS ==================================================

impl Cube {

    fn change_to_state(&mut self, to_state: CubeState) { self.state = to_state }
}

impl Rule {

    fn new(state_str: String, ranges: Vec<(char, PositionUnit, PositionUnit)>) -> Rule {

        let state = convert_string_to_state(&state_str);
        let mut x_info : Option<(PositionUnit, PositionUnit)> = None;
        let mut y_info : Option<(PositionUnit, PositionUnit)> = None;
        let mut z_info : Option<(PositionUnit, PositionUnit)> = None;

        for range in ranges.into_iter() {

            let axis_characther : char = range.0;
            if axis_characther == 'x' { x_info = Some((range.1, range.2)) }
            if axis_characther == 'y' { y_info = Some((range.1, range.2)) }
            if axis_characther == 'z' { z_info = Some((range.1, range.2)) }
        }

        if x_info.is_none() { panic!("🚨  X Range not defined for the given rule") }
        if y_info.is_none() { panic!("🚨  Y Range not defined for the given rule") }
        if z_info.is_none() { panic!("🚨  Z Range not defined for the given rule") }

        Rule {
            to_state: state,
            x_range: x_info.unwrap(),
            y_range: y_info.unwrap(),
            z_range: z_info.unwrap(),
        }
    }
}

impl LimitedMap {

    pub fn new(rules_info: Vec<(String, Vec<(char, PositionUnit, PositionUnit)>)>, limits_info: Vec<(char, PositionUnit, PositionUnit)>) -> LimitedMap {

        let rules : Vec<Rule> = rules_info.into_iter()
            .map(|rule_info| Rule::new(rule_info.0, rule_info.1))
            .collect();

        let mut x_info_limit : Option<(PositionUnit, PositionUnit)> = None;
        let mut y_info_limit : Option<(PositionUnit, PositionUnit)> = None;
        let mut z_info_limit : Option<(PositionUnit, PositionUnit)> = None;
        for range in limits_info.into_iter() {
            let axis_characther : char = range.0;
            if axis_characther == 'x' { x_info_limit = Some((range.1, range.2)) }
            if axis_characther == 'y' { y_info_limit = Some((range.1, range.2)) }
            if axis_characther == 'z' { z_info_limit = Some((range.1, range.2)) }
        }

        LimitedMap {
            iteration: 0,
            rules: rules,
            cube_map: HashMap::new(),
            limit_x: x_info_limit,
            limit_y: y_info_limit,
            limit_z: z_info_limit,
        }
    }

    pub fn completed_rules(&self) -> bool { self.rules.len() == self.iteration }
    pub fn compute_with_state(&self, state: CubeState) -> usize {
        self.cube_map.iter()
            .filter(|(_, cube)| cube.state == state)
            .count()
    }

    pub fn do_iteration(&mut self) {

        if self.completed_rules() { return }

        print!("\r⚙️  Processing {} out of {} ({:.2}%)...", 
            self.iteration + 1,
            self.rules.len(),
            (self.iteration as f64 + 1.0) / (self.rules.len() as f64) * 100.0 );

        stdout().flush().unwrap();
        
        let target_rule : &Rule = self.rules.get(self.iteration).unwrap();
        let mut x_range = target_rule.x_range;
        let mut y_range = target_rule.y_range;
        let mut z_range = target_rule.z_range;

        if self.limit_x.is_some() { x_range = limit_range(x_range, self.limit_x.unwrap()) }
        if self.limit_y.is_some() { y_range = limit_range(y_range, self.limit_y.unwrap()) }
        if self.limit_z.is_some() { z_range = limit_range(z_range, self.limit_z.unwrap()) }

        for x_value in x_range.0..=x_range.1 {
            for y_value in y_range.0..=y_range.1 {
                for z_value in z_range.0..=z_range.1 {

                    let position : Position = (x_value, y_value, z_value);
                    if self.cube_map.contains_key(&position) {
                        let set_cube = self.cube_map.get_mut(&position).unwrap();
                        set_cube.change_to_state(target_rule.to_state);
                    } else if target_rule.to_state == CubeState::On {
                        let new_cube : Cube = Cube{ state: target_rule.to_state };
                        self.cube_map.insert(position, new_cube);
                    }

                }
            }
        }

        // Update iteration
        self.iteration = self.iteration + 1;
    }
}

impl Region {

    fn new(start_point: Position, end_point: Position) -> Region {

        Region {
            start_position: start_point,
            end_position: end_point,
        }
    }

    fn compute_size(&self) -> i128 {

        let x_range = (self.start_position.0 - self.end_position.0).abs() as i128 + 1;
        let y_range = (self.start_position.1 - self.end_position.1).abs() as i128 + 1;
        let z_range = (self.start_position.2 - self.end_position.2).abs() as i128 + 1;

        return x_range * y_range * z_range;
    }

    fn subtract_region(&self, region_to_remove: &Region) -> Vec<Region> {

        if self.end_position.0 < region_to_remove.start_position.0 || self.start_position.0 > region_to_remove.end_position.0 ||
            self.end_position.1 < region_to_remove.start_position.1 || self.start_position.1 > region_to_remove.end_position.1 ||
            self.end_position.2 < region_to_remove.start_position.2 || self.start_position.2 > region_to_remove.end_position.2 {
                return vec!(self.clone());
            }

        let intersection_x_lower = std::cmp::max(self.start_position.0, region_to_remove.start_position.0) - 1;
        let intersection_x_higher = std::cmp::min(self.end_position.0, region_to_remove.end_position.0) + 1;
        let intersection_y_lower = std::cmp::max(self.start_position.1, region_to_remove.start_position.1) - 1;
        let intersection_y_higher = std::cmp::min(self.end_position.1, region_to_remove.end_position.1) + 1;
        let intersection_z_lower = std::cmp::max(self.start_position.2, region_to_remove.start_position.2) - 1;
        let intersection_z_higher = std::cmp::min(self.end_position.2, region_to_remove.end_position.2) + 1;

        let mut new_regions : Vec<Region> = Vec::new();
        // 1st Bigger Solid on X's
        new_regions.push(Region::new((self.start_position.0, self.start_position.1, self.start_position.2),
            (intersection_x_lower, self.end_position.1, self.end_position.2)));
        new_regions.push(Region::new((intersection_x_higher, self.start_position.1, self.start_position.2),
            (self.end_position.0, self.end_position.1, self.end_position.2)));
        // 2nd Bigger Solid on Y's
        new_regions.push(Region::new((intersection_x_lower + 1, self.start_position.1, self.start_position.2),
            (intersection_x_higher - 1, intersection_y_lower, self.end_position.2)));
        new_regions.push(Region::new((intersection_x_lower + 1, intersection_y_higher, self.start_position.2),
            (intersection_x_higher - 1, self.end_position.1, self.end_position.2)));
        // 3rd Bigger Solid on Z's
        new_regions.push(Region::new((intersection_x_lower + 1, intersection_y_lower + 1, self.start_position.2),
            (intersection_x_higher - 1, intersection_y_higher - 1, intersection_z_lower)));
        new_regions.push(Region::new((intersection_x_lower + 1, intersection_y_lower + 1, intersection_z_higher),
            (intersection_x_higher - 1, intersection_y_higher - 1, self.end_position.2)));

        let new_regions_filtered : Vec<Region> = new_regions.into_iter()
            .filter(|region| {

                let x_invalid : bool = region.end_position.0 < region.start_position.0;
                let y_invalid : bool = region.end_position.1 < region.start_position.1;
                let z_invalid : bool = region.end_position.2 < region.start_position.2;

                return !(x_invalid || y_invalid || z_invalid);

            }).collect();

        return new_regions_filtered;
    }

    fn compute_intersection(&self, other_region: &Region) -> Option<Region> {
     
        let intersection_x_lower = std::cmp::max(self.start_position.0, other_region.start_position.0);
        let intersection_x_higher = std::cmp::min(self.end_position.0, other_region.end_position.0);
        let intersection_y_lower = std::cmp::max(self.start_position.1, other_region.start_position.1);
        let intersection_y_higher = std::cmp::min(self.end_position.1, other_region.end_position.1);
        let intersection_z_lower = std::cmp::max(self.start_position.2, other_region.start_position.2);
        let intersection_z_higher = std::cmp::min(self.end_position.2, other_region.end_position.2);

        let region = Region::new((intersection_x_lower, intersection_y_lower, intersection_z_lower),
            (intersection_x_higher, intersection_y_higher, intersection_z_higher));

        let x_invalid : bool = region.end_position.0 < region.start_position.0;
        let y_invalid : bool = region.end_position.1 < region.start_position.1;
        let z_invalid : bool = region.end_position.2 < region.start_position.2;

        if x_invalid || y_invalid || z_invalid { return None; }
        else { return Some(region); }
    }
}

impl UnlimitedMap {

    pub fn new(rules_info: Vec<(String, Vec<(char, PositionUnit, PositionUnit)>)>) -> UnlimitedMap {

        let rules : Vec<Rule> = rules_info.into_iter()
            .map(|rule_info| Rule::new(rule_info.0, rule_info.1))
            .collect();

        UnlimitedMap {
            iteration: 0,
            rules: rules,
            current_regions: Vec::new(),
        }
    }

    pub fn completed_rules(&self) -> bool { self.rules.len() == self.iteration }
    pub fn compute_on_state(&mut self) -> i128 {

        let intersection_size = self.compute_intersection_size();
        let full_size : i128 = self.current_regions.iter()
            .map(|region| region.compute_size())
            .sum();

        return full_size - intersection_size;
    }

    pub fn do_iteration(&mut self) {

        if self.completed_rules() { return }

        print!("\r⚙️  Processing {} out of {} ({:.2}%)...", 
            self.iteration + 1,
            self.rules.len(),
            (self.iteration as f64 + 1.0) / (self.rules.len() as f64) * 100.0 );
        stdout().flush().unwrap();
        
        let target_rule : &Rule = self.rules.get(self.iteration).unwrap();
        let target_region : Region = Region::new((target_rule.x_range.0, target_rule.y_range.0, target_rule.z_range.0),
            (target_rule.x_range.1, target_rule.y_range.1, target_rule.z_range.1));
            
        match target_rule.to_state {
            CubeState::Off => {
                
                let mut new_regions : Vec<Region> = Vec::new();
                for region in self.current_regions.iter() {
                    let mut sub_regions : Vec<Region> = region.subtract_region(&target_region);
                    new_regions.append(&mut sub_regions);
                }
                self.current_regions = new_regions;
            },

            CubeState::On => {
                self.current_regions.push(target_region);
            }
        }
        
        // Update iteration
        self.iteration = self.iteration + 1;
    }

    fn compute_intersection_size(&self) -> i128 {

        let mut intersection_size : i128 = 0;
        for region_index in 0..self.current_regions.len() {
            
            print!("\r⚙️  Processing 'region intersection' {} out of {} ({:.2}%)...", 
                region_index + 1,
                self.current_regions.len(),
                (region_index as f64 + 1.0) / (self.current_regions.len() as f64) * 100.0 );
            
            let mut intersections : Vec<Region> = Vec::new();
            let region = self.current_regions.get(region_index).unwrap();
            for other_region_index in (region_index + 1)..self.current_regions.len() {
                
                let other_region = self.current_regions.get(other_region_index).unwrap();
                let intersection = region.compute_intersection(other_region);
                if intersection.is_some() {
                    
                    let intersection = intersection.unwrap();
                    let mut new_intersections : Vec<Region> = vec!(intersection);
                    for set_intersection in intersections.iter() {

                        let mut new_new_intersections : Vec<Region> = Vec::new();
                        for sub_new_intersection in new_intersections.iter() {
                            let mut sub_new_intersection_corrected = sub_new_intersection.subtract_region(set_intersection);
                            new_new_intersections.append(&mut sub_new_intersection_corrected);
                        }
                        new_intersections = new_new_intersections;
                        
                    }
                    intersections.append(&mut new_intersections);
                }
            }

            intersection_size += intersections.iter()
                .map(|intersection| intersection.compute_size())
                .sum::<i128>();
        }

        return intersection_size;
    }
}

impl fmt::Display for Region {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        let start : String = format!("({}, {}, {})", self.start_position.0, self.start_position.1, self.start_position.2);
        let end : String = format!("({}, {}, {})", self.end_position.0, self.end_position.1, self.end_position.2);
        return write!(f, "{} => {}", start, end);
    }
}