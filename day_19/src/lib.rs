use std::collections::{HashMap};

use strum::IntoEnumIterator;
use strum_macros::EnumIter;

// ================================================== STRUCTS ==================================================

type PositionUnit = i64;
type Position = (PositionUnit, PositionUnit, PositionUnit);

#[derive(PartialEq, Copy, Clone)]
enum MapEntry { Scanner, Beacon }

#[derive(Clone)]
struct ScannerInfo {
    identification:             String,
    scanner_absolute_position:  Option<Position>,
    relative_positions:         Vec<Position>,
    absolute_positions:         Option<Vec<Position>>,
}

pub struct Map {
    entry_map:              HashMap<Position, MapEntry>,
    scanners:               Vec<ScannerInfo>,
}

const MAX_DISTANCE_DETECTABLE : PositionUnit = 1000;
const THRESHOLD : usize = 3;

#[derive(EnumIter, PartialEq, Copy ,Clone)]
enum FacingAxis { X, Y, Z, NegX, NegY, NegZ }
#[derive(EnumIter, PartialEq, Copy, Clone)]
enum UpAxis { X, Y, Z, NegX, NegY, NegZ }

// ================================================== AUX FUNCTIONS ==================================================



// ================================================== IMPLEMENTATIONS ==================================================

impl ScannerInfo {
    
    fn new(id: String, positions: Vec<Position>) -> ScannerInfo {
        ScannerInfo {
            identification: id,
            scanner_absolute_position: None,
            relative_positions: positions,
            absolute_positions: None,
        }
    }

    pub fn get_permutation(&self, axis_facing: FacingAxis, axis_up: UpAxis) -> Option<Vec<Position>> {

        match (axis_facing, axis_up) {
            (FacingAxis::X, UpAxis::X) => return None,
            (FacingAxis::X, UpAxis::NegX) => return None,
            (FacingAxis::NegX, UpAxis::X) => return None,
            (FacingAxis::NegX, UpAxis::NegX) => return None,
            (FacingAxis::Y, UpAxis::Y) => return None,
            (FacingAxis::Y, UpAxis::NegY) => return None,
            (FacingAxis::NegY, UpAxis::Y) => return None,
            (FacingAxis::NegY, UpAxis::NegY) => return None,
            (FacingAxis::Z, UpAxis::Z) => return None,
            (FacingAxis::Z, UpAxis::NegZ) => return None,
            (FacingAxis::NegZ, UpAxis::Z) => return None,
            (FacingAxis::NegZ, UpAxis::NegZ) => return None,
            _ => (),
        };

        let mut permutations : Vec<Position> = Vec::new();
        for &value in self.relative_positions.iter() {

            let value_permed = match (axis_facing, axis_up) {
                (FacingAxis::X, UpAxis::Z) => (value.0, value.1, value.2),
                (FacingAxis::Y, UpAxis::Z) => (- value.1, value.0, value.2),
                (FacingAxis::NegX, UpAxis::Z) => (- value.0, - value.1, value.2),
                (FacingAxis::NegY, UpAxis::Z) => (value.1, - value.0, value.2),
                (FacingAxis::X, UpAxis::NegZ) => (value.0, - value.1, - value.2),
                (FacingAxis::Y, UpAxis::NegZ) => (value.1, value.0, - value.2),
                (FacingAxis::NegX, UpAxis::NegZ) => (- value.0, value.1, - value.2),
                (FacingAxis::NegY, UpAxis::NegZ) => (- value.1, - value.0, - value.2),

                (FacingAxis::X, UpAxis::Y) => (value.0, - value.2, value.1),
                (FacingAxis::Z, UpAxis::Y) => (value.2, value.0, value.1),
                (FacingAxis::NegX, UpAxis::Y) => (- value.0, value.2, value.1),
                (FacingAxis::NegZ, UpAxis::Y) => (- value.2, - value.0, value.1),
                (FacingAxis::X, UpAxis::NegY) => (value.0, value.2, - value.1),
                (FacingAxis::Z, UpAxis::NegY) => (- value.2, value.0, - value.1),
                (FacingAxis::NegX, UpAxis::NegY) => (- value.0, - value.2, - value.1),
                (FacingAxis::NegZ, UpAxis::NegY) => (value.2, - value.0, - value.1),

                (FacingAxis::Y, UpAxis::X) => (value.2, - value.1, value.0),
                (FacingAxis::Z, UpAxis::X) => (value.1, value.2, value.0),
                (FacingAxis::NegY, UpAxis::X) => (- value.2, value.1, value.0),
                (FacingAxis::NegZ, UpAxis::X) => (- value.1, - value.2, value.0),
                (FacingAxis::Y, UpAxis::NegX) => (- value.2, - value.1, - value.0),
                (FacingAxis::Z, UpAxis::NegX) => (- value.1, value.2, - value.0),
                (FacingAxis::NegY, UpAxis::NegX) => (value.2, value.1, - value.0),
                (FacingAxis::NegZ, UpAxis::NegX) => (value.1, - value.2, - value.0),

                _ => panic!("🚨  Should not be reached")
            };
            
            permutations.push(value_permed);
        }

        return Some(permutations);
    }
}

