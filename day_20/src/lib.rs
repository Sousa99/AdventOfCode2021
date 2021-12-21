use std::fmt;

use std::collections::{HashMap};

// ================================================== STRUCTS ==================================================

#[derive(PartialEq, Copy, Clone)]
enum SlotState { Emtpy, Filled }

struct Image {
    infinite_slot:          SlotState,
    slots:                  HashMap<(i64, i64), SlotState>,
    finite_dimension_start: (i64, i64),
    finite_dimension_end:   (i64, i64),
}

pub struct EnhancingTool {
    current_iteration:  usize,
    codification:       HashMap<i64, SlotState>,
    current_image:      Image,
}

const INFLUENCE_RADIUS : i64 = 1;

// ================================================== AUX FUNCTIONS ==================================================

fn convert_slot_state_to_symbol(slot: SlotState) -> char {
    match slot {
        SlotState::Emtpy => '.',
        SlotState::Filled => '#',
    }
}

fn convert_char_to_slot_state(slot_char: char) -> SlotState {
    match slot_char {
        '.' => SlotState::Emtpy,
        '#' => SlotState::Filled,
        _ => panic!("🚨  Unrecognized symbol for a slot '{}'", slot_char),
    }
}

fn get_code_from_chars(slots: Vec<SlotState>) -> usize {

    let mut binary_code : String = String::new();
    for slot in slots {

        let digit : char = match slot {
            SlotState::Emtpy => '0',
            SlotState::Filled => '1',
        };

        binary_code.push(digit);
    }

    return usize::from_str_radix(&binary_code, 2).unwrap()
}

// ================================================== IMPLEMENTATIONS ==================================================

impl EnhancingTool {

    pub fn new(codification: String, start_image_info: Vec<String>) -> EnhancingTool {

        let codification_fixed : HashMap<i64, SlotState> = codification.chars().into_iter().enumerate()
            .map(|(index, characther)| (index as i64, convert_char_to_slot_state(characther)))
            .collect();

        let rows : i64 = start_image_info.len() as i64;
        let columns : i64 = start_image_info.iter()
            .map(|info_line| info_line.len())
            .max().unwrap() as i64;
        let mut image_slots : HashMap<(i64, i64), SlotState> = HashMap::new();
        for (row_index, string_info) in start_image_info.into_iter().enumerate() {
            for (col_index, characther) in string_info.chars().into_iter().enumerate() {
                image_slots.insert((row_index as i64, col_index as i64), convert_char_to_slot_state(characther));
            }
        }

        let start_image : Image = Image {
            infinite_slot: SlotState::Emtpy,
            slots: image_slots,
            finite_dimension_start: (0, 0),
            finite_dimension_end: (rows, columns),
        };

        EnhancingTool {
            current_iteration: 0,
            codification: codification_fixed,
            current_image: start_image
        }
    }

    pub fn do_iteration(&mut self) {

        let new_image_start : (i64, i64) = ( self.current_image.finite_dimension_start.0 - INFLUENCE_RADIUS,
            self.current_image.finite_dimension_start.1 - INFLUENCE_RADIUS);
        let new_image_end : (i64, i64) = ( self.current_image.finite_dimension_end.0 + INFLUENCE_RADIUS,
            self.current_image.finite_dimension_end.1 + INFLUENCE_RADIUS);

        let mut infinite_code : Vec<SlotState> = Vec::new();
        for _ in 0..((2 * INFLUENCE_RADIUS + 1).pow(2)) { infinite_code.push(self.current_image.infinite_slot) }
        let infinite_slot_index : i64 = get_code_from_chars(infinite_code) as i64;
        let infinite_slot : SlotState = *self.codification.get(&infinite_slot_index).unwrap();

        let mut image_slots : HashMap<(i64, i64), SlotState> = HashMap::new();
        
        for row in new_image_start.0..=new_image_end.0 {
            for col in new_image_start.1..=new_image_end.1 {

                let mut slots : Vec<SlotState> = Vec::new();
                for var_row in -INFLUENCE_RADIUS..=INFLUENCE_RADIUS {
                    for var_col in -INFLUENCE_RADIUS..=INFLUENCE_RADIUS {

                        let slot_option = self.current_image.slots.get(&(row + var_row, col + var_col));
                        if slot_option.is_none() { slots.push(self.current_image.infinite_slot) }
                        else { slots.push(*slot_option.unwrap()) }
                    }
                }

                let slot_code_index : i64 = get_code_from_chars(slots) as i64;
                let slot_code : SlotState = *self.codification.get(&slot_code_index).unwrap();
                image_slots.insert((row, col), slot_code);
            }
        }

        let new_image : Image = Image {
            infinite_slot: infinite_slot,
            finite_dimension_start: new_image_start,
            finite_dimension_end: new_image_end,
            slots: image_slots,
        };

        self.current_image = new_image;
        self.current_iteration = self.current_iteration + 1;
    }

    pub fn get_current_iteration(&self) -> usize { self.current_iteration }

    pub fn compute_finite_number_of_characther(&self, characther: char) -> usize {
        self.current_image.slots.iter()
            .filter(|(_, &slot)| slot == convert_char_to_slot_state(characther))
            .count()
    }
}

impl fmt::Display for Image {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        let mut line = String::new();
        for row in (self.finite_dimension_start.0 - 1)..=(self.finite_dimension_end.0 + 1) {
            for col in (self.finite_dimension_start.1 - 1)..=(self.finite_dimension_end.1 + 1) {

                let slot_option = self.slots.get(&(row, col));
                if slot_option.is_none() { line = format!("{}{}", line, convert_slot_state_to_symbol(self.infinite_slot)) }
                else { line = format!("{}{}", line, convert_slot_state_to_symbol(slot_option.unwrap().clone())) }
            }

            line = format!("{}\n", line);
        }

        write!(f, "{}", line)?;
        return Ok(());
    }
}

impl fmt::Display for EnhancingTool {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        write!(f, "Current Image:\n{}", self.current_image)?;
        return Ok(());
    }
}