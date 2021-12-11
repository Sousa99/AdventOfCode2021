use std::fmt;
use std::collections::HashMap;

use colored::*;

// ================================================== STRUCTS ==================================================

type EnergyLevel = u32;
type PositionUnit = i64;
type Position = (PositionUnit, PositionUnit);

const RESET_ENERGY_LEVEL : EnergyLevel = 0;
const FLASH_ENERGY_LEVEL : EnergyLevel = 9;
#[derive(PartialEq)]
enum FlashOption { Flahsed, NotFlashed }
struct DumboOctopus {
    energy_level:           EnergyLevel,
    activated_this_iter:    bool
}

pub struct Map {
    iteration:          u64,
    number_of_flashes:  u64,
    map_size:           Position,
    octopi:             HashMap<Position, DumboOctopus>
}

// ================================================== AUX FUNCTIONS ==================================================

// ================================================== IMPLEMENTATIONS ==================================================

impl DumboOctopus {

    fn new(initial_energy: EnergyLevel) -> DumboOctopus {
        DumboOctopus {
            energy_level: initial_energy,
            activated_this_iter: false,
        }
    }

    fn increment_energy_level(&mut self) -> FlashOption {

        self.energy_level = self.energy_level + 1;
        if self.energy_level > FLASH_ENERGY_LEVEL {

            self.energy_level = RESET_ENERGY_LEVEL;
            return FlashOption::Flahsed;
        }

        return FlashOption::NotFlashed;
    }
}

impl Map {

    pub fn new(energy_levels: Vec<Vec<EnergyLevel>>) -> Map {

        let number_of_rows : usize = energy_levels.len();
        let number_of_columns : usize = energy_levels.iter()
            .map(|row| row.len()).max().unwrap();

        let octopi : HashMap<Position, DumboOctopus>= energy_levels.into_iter().enumerate()
            .map(|(row_index, row)| row.into_iter().enumerate()
                .map(|(col_index, energy)| ((row_index as PositionUnit, col_index as PositionUnit), DumboOctopus::new(energy)))
                .collect::<Vec<(Position, DumboOctopus)>>())
            .collect::<Vec<Vec<(Position, DumboOctopus)>>>()
            .into_iter().flatten()
            .collect::<Vec<(Position, DumboOctopus)>>()
            .into_iter()
            .collect::<HashMap<Position, DumboOctopus>>();

        Map {
            iteration: 0,
            number_of_flashes: 0,
            map_size: (number_of_rows as PositionUnit, number_of_columns as PositionUnit) as Position,
            octopi: octopi,
        }
    }

    pub fn get_number_of_flashes(&self) -> u64 { self.number_of_flashes }
    pub fn get_iteration(&self) -> u64 { self.iteration }

    fn reset_octopi_iteration(&mut self) -> bool {

        let mut all_flash : bool = true;
        for (_, octopi) in self.octopi.iter_mut() {

            if !octopi.activated_this_iter {
                all_flash = false;
                continue;
            }

            octopi.activated_this_iter = false;
        }

        return all_flash;
    }

    pub fn do_iteration(&mut self) -> bool {

        let mut positions_flashed : Vec<Position> = Vec::new();
        // Increment all Octopi one value
        for (&octopi_position, octopi) in self.octopi.iter_mut() {

            let flashed : bool = octopi.increment_energy_level() == FlashOption::Flahsed;
            if flashed {

                octopi.activated_this_iter = true;
                positions_flashed.push(octopi_position);
                self.number_of_flashes = self.number_of_flashes + 1;
            }
        }

        // Increment neighbours
        while positions_flashed.len() != 0 {

            let octopi_position = positions_flashed.pop().unwrap();
            for row_var in -1..=1 {
                for col_var in -1..=1 {

                    if row_var == 0 && col_var == 0 { continue }
                    let neighbour_octopi_position = ( octopi_position.0 + row_var, octopi_position.1 + col_var );
                    let neighbour_option = self.octopi.get_mut(&neighbour_octopi_position);

                    if neighbour_option.is_none() { continue }
                    let neighbour = neighbour_option.unwrap();

                    if neighbour.activated_this_iter { continue }
                    
                    let flashed : bool = neighbour.increment_energy_level() == FlashOption::Flahsed;
                    if flashed {

                        neighbour.activated_this_iter = true;
                        positions_flashed.push(neighbour_octopi_position);
                        self.number_of_flashes = self.number_of_flashes + 1;
                    }
                }
            }
        }

        // Update Iteration Counter
        self.iteration = self.iteration + 1;
        // Reset Octopi
        let all_flashed : bool = self.reset_octopi_iteration();
        return all_flashed;
    }
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        let mut line : String = format!("Iteration: {}\n", self.iteration);
        for row_index in 0..self.map_size.0 {

            for col_index in 0..self.map_size.1 {

                let octopi = self.octopi.get(&(row_index, col_index)).unwrap();
                if octopi.energy_level == 0 { line = format!("{} {}", line, "0".green()) }
                else { line = format!("{} {}", line, octopi.energy_level) }
            }

            line = format!("{}\n", line);

        }

        return write!(f, "{}", line);
    }
}