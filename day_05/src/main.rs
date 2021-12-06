use std::io::{BufRead, BufReader, Error, ErrorKind};

mod read;
mod lib;

use lib::{Line, Map};

fn main() {

    let input_lines = read::read_lines("input.txt".to_owned());
    // Sepparate Input
    let mut map_lines : Vec<Line> = Vec::new();
    for line in input_lines {

        let pos_split : Vec<&str> = line.split(" -> ").collect();

        let pos1_info : &str = pos_split[0];
        let pos2_info : &str = pos_split[1];

        let pos1_numbers : Vec<i64> = pos1_info.split(',')
            .map(|elem| elem.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e)))
            .collect::<Result<_, _>>().unwrap();
        let pos2_numbers : Vec<i64> = pos2_info.split(',')
            .map(|elem| elem.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e)))
            .collect::<Result<_, _>>().unwrap();

        map_lines.push(Line::new(pos1_numbers[0], pos1_numbers[1], pos2_numbers[0], pos2_numbers[1]))
    }

    // Create Map
    let mut map = Map::new(map_lines.clone(), false);
    let mut map_diagonal = Map::new(map_lines.clone(), true);
    map.mark_lines();
    map_diagonal.mark_lines();

    // Part 1
    let count = map.count_positions_with_me(2);
    //println!("{}", map);
    println!("💨 Overlapping twice with diagonal: {} (Part 1)", count);
    
    // Part 2
    let count = map_diagonal.count_positions_with_me(2);
    //println!("{}", map_diagonal);
    println!("💨 Overlapping twice without diagonal: {} (Part 2)", count);
}
