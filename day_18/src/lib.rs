use std::fmt;
use std::collections::{HashMap, VecDeque};

use std::io::{Error, ErrorKind};

// ================================================== STRUCTS ==================================================

type Literal = i64;

#[derive(Copy, Clone, PartialEq)]
enum SnailNumberType { LiteralNumber, PairNumber }

#[derive(Copy, Clone, PartialEq)]
enum SideToAdd { Left, Right }

trait SnailFishNumber : fmt::Display {

    fn clone_box(&self) -> Box<dyn SnailFishNumber>;
    fn get_type(&self) -> SnailNumberType;

    fn get_magnitude(&self) -> Literal;

    fn explode(&mut self, level: usize) -> Option<((Literal, Literal), bool)>;
    fn split(&mut self, level: usize) -> bool;

    fn add_value(&mut self, side: SideToAdd, value: Literal); 

    fn do_iteration(&mut self) -> bool;
}

struct LiteralNumber {
    literal_value:  Literal
}

struct PairNumber {
    first_elem:     Box<dyn SnailFishNumber>,
    second_elem:    Box<dyn SnailFishNumber>,
}

pub struct SnailMathProblem {
    elements:       Vec<Box<dyn SnailFishNumber>>,
    sum_element:    Option<Box<dyn SnailFishNumber>>,
}

// ================================================== AUX FUNCTIONS ==================================================



// ================================================== IMPLEMENTATIONS ==================================================

impl LiteralNumber {
    fn new(literal: Literal) -> LiteralNumber {
        LiteralNumber {
            literal_value: literal,
        }
    }
}

impl SnailFishNumber for LiteralNumber {

    fn clone_box(&self) -> Box<dyn SnailFishNumber> {
        Box::new(LiteralNumber::new(self.literal_value))
    }

    fn get_type(&self) -> SnailNumberType { SnailNumberType::LiteralNumber }
    
    fn get_magnitude(&self) -> Literal { self.literal_value }

    fn explode(&mut self, _level: usize) -> Option<((Literal, Literal), bool)> { None }
    fn split(&mut self, _level: usize) -> bool { false }

    fn add_value(&mut self, _side: SideToAdd, value: Literal) { self.literal_value += value }

    fn do_iteration(&mut self) -> bool {

        if self.explode(0).is_some() { return false }
        if self.split(0) { return false }

        return true;
    }
}

impl PairNumber {
    fn new(first_elem: Box<dyn SnailFishNumber>, second_elem: Box<dyn SnailFishNumber>) -> PairNumber {
        PairNumber {
            first_elem: first_elem,
            second_elem: second_elem,
        }
    }
}

impl SnailFishNumber for PairNumber {

    fn clone_box(&self) -> Box<dyn SnailFishNumber> {
        Box::new(PairNumber::new(self.first_elem.clone_box(), self.second_elem.clone_box()))
    }

    fn get_type(&self) -> SnailNumberType { SnailNumberType::PairNumber }

    fn get_magnitude(&self) -> Literal { 3 * self.first_elem.get_magnitude() + 2 * self.second_elem.get_magnitude()  }

    fn explode(&mut self, level: usize) -> Option<((Literal, Literal), bool)> {

        // Explode this one
        if level >= 4 && self.second_elem.get_type() == SnailNumberType::LiteralNumber &&
            self.first_elem.get_type() == SnailNumberType::LiteralNumber {

            let values : (Literal, Literal) = (self.first_elem.get_magnitude(), self.second_elem.get_magnitude());
            return Some((values, true));
        }

        let first_explode_option = self.first_elem.explode(level + 1);
        if first_explode_option.is_some() {

            let first_explode = first_explode_option.unwrap();
            if first_explode.1 { self.first_elem = LiteralNumber::new(0).clone_box(); }

            self.second_elem.add_value(SideToAdd::Left, first_explode.0.1);
            return Some(((first_explode.0.0, 0), false));
        }

        let second_explode_option = self.second_elem.explode(level + 1);
        if second_explode_option.is_some() {

            let second_explode = second_explode_option.unwrap();
            if second_explode.1 { self.second_elem = LiteralNumber::new(0).clone_box(); }

            self.first_elem.add_value(SideToAdd::Right, second_explode.0.0);
            return Some(((0, second_explode.0.1), false));
        }

        return None;
    }

    fn split(&mut self, level: usize) -> bool {

        let magnitude_first_item = self.first_elem.get_magnitude();
        if self.first_elem.get_type() == SnailNumberType::LiteralNumber && magnitude_first_item > 9 {

            let new_first_value = ((magnitude_first_item as f64 / 2.0).floor()) as Literal;
            let new_first_elem = LiteralNumber::new(new_first_value).clone_box();
            let new_second_value = ((magnitude_first_item as f64 / 2.0).ceil()) as Literal;
            let new_second_elem = LiteralNumber::new(new_second_value).clone_box();
            let new_elem = PairNumber::new(new_first_elem, new_second_elem).clone_box();

            self.first_elem = new_elem;
            return true;
        }

        if self.first_elem.split(level + 1) { return true }

        let magnitude_second_item = self.second_elem.get_magnitude();
        if self.second_elem.get_type() == SnailNumberType::LiteralNumber && magnitude_second_item > 9 {

            let new_first_value = ((magnitude_second_item as f64 / 2.0).floor()) as Literal;
            let new_first_elem = LiteralNumber::new(new_first_value).clone_box();
            let new_second_value = ((magnitude_second_item as f64 / 2.0).ceil()) as Literal;
            let new_second_elem = LiteralNumber::new(new_second_value).clone_box();
            let new_elem = PairNumber::new(new_first_elem, new_second_elem).clone_box();

            self.second_elem = new_elem;
            return true;
        }

        if self.second_elem.split(level + 1) { return true }
        return false;
    }

