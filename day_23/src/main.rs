mod read;
mod lib;

use lib::{Map};

fn main() {

    let lines = read::read_lines("input.txt".to_owned());
    let mut map = Map::new(lines);

    // Part 1
    map.reach_final_node();
    let minimum_energy = map.get_energy_of_final_node();
    println!("\r🦐  The minimum energy level needed is '{}' (Part 1)", minimum_energy);
    
    // Part 2

}
