mod read;
mod lib;

use lib::{Map};

fn main() {

    
    // Part 1
    let lines_1 = read::read_lines("input_1.txt".to_owned());
    let mut map_1 = Map::new(lines_1);

    map_1.reach_final_node();
    let minimum_energy = map_1.get_energy_of_final_node();
    println!("\r🦐  The minimum energy level needed for level '1' is '{}' (Part 1)", minimum_energy);
    
    // Part 2
    let lines_2 = read::read_lines("input_2.txt".to_owned());
    let mut map_2 = Map::new(lines_2);
    
    map_2.reach_final_node();
    let minimum_energy = map_2.get_energy_of_final_node();
    println!("\r🦐  The minimum energy level needed for level '2' is '{}' (Part 2)", minimum_energy);
}
