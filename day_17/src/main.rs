mod read;
mod lib;

use lib::{Map};

use std::io::{Error, ErrorKind};

fn main() {

    let mut line : String = read::read_lines("input.txt".to_owned())[0].clone();
    line = line.replace("target area: ", "");

    let splitted_values : Vec<(i64, i64)> = line.split(", ").into_iter()
        .map(|coordinate_info| {

            let split_coordinate_info : Vec<&str> = coordinate_info.split("=").into_iter().collect();
            let split_values : Vec<i64> = split_coordinate_info[1].split("..").into_iter()
                .map(|value_str| value_str.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e)))
                .collect::<Result<_, _>>().unwrap();

            let value_min = split_values[0];
            let value_max = split_values[1];

            return (value_min, value_max);

        }).collect();

    let map = Map::new(splitted_values[0].0, splitted_values[1].0, splitted_values[0].1, splitted_values[1].1);
    
    //println!("{}", system);

    // Part 1
    let highest_y_info = map.compute_highest_y_velocity();
    let initial_velocity = highest_y_info.0;
    let highest_y_value = highest_y_info.1;
    println!("🚀  The peak achieved was '{}' for a velocity of '{{ x: {}, y: {} }}' (Part 1)", highest_y_value, initial_velocity.x, initial_velocity.y);
    
    // Part 2
    let number_of_shots = map.compute_number_of_shots();
    println!("🚀  There are '{}' alternative shots possible (Part 2)", number_of_shots);
}
