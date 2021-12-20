mod read;
mod lib;

use std::io::{Error, ErrorKind};

use lib::{Map};

fn main() {

    let lines = read::read_lines("input.txt".to_owned());
    let mut scanner_lines : Vec<(String, Vec<Vec<i64>>)> = Vec::new();

    let mut current_scanner_option : Option<(String, Vec<Vec<i64>>)> = None;
    for line in lines.into_iter() {
        // Scanner ID
        if line.contains("---") {

            let mut identification = line.replace("--- ", "");
            identification = identification.replace(" ---", "");

            current_scanner_option = Some((identification, Vec::new()));
        }
        // Empty Line
        else if line == "" {
            if current_scanner_option.is_some() {
                scanner_lines.push(current_scanner_option.clone().unwrap())
            }
        }
        // Scanner Positions
        else {
            let data: Vec<i64> = line.split(",")
                .map(|line| line.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e)))
                .collect::<Result<_, _>>().unwrap();
            let mut current_scanner = current_scanner_option.unwrap();
            current_scanner.1.push(data);
            current_scanner_option = Some(current_scanner);
        }
    }
    if current_scanner_option.is_some() { scanner_lines.push(current_scanner_option.unwrap()) }


    let mut map = Map::new(scanner_lines);
    while map.get_number_of_scanners() != map.number_of_scanner_processed() { map.add_next_scanner_info() }
    println!();

    // Part 1
    let number_beacons = map.compute_number_of_beacons();
    println!("🌐  The total number of beacons was '{}' (Part 1)", number_beacons);
    
    // Part 2
    let max_info = map.largest_distance_scanners().unwrap();
    println!("🌐  The largest distance between scanners was '{}' between '{}' and '{}' (Part 2)", max_info.2, max_info.0, max_info.1);
}
