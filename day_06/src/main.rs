use std::io::{BufRead, BufReader, Error, ErrorKind};

mod read;
mod lib;

use lib::{SeaFloor, ExponentialSeaFloor};

fn main() {

    let input_lines = read::read_lines("input.txt".to_owned());
    // Sepparate Input
    let mut lifes : Vec<u64> = input_lines[0].split(',')
        .map(|life_str| life_str.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e)))
        .collect::<Result<_, _>>().unwrap();

    // Create Sea Floor
    let mut sea_floor = SeaFloor::new(lifes.clone());
    let mut exponential_sea_floor = ExponentialSeaFloor::new(lifes.clone());

    // Part 1
    while sea_floor.get_days_done() != 80 {
        //println!("{}", sea_floor);
        sea_floor.do_iteration();
    }
    //println!("{}", sea_floor);
    println!("🐠 The sea floor has '{}' after '{}' days (Part 1)", sea_floor.number_of_fishes(), sea_floor.get_days_done());
    
    // Part 2
    while exponential_sea_floor.get_days_done() != 80 { exponential_sea_floor.do_iteration() }
    println!("🐠 The sea floor has '{}' after '{}' days (Part 2)", exponential_sea_floor.number_of_fishes(), exponential_sea_floor.get_days_done());
    while exponential_sea_floor.get_days_done() != 256 { exponential_sea_floor.do_iteration() }
    println!("🐠 The sea floor has '{}' after '{}' days (Part 2)", exponential_sea_floor.number_of_fishes(), exponential_sea_floor.get_days_done());
}