    fn add_value(&mut self, side: SideToAdd, value: Literal) {
        match side {
            SideToAdd::Left => self.first_elem.add_value(SideToAdd::Left, value),
            SideToAdd::Right => self.second_elem.add_value(SideToAdd::Right, value),
        }
    }

    fn do_iteration(&mut self) -> bool {

        if self.explode(0).is_some() { return false }
        if self.split(0) { return false }

        return true;
    }
}

impl SnailMathProblem {

    fn get_elements_from_input(input_lines: Vec<String>) -> Vec<Box<dyn SnailFishNumber>> {

        let elements : Vec<Box<dyn SnailFishNumber>> = input_lines.into_iter()
            .map(|input_line| {

                let mut current_heap_by_level : HashMap<usize, VecDeque<Box<dyn SnailFishNumber>>> = HashMap::new();
                let mut level_count : usize = 0;
        
                let mut current_index : usize = 0;
                while current_index < input_line.len() {

                    // Add to the heap level
                    if !current_heap_by_level.contains_key(&level_count) {
                        current_heap_by_level.insert(level_count, VecDeque::new());
                    }
                    
                    let current_char : char = input_line.chars().nth(current_index).unwrap();
                    // Opened new pair
                    if current_char == '[' {
                        level_count = level_count + 1;
                        current_index = current_index + 1;
                    }
                    // Closed pair
                    else if current_char == ']' {

                        let current_heap = current_heap_by_level.get_mut(&level_count).unwrap();
                        let first_elem = current_heap.pop_front().unwrap();
                        let second_elem = current_heap.pop_front().unwrap();
                        
                        level_count = level_count - 1;
                        current_index = current_index + 1;
                        let new_elem = PairNumber::new(first_elem, second_elem).clone_box();
                        current_heap_by_level.get_mut(&level_count).unwrap().push_back(new_elem);
                    }
                    else if current_char == ',' { current_index = current_index + 1; }
                    // Saw other things (must be number for literal, only possibility)
                    else {

                        let index_of_other_symbol = vec!('[', ',', ']').into_iter()
                            .map(|symbol| input_line.as_str()[current_index..]
                                .find(symbol).map(|i| i + current_index))
                            .filter(|option| option.is_some())
                            .map(|option| option.unwrap())
                            .min().unwrap();
                        
                        let sub_string_with_value = input_line.as_str()[current_index..index_of_other_symbol].to_owned();

                        let value = sub_string_with_value.parse()
                            .map_err(|e| Error::new(ErrorKind::InvalidData, e))
                            .unwrap();

                        let literal_number = LiteralNumber::new(value).clone_box();
                        current_heap_by_level.get_mut(&level_count).unwrap().push_back(literal_number);
                        current_index = index_of_other_symbol;
                    }
                }

                let snail_number : Box<dyn SnailFishNumber> = current_heap_by_level.get_mut(&0).unwrap()
                    .pop_front().unwrap();

                return snail_number;
            }).collect();

        return elements
    }

    fn join_elements(first_elem: Box<dyn SnailFishNumber>, second_elem: Box<dyn SnailFishNumber>) -> Box<dyn SnailFishNumber> {

        let current_element = PairNumber::new(first_elem, second_elem).clone_box();
        return current_element;
    }

    pub fn new(input_lines: Vec<String>) -> SnailMathProblem {

        let elements = SnailMathProblem::get_elements_from_input(input_lines);

        SnailMathProblem {
            elements: elements,
            sum_element: None,
        }
    }

    pub fn sum_elements(&mut self) {

        let mut current_value = self.elements[0].clone_box();
        for i in 1..self.elements.len() {
            
            let other_elem = self.elements[i].clone_box();
            current_value = SnailMathProblem::join_elements(current_value, other_elem);
            
            while !current_value.do_iteration() {}
        }
        
        self.sum_element = Some(current_value);
    }

    pub fn get_magnitude(&self) -> Literal { self.sum_element.as_ref().unwrap().get_magnitude() }
    
    pub fn compute_max_sum(&self) -> Option<((usize, usize), Literal)> {

        let mut max_info : Option<((usize, usize), Literal)> = None;

        for first_index in 0..self.elements.len() {
            for second_index in 0..self.elements.len() {

                if first_index == second_index { continue }
                
                let first_element = self.elements[first_index].clone_box();
                let second_element = self.elements[second_index].clone_box();
                let mut sum_element = SnailMathProblem::join_elements(first_element, second_element);

                while !sum_element.do_iteration() {}
                let magnitude = sum_element.get_magnitude();

                if max_info.is_none() || max_info.unwrap().1 < magnitude {
                    max_info = Some(((first_index, second_index), magnitude));
                }
            }
        }

        return max_info;
    }
}

impl fmt::Display for LiteralNumber {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        write!(f, "{}", self.literal_value)?;
        return Ok(());
    }
}

impl fmt::Display for PairNumber {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        write!(f, "[{}, {}]", self.first_elem, self.second_elem)?;
        return Ok(());
    }
}