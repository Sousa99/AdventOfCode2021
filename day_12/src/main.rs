mod read;
mod lib;

use lib::Map;

fn main() {

    let lines : Vec<String> = read::read_lines("input.txt".to_owned());
    let info : Vec<(String, String)> = lines.into_iter()
        .map(|line| {

            let line_split : Vec<&str> = line.split("-").collect();
            return (line_split[0].to_owned(), line_split[1].to_owned())
        }).collect();

    let map : Map = Map::new(info);
    
    // Part 1
    let paths = map.compute_paths(false);
    //for path in paths.iter() { println!("\t{:?}", path); }
    let count_1 = paths.len();
    println!("🏞️  The number of paths found was '{}' (Part 1)", count_1);
    
    // Part 2
    let paths = map.compute_paths(true);
    //for path in paths.iter() { println!("\t{:?}", path); }
    let count_2 = paths.len();
    println!("🏞️  The number of paths found was '{}' (Part 2)", count_2);
}