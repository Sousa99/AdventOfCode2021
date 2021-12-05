use std::io::{BufRead, BufReader, Error, ErrorKind};

mod read;
mod lib;

use lib::Game;

fn main() {

    let mut input_lines = read::read_lines("input.txt".to_owned());

    // Sepparate Input
    let called_numbers : Vec<i64> = input_lines.remove(0).split(',')
        .map(|line| line.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e)))
        .collect::<Result<_, _>>().unwrap();
    input_lines.remove(0);

    let mut cards_numbers : Vec<Vec<Vec<i64>>> = Vec::new();
    let mut card_numbers : Vec<Vec<i64>> = Vec::new();
    for line in input_lines {
        
        if line == "" {

            cards_numbers.push(card_numbers);
            card_numbers = Vec::new();

        } else {

            card_numbers.push(line.split_whitespace()
                .map(|line| line.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e)))
                .collect::<Result<_, _>>().unwrap())

        }
    }
    cards_numbers.push(card_numbers);

    let mut game : Game = Game::new(called_numbers, cards_numbers);

    // Part 1
    let score = game.do_iterations_until_one_won(false);
    println!("🪙  Score achieved by 'winning' card: {} (Part 1)", score.unwrap());
    
    // Part 2
    let score = game.do_iterations_until_all_but_one_won(false);
    println!("🪙  Score achieved by 'loosing' card: {} (Part 2)", score.unwrap());
}
