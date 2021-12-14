mod read;
mod lib;

use lib::{Polymerization, ExpPolymerization};

fn main() {

    let lines = read::read_lines("input.txt".to_owned());
    let template : Vec<char> = lines[0].chars().collect();
    let mut rules : Vec<(char, char, char)> = Vec::new();
    for line in lines[2..].into_iter() {

        let line_split : Vec<&str> = line.split(" -> ").collect();
        let origin : Vec<char> = line_split[0].chars().collect();
        let result : Vec<char> = line_split[1].chars().collect();

        rules.push((origin[0], origin[1], result[0]));
    }

    let mut polymerization = Polymerization::new(template.clone(), rules.clone());
    let mut exp_polymerization = ExpPolymerization::new(template, rules);
    
    // Part 1

    //println!("{}", polymerization);
    while polymerization.get_current_iteration() != 10 {
        polymerization.do_iteration();
        //println!("{}", polymerization);
    }

    let counts = polymerization.count_element();
    let value = counts.last().unwrap().1 - counts.first().unwrap().1;
    println!("🧪  The obtained value for the polymer iterated '10' times was '{}' (Part 1)", value);
    
    // Part 2
    while exp_polymerization.get_current_iteration() != 40 {
        exp_polymerization.do_iteration();
    }

    let counts = exp_polymerization.count_element();
    let value = counts.last().unwrap().1 - counts.first().unwrap().1;
    println!("🧪  The obtained value for the polymer iterated '40' times was '{}' (Part 2)", value);
}
