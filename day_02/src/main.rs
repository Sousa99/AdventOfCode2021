mod read;
mod lib;

use lib::Submarine;
use lib::AimedSubmarine;

fn main() {

    let data = read::read_lines("input.txt".to_owned());
    let mut submarine : Submarine = Submarine::new();
    let mut aimed_submarine : AimedSubmarine = AimedSubmarine::new();

    // Part 1
    submarine.update_chart(data.clone());
    let last_position = submarine.get_last_position().unwrap();
    println!("Last position for submarine: (horizontal: {}, depth: {})", last_position.horizontal, last_position.depth);
    println!("Result (Part 1): {}", last_position.horizontal * last_position.depth);

    println!();

    // Part 1
    aimed_submarine.update_chart(data);
    let last_position = aimed_submarine.get_last_position().unwrap();
    println!("Last position for aimed submarine: (horizontal: {}, depth: {})", last_position.horizontal, last_position.depth);
    println!("Result (Part 2): {}", last_position.horizontal * last_position.depth);
}
