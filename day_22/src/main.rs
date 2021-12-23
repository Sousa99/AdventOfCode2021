mod read;
mod lib;

use lib::{CubeState, LimitedMap, UnlimitedMap};

fn main() {

    let lines = read::read_lines("input.txt".to_owned());
    let lines_converted : Vec<(String, Vec<(char, i64, i64)>)>= lines.into_iter()
        .map(|line| {

            let split_white : Vec<&str> = line.split_whitespace().collect();
            let state : String = split_white[0].to_owned();
            let ranges : Vec<(char, i64, i64)> = split_white[1].split(',').into_iter()
                .map(|axis_info| {

                    let mut axis_info_str = axis_info.to_owned();
                    let axis_characther = axis_info_str.remove(0);
                    axis_info_str.remove(0);
                    let ranges : Vec<i64> = axis_info_str.split("..").into_iter()
                        .map(|number| number.parse().unwrap())
                        .collect();

                    return (axis_characther, ranges[0], ranges[1]);
                }).collect();
            return (state, ranges);
        }).collect();

    let limits_first : Vec<(char, i64, i64)> = vec![('x', -50, 50), ('y', -50, 50), ('z', -50, 50)];
    let mut map_limited : LimitedMap = LimitedMap::new(lines_converted.clone(), limits_first);
    let mut map_unlimited : UnlimitedMap = UnlimitedMap::new(lines_converted);

    // Part 1
    while !map_limited.completed_rules() { map_limited.do_iteration() }
    let number_on = map_limited.compute_with_state(CubeState::On);
    println!("\r🧊  The number of turned 'on' cubes, in limited, was '{}' (Part 1)", number_on);
    
    // Part 2
    while !map_unlimited.completed_rules() { map_unlimited.do_iteration() }
    let number_on = map_unlimited.compute_on_state();
    println!("\r🧊  The number of turned 'on' cubes, in unlimited, was '{}' (Part 2)", number_on);
}