impl Map {

    pub fn new(scanners_values: Vec<(String, Vec<Vec<PositionUnit>>)>) -> Map {

        let scanners_infos : Vec<ScannerInfo> = scanners_values.into_iter()
            .map(|scanner_values| {

                let positions = scanner_values.1.into_iter()
                    .map(|values| (values[0], values[1], values[2]))
                    .collect();
                
                return ScannerInfo::new(scanner_values.0, positions); })
            .collect();

        Map {
            entry_map: HashMap::new(),
            scanners: scanners_infos,
        }
    }

    pub fn get_number_of_scanners(&self) -> usize { self.scanners.len() }

    pub fn number_of_scanner_processed(&self) -> usize {

        let mut count = 0;
        for scanner in self.scanners.iter() {
            if scanner.absolute_positions.is_some() {
                count = count + 1;
            }
        }

        return count;
    }

    pub fn add_next_scanner_info(&mut self) {
        // First can be blindly added
        if self.number_of_scanner_processed() == 0 {
            
            let mut absolute_positions : Vec<Position> = Vec::new();
            let scanner_info = self.scanners.get_mut(0).unwrap();
            self.entry_map.insert((0, 0, 0), MapEntry::Scanner);
            for &position in scanner_info.relative_positions.iter() {
                self.entry_map.insert(position, MapEntry::Beacon);
                absolute_positions.push(position);
            }
            
            scanner_info.scanner_absolute_position = Some((0, 0, 0));
            scanner_info.absolute_positions = Some(absolute_positions);
            return
        }

        else {

            let scanners_processed_values : Vec<(String, Position, Vec<Position>)> = self.scanners.iter()
                .filter(|scanner| scanner.absolute_positions.is_some())
                .map(|scanner| (scanner.identification.clone(), scanner.scanner_absolute_position.unwrap(), scanner.absolute_positions.clone().unwrap()))
                .collect();
            let scanners_unprocessed : Vec<&mut ScannerInfo> = self.scanners.iter_mut()
                .filter(|scanner| scanner.absolute_positions.is_none())
                .collect();
            for scanner_unprocessed in scanners_unprocessed.into_iter() {
                for facing_axis in FacingAxis::iter() {
                    for up_axis in UpAxis::iter() {

                        let unprocessed_values_to_check_option = scanner_unprocessed.get_permutation(facing_axis, up_axis);
                        if unprocessed_values_to_check_option.is_none() { continue }

                        let unprocessed_values_to_check = unprocessed_values_to_check_option.unwrap();

                        for processed_values_to_check in scanners_processed_values.iter() {

                            for link_unprocessed_value in unprocessed_values_to_check.iter() {
                                for link_processed_value in processed_values_to_check.2.iter() {

                                    let possible_origin = (link_processed_value.0 - link_unprocessed_value.0,
                                        link_processed_value.1 - link_unprocessed_value.1,
                                        link_processed_value.2 - link_unprocessed_value.2);
                                    let absolute_values : Vec<Position> = unprocessed_values_to_check.iter()
                                        .map(|value| (possible_origin.0 + value.0, possible_origin.1 + value.1, possible_origin.2 + value.2))
                                        .collect();
                                    
                                    let mut count_matches : usize = 0;
                                    let mut invalid : bool = false;

                                    for &absolute_value in absolute_values.iter() {

                                        let distance_processed_origin = ((absolute_value.0 - processed_values_to_check.1.0).abs(),
                                            (absolute_value.1 - processed_values_to_check.1.1).abs(),
                                            (absolute_value.2 - processed_values_to_check.1.2).abs());

                                        if processed_values_to_check.2.contains(&absolute_value) {
                                            count_matches = count_matches + 1;
                                        } else if distance_processed_origin.0 <= MAX_DISTANCE_DETECTABLE &&
                                            distance_processed_origin.1 <= MAX_DISTANCE_DETECTABLE &&
                                            distance_processed_origin.2 <= MAX_DISTANCE_DETECTABLE {
                                            invalid = true;
                                        }
                                    }

                                    for &processed_value in processed_values_to_check.2.iter() {

                                        let distance_processed_origin = ((possible_origin.0 - processed_value.0).abs(),
                                            (possible_origin.1 - processed_value.1).abs(),
                                            (possible_origin.2 - processed_value.2).abs());

                                        if !absolute_values.contains(&processed_value) &&
                                            distance_processed_origin.0 <= MAX_DISTANCE_DETECTABLE &&
                                            distance_processed_origin.1 <= MAX_DISTANCE_DETECTABLE &&
                                            distance_processed_origin.2 <= MAX_DISTANCE_DETECTABLE {
                                            invalid = true;
                                        }
                                    }

                                    if count_matches >= THRESHOLD && !invalid {

                                        println!("🪞  Matched '{}' and '{}'", processed_values_to_check.0, scanner_unprocessed.identification);

                                        self.entry_map.insert(possible_origin, MapEntry::Scanner);
                                        for &position in absolute_values.iter() {
                                            self.entry_map.insert(position, MapEntry::Beacon);
                                        }
                                        
                                        scanner_unprocessed.scanner_absolute_position = Some(possible_origin);
                                        scanner_unprocessed.absolute_positions = Some(absolute_values);

                                        return
                                    }
                                }
                            }
                        }
                    }
                }

                
            }
        }

        panic!("🚨  No scanner info was able of being added");
    }

