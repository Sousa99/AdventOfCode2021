mod read;
mod lib;

use lib::Map;

fn main() {

    let digits : Vec<Vec<u32>> = read::read_digits("input.txt".to_owned());
    let mut map : Map = Map::new(digits);
    map.find_low_points();

    // Part 1
    let count_1 : u32 = map.find_sum_risk_levels();
    println!("🌋  The risk level is '{}' (Part 1)", count_1);
    
    // Part 2
    let count_2 : u32 = map.find_basins_value();
    println!("🌋  The value for the basins '{}' (Part 2)", count_2);
}