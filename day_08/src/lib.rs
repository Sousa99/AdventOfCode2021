use std::collections::HashMap;

// ================================================== STRUCTS ==================================================

type Code = String;
type Digit = u64;

pub struct EntryProblem {
    map_codes:      HashMap<Code, Vec<Digit>>,
    output_codes:   Vec<Code>,
    output:         Vec<Option<Digit>>,
}

pub struct Display {
    entry_problems:     Vec<EntryProblem>,
}

// ================================================== AUX FUNCTIONS ==================================================

fn sort_string(string: String) -> String {

    let string_slice: &str = &string[..];
    let mut chars: Vec<char> = string_slice.chars().collect();
    chars.sort_by(|a, b| b.cmp(a));

    let final_string : String = chars.iter().collect();
    return final_string;
}

fn common_characthers(string_1: String, string_2: String) -> usize {

    let mut count = 0;
    for char_1 in string_1.chars() {
        if string_2.contains(char_1) {
            count = count + 1;
        }
    }

    return count;
}

// ================================================== IMPLEMENTATIONS ==================================================

impl EntryProblem {

    pub fn new(patterns: Vec<Code>, output_codes: Vec<Code>) -> EntryProblem {

        let mut map_codes: HashMap<Code, Vec<Digit>> = HashMap::new();
        let digits: Vec<Digit> = (0..10).collect();
        for code in patterns.iter() { map_codes.insert(sort_string(code.clone()), digits.clone()); }

        EntryProblem {
            map_codes: map_codes,
            output_codes: output_codes.iter().map(|code| sort_string(code.clone())).collect(),
            output: Vec::new(),
        }
    }

    fn get_output(&self) -> Vec<Option<Digit>> { self.output.clone() }
    fn get_output_number(&self) -> Option<u64> {

        let mut number : u64 = 0;
        for (index_digit, digit) in self.output.iter().rev().enumerate() {

            if digit.is_none() { return None }
            number = number + (10 as u64).pow(index_digit as u32) * digit.unwrap();
        }

        return Some(number);
    }

    fn solve_mapping_by_incorrect_size(&mut self) {

        let number_segments_map : HashMap<Digit, usize> = [
            (0, 6),
            (1, 2), (2, 5), (3, 5),
            (4, 4), (5, 5), (6, 6),
            (7, 3), (8, 7), (9, 6)].iter().cloned().collect();

        // Remove of number with incorrect number of segments
        for (code, digits) in self.map_codes.iter_mut() {

            let mut digits_to_remove : Vec<usize> = Vec::new();
            let code_length : usize = code.len();

            for (digit_index, digit) in digits.iter().enumerate() {
                if *number_segments_map.get(digit).unwrap() != code_length {
                    digits_to_remove.push(digit_index);
                }
            }

            for digit_to_remove in digits_to_remove.into_iter().rev() {
                digits.remove(digit_to_remove);
            }
        }
    }

    fn solve_mapping_by_commonalities(&mut self) {

        let mut mapping_digits : HashMap<Digit, Code> = HashMap::new();
        for (code, digits) in self.map_codes.iter() {
            if digits.len() == 1 { mapping_digits.insert(digits[0], code.clone()); }
        }

        struct Commonality {
            know:           Digit,
            target:         Digit,
            common_chars:   usize,
        }

        let know_commonalities : Vec<Commonality> = vec!(
            Commonality{know: 1, target: 2, common_chars: 1},
            Commonality{know: 1, target: 3, common_chars: 2},
            Commonality{know: 1, target: 5, common_chars: 1},

            Commonality{know: 4, target: 2, common_chars: 2},
            Commonality{know: 4, target: 3, common_chars: 3},
            Commonality{know: 4, target: 5, common_chars: 3},

            Commonality{know: 1, target: 0, common_chars: 2},
            Commonality{know: 1, target: 6, common_chars: 1},
            Commonality{know: 1, target: 9, common_chars: 2},

            Commonality{know: 4, target: 0, common_chars: 3},
            Commonality{know: 4, target: 6, common_chars: 3},
            Commonality{know: 4, target: 9, common_chars: 4},

        );

        for commonality in know_commonalities {

            let known_code = mapping_digits.get(&commonality.know).unwrap();
            for (code, digits) in self.map_codes.iter_mut() {

                let common_chars = common_characthers(known_code.clone(), code.clone());
                let position_target = digits.iter().position(|&elem| elem == commonality.target);

                if position_target.is_some() && common_chars != commonality.common_chars {
                    digits.remove(position_target.unwrap());
                }
            }

        }
    }

    fn solve_mapping(&mut self) {

        self.solve_mapping_by_incorrect_size();
        self.solve_mapping_by_commonalities();
    }

    fn solve_output(&mut self) {

        self.output = Vec::new();
        for output_code in self.output_codes.iter() {

            let possible_digits = self.map_codes.get(output_code).unwrap();
            if possible_digits.len() == 1 { self.output.push(Some(possible_digits[0])); }
            else { self.output.push(None); }
        }
    }

}

impl Display {

    pub fn new(entries: Vec<EntryProblem>) -> Display {
        Display {
            entry_problems: entries,
        }
    }

    pub fn solve_entries(&mut self) {

        for entry in self.entry_problems.iter_mut() {

            entry.solve_mapping();
            entry.solve_output();
        }
    }

    pub fn number_of_digits(&self, digits_to_count: Vec<Digit>) -> usize {

        let mut count: usize = 0;
        for entry in self.entry_problems.iter() {

            let output = entry.get_output();

            for output_digit_option in output {

                if output_digit_option.is_none() { continue }
                if digits_to_count.contains(&output_digit_option.unwrap()) { count = count + 1; }
            }
        }

        return count;
    }

    pub fn sum_outputs(&self) -> u64 {

        let mut sum : u64 = 0;
        for entry in self.entry_problems.iter() {
            sum = sum + entry.get_output_number().unwrap();
        }

        return sum;
    }
}