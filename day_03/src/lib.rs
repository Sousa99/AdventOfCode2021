use std::collections::HashMap;

// ================================================== STRUCTS ==================================================

struct InfoPosition {
    zeros:      usize,
    ones:       usize,
}

#[derive(Clone)]
pub struct Rate {
    pub binary:     String,
    pub decimal:    i64
}

pub struct Diagnostic {
    lines:          Vec<String>,
    gamma_rate:     Option<Rate>,
    epsilon_rate:   Option<Rate>,
    oxygen_rate:    Option<Rate>,
    dyoxide_rate:   Option<Rate>
}

// ================================================== AUX FUNCTIONS ==================================================

fn calculate_position_map(lines: &Vec<String>) -> HashMap<usize, InfoPosition> {
    
    let mut position_map : HashMap<usize, InfoPosition> = HashMap::new();
    for line in lines.iter() {

        for (index_position, characther) in line.chars().enumerate() {

            let current_info_option : Option<&mut InfoPosition> = position_map.get_mut(&index_position);
            match current_info_option {
                Some(current_info) if characther == '0' => current_info.zeros += 1,
                Some(current_info) if characther == '1' => current_info.ones += 1,
                None if characther == '0' => { position_map.insert(index_position, InfoPosition{ zeros: 1, ones: 0 }); },
                None if characther == '1' => { position_map.insert(index_position, InfoPosition{ zeros: 0, ones: 1 }); },
                Some(_) => panic!("Characther not expected!"), 
                None => panic!("Characther not expected!"),
            }
        }
    }

    return position_map;
}

// ================================================== IMPLEMENTATIONS ==================================================

impl Diagnostic {

    pub fn new(lines: Vec<String>) -> Diagnostic {
        Diagnostic {
            lines:          lines,
            // Combustion
            gamma_rate:     None,
            epsilon_rate:   None,
            // Life support
            oxygen_rate:    None,
            dyoxide_rate:   None,
        }
    }

    pub fn calculate_rates(&mut self) {

        // Combustion Rates
        self.calculate_combustion_rates();
        // Support Rates
        self.calculate_support_rates();
    }

    fn calculate_combustion_rates(&mut self) {

        let position_map = calculate_position_map(&self.lines);

        // Calculate String Rates
        let mut gamma_string : String = "".to_owned();
        let mut epsilon_string : String = "".to_owned();
        for position in 0..position_map.len() {

            let info = position_map.get(&position).unwrap();
            if info.zeros > info.ones {
                gamma_string.push('0');
                epsilon_string.push('1');
            } else {
                gamma_string.push('1');
                epsilon_string.push('0');
            }
        }

        // Calculate Rates
        self.gamma_rate = Some(Rate{binary: gamma_string.clone(), decimal: i64::from_str_radix(&gamma_string, 2).unwrap()});
        self.epsilon_rate = Some(Rate{binary: epsilon_string.clone(), decimal: i64::from_str_radix(&epsilon_string, 2).unwrap()});
    }

    fn calculate_support_rates(&mut self) {

        let mut valid_lines_oxygen = self.lines.clone();
        let mut valid_lines_dyoxide = self.lines.clone();

        let mut count_position : usize = 0;
        while valid_lines_oxygen.len() != 1 {

            let position_map = calculate_position_map(&valid_lines_oxygen);
            let info_position = position_map.get(&count_position).unwrap();
            let mut indexes_to_remove: Vec<usize> = Vec::new();
            for (index_line, line) in valid_lines_oxygen.iter().enumerate() {
                
                if (info_position.ones >= info_position.zeros && line.chars().nth(count_position).unwrap() == '0') ||
                (info_position.ones < info_position.zeros && line.chars().nth(count_position).unwrap() == '1') {
                    indexes_to_remove.push(index_line);
                }
            }
            
            for index in indexes_to_remove.into_iter().rev() { valid_lines_oxygen.remove(index); }
            count_position += 1;
        }
        
        let mut count_position : usize = 0;
        while valid_lines_dyoxide.len() != 1 {
            
            let position_map = calculate_position_map(&valid_lines_dyoxide);
            let info_position = position_map.get(&count_position).unwrap();
            let mut indexes_to_remove: Vec<usize> = Vec::new();
            for (index_line, line) in valid_lines_dyoxide.iter().enumerate() {

                if (info_position.ones < info_position.zeros && line.chars().nth(count_position).unwrap() == '0') ||
                    (info_position.ones >= info_position.zeros && line.chars().nth(count_position).unwrap() == '1') {
                        indexes_to_remove.push(index_line);
                }
            }

            for index in indexes_to_remove.into_iter().rev() { valid_lines_dyoxide.remove(index); }
            count_position += 1;
        }

        let oxygen_rate_string = valid_lines_oxygen.first().unwrap().clone();
        let dyoxide_rate_string = valid_lines_dyoxide.first().unwrap().clone();
        self.oxygen_rate = Some(Rate{binary: oxygen_rate_string.clone(), decimal: i64::from_str_radix(&oxygen_rate_string, 2).unwrap()});
        self.dyoxide_rate = Some(Rate{binary: dyoxide_rate_string.clone(), decimal: i64::from_str_radix(&dyoxide_rate_string, 2).unwrap()});
    }

    pub fn get_combustion_rates(&self) -> Option<(Rate, Rate)> {

        return match (self.gamma_rate.clone(), self.epsilon_rate.clone()) {
            (Some(gamma_rate), Some(epsilon_rate)) => Some((gamma_rate, epsilon_rate)),
            (_, _) => None,
        }
    }

    pub fn get_support_rates(&self) -> Option<(Rate, Rate)> {

        return match (self.oxygen_rate.clone(), self.dyoxide_rate.clone()) {
            (Some(oxygen_rate), Some(dyoxide_rate)) => Some((oxygen_rate, dyoxide_rate)),
            (_, _) => None,
        }
    }
}