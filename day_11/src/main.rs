mod read;
mod lib;

use lib::Map;

fn main() {

    let energy_levels : Vec<Vec<u32>> = read::read_digits("input.txt".to_owned());
    let mut map = Map::new(energy_levels);
    
    // Part 1
    for _ in 0..100 {
        map.do_iteration();
        println!("{}", map);
    }

    let count_1 : u64 = map.get_number_of_flashes();
    
    // Part 2
    while !map.do_iteration() {
        println!("{}", map);
    }
    println!("{}", map);
    let count_2 : u64 = map.get_iteration();
    
    
    // Print Results
    println!("🌩️  The number of flashes detected was '{}' (Part 1)", count_1);
    println!("🌩️  Number of iterations untill all flash '{}' (Part 2)", count_2);
}