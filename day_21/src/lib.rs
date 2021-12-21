use std::collections::{HashMap};

// ================================================== STRUCTS ==================================================

type Value = i64;

struct DiracDice {
    start_value:    Value,
    end_value:      Value,
    dice_value:     Value
}

struct QuantumDiracDice {
    start_value:    Value,
    end_value:      Value,
}

#[derive(Clone)]
struct Player {
    identification: String,
    position:       Value,
    score:          Value,
}

pub struct Game {
    players:            Vec<Player>,
    dirac_dice:         DiracDice,
    // Aux
    next_player_index:  usize,
    iterations:         usize,
    // Constants
    start_map:          Value,
    end_map:            Value,
    score_value:        Value,
    number_of_rolls:    usize,
}

pub struct QuantumGame {
    player_universes:   HashMap<Vec<(Value, Value)>, usize>,
    universe_scores:    HashMap<usize, usize>,
    dirac_dice:         QuantumDiracDice,
    // Aux
    number_of_players:  usize,
    next_player_index:  usize,
    // Constants
    start_map:          Value,
    end_map:            Value,
    score_value:        Value,
    number_of_rolls:    usize,
}

// ================================================== AUX FUNCTIONS ==================================================



// ================================================== IMPLEMENTATIONS ==================================================

impl DiracDice {

    fn new(start_value: Value, limit_value: Value) -> DiracDice {
        DiracDice {
            start_value: start_value,
            end_value: limit_value,
            dice_value: 1,
        }
    }

    fn roll_dice(&mut self) -> Value {

        let rolled_value = self.dice_value;
        self.dice_value = self.dice_value + 1;
        if self.dice_value > self.end_value { self.dice_value = self.start_value; }

        return rolled_value;
    }
}

impl QuantumDiracDice {

    fn new(start_value: Value, end_value: Value) -> QuantumDiracDice {
        QuantumDiracDice {
            start_value: start_value,
            end_value: end_value,
        }
    }

    fn roll_dice(&mut self) -> Vec<Value> {

        let mut rolled_values : Vec<Value> = Vec::new();
        for value in self.start_value..=self.end_value { rolled_values.push(value) }

        return rolled_values;
    }
}

impl Player {

    fn new(identification: String, position: Value) -> Player {
        Player {
            identification: identification,
            position: position,
            score: 0,
        }
    }

    fn get_position(&self) -> Value { self.position }
    fn get_score(&self) -> Value { self.score }

    fn move_player_to_position(&mut self, position: Value) {

        self.position = position;
        self.score = self.score + self.position;
    }
}

impl Game {

    pub fn new(player_infos: Vec<(String, Value)>, map_limits: (Value, Value), dice_limits: (Value, Value), score_value: Value, number_of_rolls: usize) -> Game {
        Game {
            players: player_infos.into_iter().map(|info| Player::new(info.0, info.1)).collect(),
            dirac_dice: DiracDice::new(dice_limits.0, dice_limits.1),
            // Aux
            next_player_index: 0,
            iterations: 0,
            // Constants
            start_map: map_limits.0,
            end_map: map_limits.1,
            score_value: score_value,
            number_of_rolls: number_of_rolls,
        }
    }

    pub fn get_iteration(&self) -> usize { self.iterations }
    pub fn get_loosing_score(&self) -> Value { self.players.iter().map(|player| player.get_score()).min().unwrap() }

    pub fn some_player_won(&self) -> bool {
        self.players.iter()
            .any(|player| player.get_score() >= self.score_value)
    }

    pub fn next_play(&mut self) {

        if self.some_player_won() { return }

        self.iterations = self.iterations + 3;

        let number_of_players : usize = self.players.len();
        let next_player : &mut Player = self.players.get_mut(self.next_player_index).unwrap();
        self.next_player_index = (self.next_player_index + 1) % number_of_players;

        let mut roll : Value = 0;
        for _ in 0..self.number_of_rolls { roll = roll + self.dirac_dice.roll_dice(); }
        let next_position : Value = (next_player.get_position() + roll - 1) % self.end_map + 1 + (self.start_map - 1);

        next_player.move_player_to_position(next_position);
    }
}

impl QuantumGame {

    pub fn new(player_infos: Vec<(String, Value)>, map_limits: (Value, Value), dice_limits: (Value, Value), score_value: Value, number_of_rolls: usize) -> QuantumGame {
        
        let number_of_players = player_infos.len();
        let universe_scores = player_infos.iter().enumerate().map(|(index, _)| (index, 0)).collect();

        let entry : Vec<(Value, Value)> = player_infos.into_iter().map(|info| (info.1, 0)).collect();
        let mut player_universes : HashMap<Vec<(Value, Value)>, usize> = HashMap::new();
        player_universes.insert(entry, 1);

        QuantumGame {
            player_universes: player_universes,
            universe_scores: universe_scores,
            dirac_dice: QuantumDiracDice::new(dice_limits.0, dice_limits.1),
            // Aux
            next_player_index: 0,
            number_of_players: number_of_players,
            // Constants
            start_map: map_limits.0,
            end_map: map_limits.1,
            score_value: score_value,
            number_of_rolls: number_of_rolls,
        }
    }

    pub fn all_universes_finished(&self) -> bool { self.player_universes.len() == 0 }

    pub fn get_winning_player_universes(&self) -> usize {

        self.universe_scores.iter()
            .map(|(_, &score)| score)
            .max()
            .unwrap()
    }

    pub fn next_play(&mut self) {

        if self.all_universes_finished() { return }
        let number_of_players : usize = self.number_of_players;

        let mut new_universes : HashMap<Vec<(Value, Value)>, usize> = HashMap::new();
        println!("🪐  Number of different state Universes: {}", self.player_universes.len());
        for (players, &count) in self.player_universes.iter() {
            let mut rolls : Vec<Value> = vec![0];
            for _ in 0..self.number_of_rolls {

                let mut new_rolls : Vec<Value> = Vec::new();
                for value in self.dirac_dice.roll_dice() {
                    for already_roll_value in rolls.iter() {
                        new_rolls.push(already_roll_value + value);
                    }
                }

                rolls = new_rolls;
            }

            
            for roll in rolls {

                let mut new_players : Vec<(Value, Value)> = players.clone();
                let next_player : &mut (Value, Value) = new_players.get_mut(self.next_player_index).unwrap();

                let next_position : Value = (next_player.0 + roll - 1) % self.end_map + 1 + (self.start_map - 1);
                next_player.0 = next_position;
                next_player.1 = next_player.1 + next_position;

                if next_player.1 >= self.score_value {
                    let current_score = self.universe_scores.get_mut(&self.next_player_index).unwrap();
                    *current_score = *current_score + count;
                } else if !new_universes.contains_key(&new_players) {
                    new_universes.insert(new_players, count);
                } else {
                    let current_count = new_universes.get_mut(&new_players).unwrap();
                    *current_count = *current_count + count;
                }
            }
            
        }

        self.next_player_index = (self.next_player_index + 1) % number_of_players;
        self.player_universes = new_universes;
    }
}