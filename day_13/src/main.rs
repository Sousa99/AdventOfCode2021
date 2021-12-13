use std::io::{Error, ErrorKind};

mod read;
mod lib;

use lib::Map;

fn main() {

    let lines = read::read_lines("input.txt".to_owned());
    let index_of = lines.iter().position(|line| line == "").unwrap();

    let input_values: Vec<Vec<i64>> = lines[..index_of].iter()
        .map(|line| line.split(',')
            .map(|line| line.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e)))
            .collect::<Result<_, _>>().unwrap())
        .collect::<Vec<Vec<i64>>>();
    let pairs_input_values : Vec<(u64, u64)>= input_values.into_iter()
        .map(|row| (row[0] as u64, row[1] as u64))
        .collect();

    let fold_info : Vec<(String, u64)> = lines[(index_of + 1)..].iter()
        .map(|line| line.replace("fold along ", "")
            .split("=")
            .map(|value| value.to_owned())
            .collect::<Vec<String>>())
        .map(|line| (line[0].clone(), line[1].parse().unwrap()))
        .collect();

    let mut map : Map = Map::new(pairs_input_values, fold_info);
    //println!("{}", map);
    
    
    // Part 1
    map.make_next_fold();
    //println!("{}", map);
    let count_1 = map.count_set();
    println!("🧻  After first fold '{}' dots were found! (Part 1)", count_1);
    
    // Part 2
    while map.number_of_folds() != 0 { map.make_next_fold(); }
    println!("🧻  After folding the paper accordingly... (Part 2)");
    println!("{}", map);
}
