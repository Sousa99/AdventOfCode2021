mod read;
mod lib;

use lib::{Game, QuantumGame};

fn main() {

    let lines = read::read_lines("input.txt".to_owned());
    let player_infos : Vec<(String, i64)> = lines.into_iter()
        .map(|line| {

            let splitted = line.replace(" starting position: ", ":").split(":")
                .into_iter()
                .map(|elem| elem.to_owned())
                .collect::<Vec<String>>();

            let name = splitted[0].clone();
            let position : i64 = splitted[1].parse().unwrap();

            return (name, position);
        }).collect();

    let mut game : Game = Game::new(player_infos.clone(), (1, 10), (1, 100), 1000, 3);
    let mut quantum_game : QuantumGame = QuantumGame::new(player_infos, (1, 10), (1, 3), 21, 3);

    // Part 1
    while !game.some_player_won() { game.next_play() }
    let losing_player_score : i64 = game.get_loosing_score();
    let rolls : i64 = game.get_iteration() as i64;
    println!("🎲  The dice was rolled '{}' times and the loosing player had '{}' points, giving a result of '{}' (Part 1)", rolls, losing_player_score, rolls * losing_player_score);
    
    println!();

    // Part 2
    while !quantum_game.all_universes_finished() { quantum_game.next_play() }
    let winning_player_universes : usize = quantum_game.get_winning_player_universes();
    println!("🎲  The quantum game had the loosing player lose in '{}' universes (Part 2)", winning_player_universes);
}
