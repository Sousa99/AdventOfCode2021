mod read;
mod lib;

use lib::Map;

fn main() {

    let energy_levels : Vec<Vec<u32>> = read::read_digits("input.txt".to_owned());
    let mut map = Map::new(energy_levels);
    
    while !map.do_iteration() {}
}