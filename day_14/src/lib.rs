use std::fmt;
use std::collections::HashMap;

// ================================================== STRUCTS ==================================================

type Element = char;
type Polymer = Vec<Element>;


pub struct Polymerization {
    rules:              HashMap<(Element, Element), Element>,
    // Incremental change
    iteration:          usize,
    current_polymer:    Polymer
}

pub struct ExpPolymerization {
    template:           Polymer,
    rules:              HashMap<(Element, Element), Element>,
    // Incremental change
    iteration:          usize,
    current_count:      HashMap<(Element, Element), usize>,
}

// ================================================== AUX FUNCTIONS ==================================================

// ================================================== IMPLEMENTATIONS ==================================================

impl Polymerization {

    pub fn new(template: Polymer, rules: Vec<(Element, Element, Element)>) -> Polymerization {

        let rules_converted : HashMap<(Element, Element), Element> = rules.into_iter()
            .map(|rule| ((rule.0, rule.1), rule.2))
            .collect();

        Polymerization {
            rules: rules_converted,
            // Incremental change
            iteration: 0,
            current_polymer: template,
        }
    }

    pub fn get_current_iteration(&self) -> usize { self.iteration }

    pub fn do_iteration(&mut self) {

        let mut next_polymer : Vec<Element> = Vec::new();
        for index in 0..(self.current_polymer.len() - 1) {

            let prev_char = self.current_polymer[index];
            let next_char = self.current_polymer[index + 1];

            next_polymer.push(prev_char);

            let generate_option = self.rules.get(&(prev_char, next_char));
            match generate_option {
                None => (),
                Some(&generate) => next_polymer.push(generate),
            }
        }
        next_polymer.push(self.current_polymer[self.current_polymer.len() - 1]);

        self.current_polymer = next_polymer;
        self.iteration = self.iteration + 1;
    }

    pub fn count_element(&self) -> Vec<(Element, usize)> {

        let mut count : HashMap<Element, usize> = HashMap::new();
        for &element in self.current_polymer.iter() {

            if count.contains_key(&element) {
                let current_value = count.get_mut(&element).unwrap();
                *current_value = *current_value + 1; 
            } else { count.insert(element, 0); }
        }

        let mut count_vec : Vec<(Element, usize)> = count.into_iter()
            .map(|(element, count)| (element, count))
            .collect();
        count_vec.sort_by_key(|element| element.1);

        return count_vec;
    }
}

impl ExpPolymerization {

    pub fn new(template: Polymer, rules: Vec<(Element, Element, Element)>) -> ExpPolymerization {

        let rules_converted : HashMap<(Element, Element), Element> = rules.iter()
            .map(|rule| ((rule.0, rule.1), rule.2))
            .collect();

        let mut current_count_convert : HashMap<(Element, Element), usize> = rules.iter()
            .map(|rule| ((rule.0, rule.1), 0))
            .collect();

        for index in 0..(template.len() - 1) {
            let sequence = (template[index], template[index + 1]);
            let current_count_seq = current_count_convert.get_mut(&sequence).unwrap();
            *current_count_seq = *current_count_seq + 1;
        }

        ExpPolymerization {
            template: template,
            rules: rules_converted,
            // Incremental change
            iteration: 0,
            current_count: current_count_convert
        }
    }

    pub fn get_current_iteration(&self) -> usize { self.iteration }

    fn generate_empty_count(&self) -> HashMap<(Element, Element), usize> {

        let count : HashMap<(Element, Element), usize> = self.rules.iter()
            .map(|(&(prev_elem, next_elem), _)| ((prev_elem, next_elem), 0))
            .collect();
        
        return count;
    }

    pub fn do_iteration(&mut self) {

        let mut new_count = self.generate_empty_count();
        for (&(prev_elem, next_elem), &count) in self.current_count.iter() {

            let &generated_elem = self.rules.get(&(prev_elem, next_elem)).unwrap();
            
            let new_prev_count = new_count.get_mut(&(prev_elem, generated_elem)).unwrap();
            *new_prev_count = *new_prev_count + count;
            let new_next_count = new_count.get_mut(&(generated_elem, next_elem)).unwrap();
            *new_next_count = *new_next_count + count;
        }

        self.current_count = new_count;
        self.iteration = self.iteration + 1;
    }

    pub fn count_element(&self) -> Vec<(Element, usize)> {

        let mut count : HashMap<Element, usize> = HashMap::new();
        for (&(first_elem, _), &count_seq) in self.current_count.iter() {
            
            if count.contains_key(&first_elem) {
                let current_count = count.get_mut(&first_elem).unwrap();
                *current_count = *current_count + count_seq;

            } else { count.insert(first_elem, count_seq); }
        }

        let last_elem = self.template.last().unwrap();
        if count.contains_key(last_elem) {
            let current_count = count.get_mut(last_elem).unwrap();
            *current_count = *current_count + 1;

        } else { count.insert(*last_elem, 1); }

        let mut count_vec : Vec<(Element, usize)> = count.into_iter()
            .map(|(element, count)| (element, count))
            .collect();
        count_vec.sort_by_key(|element| element.1);

        return count_vec;
    }
}

impl fmt::Display for Polymerization {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        let mut line : String = format!("Polymer in iteration {}: ", self.iteration);
        line = format!("{}{}", line, self.current_polymer.iter().collect::<String>());

        return write!(f, "{}", line);
    }
}