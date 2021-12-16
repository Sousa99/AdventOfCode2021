mod read;
mod lib;

use lib::{SystemBITS};

fn main() {

    let lines = read::read_lines("input.txt".to_owned());
    let mut system = SystemBITS::new(lines[0].clone());
    
    //println!("{}", system);

    // Part 1
    let sum_versions = system.get_sum_of_versions_of_packets();
    println!("💻  The sum of the version IDs of the various packets is '{}' (Part 1)", sum_versions[0]);
    
    // Part 2
    let final_values = system.compute_values();
    println!("💻  The value of the packet is '{}' (Part 2)", final_values[0]);

    //println!("{}", system);
}