    pub fn compute_number_of_beacons(&self) -> usize {

        let mut count : usize = 0;
        for (_, &map_entry) in self.entry_map.iter() {
            if map_entry == MapEntry::Beacon {
                count = count + 1;
            }
        }

        return count;
    }

    pub fn largest_distance_scanners(&self) -> Option<(String, String, PositionUnit)> {

        let mut max_distance : Option<(String, String, PositionUnit)> = None;
        for scanner_info_1 in self.scanners.iter() {
            for scanner_info_2 in self.scanners.iter() {

                let scanner_1_absolute : Position = scanner_info_1.scanner_absolute_position.unwrap();
                let scanner_2_absolute : Position = scanner_info_2.scanner_absolute_position.unwrap();

                let distance_vector : Position = (scanner_1_absolute.0 - scanner_2_absolute.0,
                    scanner_1_absolute.1 - scanner_2_absolute.1,
                    scanner_1_absolute.2 - scanner_2_absolute.2);

                let distance = distance_vector.0.abs() + distance_vector.1.abs() + distance_vector.2.abs();

                if max_distance.is_none() || distance > max_distance.as_ref().unwrap().2 {
                    max_distance = Some((scanner_info_1.identification.clone(), scanner_info_2.identification.clone(), distance));
                }

            }
        }

        return max_distance;
    }
}