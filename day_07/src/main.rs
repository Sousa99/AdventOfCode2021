mod read;
mod lib;

use lib::{CrabArmy, CrabEngineering};

fn main() {

    let lines = read::read_list_int_lines("input.txt".to_owned());
    let positions = &lines[0];
    let constant_army : CrabArmy = CrabArmy::new(CrabEngineering::Constant, positions);
    let increm_army : CrabArmy = CrabArmy::new(CrabEngineering::Incremental, positions);

    // Part 1
    let constant_info = constant_army.minimum_align_position().unwrap();
    println!("🦀 The crabs will align at position '{}' using '{}' fuel with 'constant' engineering (Part 1)", constant_info.0, constant_info.1);
    
    // Part 2
    let increm_army = increm_army.minimum_align_position().unwrap();
    println!("🦀 The crabs will align at position '{}' using '{}' fuel with 'incremental' engineering (Part 2)", increm_army.0, increm_army.1);
}