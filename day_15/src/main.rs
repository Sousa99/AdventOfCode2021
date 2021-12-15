mod read;
mod lib;

use lib::{Map};

fn main() {

    let values = read::read_digits("input.txt".to_owned());
    let mut map = Map::new(values.clone(), 1); 
    let mut tilled_map = Map::new(values, 5); 
    
    // Part 1
    let risk_level = map.get_shortest_path().unwrap();
    println!("🗺️  The obtained risk level of the shortest path is '{}' (Part 1)", risk_level);
    
    // Part 2
    let risk_level = tilled_map.get_shortest_path().unwrap();
    println!("🗺️  The obtained risk level of the shortest path in the tilled version is '{}' (Part 2)", risk_level);
}
