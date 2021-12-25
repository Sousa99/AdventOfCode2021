mod read;
mod lib;

use lib::{Map};

fn main() {

    
    let lines = read::read_chars("input.txt".to_owned());
    let mut map = Map::new(lines);

    while map.iterate() {};
    //println!("{}", map);

    // Part 1
    let number_of_iterations = map.get_final_iteration();
    println!("\r🥒  After '{}' iterations no more movements were detected (Part 1)", number_of_iterations);
    
    // Part 2
}