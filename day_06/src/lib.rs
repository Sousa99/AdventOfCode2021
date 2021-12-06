
use std::fmt;
use std::collections::HashMap;

// ================================================== STRUCTS ==================================================

type LifePoints = u64;
type Day = u64;

const LIFEPOINTS_AFTER_SPAWN : u64 = 8;
const LIFEPOINTS_AFTER_RESET : u64 = 6;

enum FishIteration { Nothing, Spawn }
struct LanternFish {
    life:       LifePoints,
}

pub struct SeaFloor {
    days_done:  Day,
    fishes:     Vec<LanternFish>,
}

pub struct ExponentialSeaFloor {
    days_done:  Day,
    fishes:     HashMap<usize, usize>,
    incubating: HashMap<usize, usize>,
}

// ================================================== AUX FUNCTIONS ==================================================

// ================================================== IMPLEMENTATIONS ==================================================

impl LanternFish {

    fn new(lifepoints: LifePoints) -> LanternFish {
        LanternFish {
            life: lifepoints,
        }
    }

    fn new_default() -> LanternFish {
        LanternFish {
            life: LIFEPOINTS_AFTER_SPAWN
        }
    }

    fn get_lifepoints(&self) -> LifePoints {
        return self.life;
    }

    fn do_step(&mut self) -> FishIteration {

        if self.life == 0 {
            self.life = LIFEPOINTS_AFTER_RESET;
            return FishIteration::Spawn;
        } else {
            self.life = self.life - 1;
            return FishIteration::Nothing;
        }
    }
}

impl SeaFloor {

    pub fn new(fish_lives: Vec<LifePoints>) -> SeaFloor {

        let fishes : Vec<LanternFish> = fish_lives.iter()
            .map(|&life| LanternFish::new(life))
            .collect();

        SeaFloor {
            days_done: 0,
            fishes: fishes,
        }
    }

    pub fn get_days_done(&self) -> Day { return self.days_done; }
    pub fn number_of_fishes(&self) -> usize { return self.fishes.len(); }

    pub fn do_iteration(&mut self) {

        let mut count_new_fishes : usize = 0;
        for fish in self.fishes.iter_mut() {

            let return_type = fish.do_step();
            match return_type {
                FishIteration::Spawn => count_new_fishes = count_new_fishes + 1,
                FishIteration::Nothing => {}
            }
        }

        for _ in 0..count_new_fishes { self.fishes.push(LanternFish::new_default()) }

        self.days_done = self.days_done + 1;
    }
}

impl ExponentialSeaFloor {

    pub fn new(fish_lives: Vec<LifePoints>) -> ExponentialSeaFloor {

        let mut fishes : HashMap<usize, usize> = HashMap::new();
        for life_value in 0..(LIFEPOINTS_AFTER_RESET + 1) { fishes.insert(life_value as usize, 0); }
        for fish_life in fish_lives { fishes.insert(fish_life as usize, fishes.get(&(fish_life as usize)).unwrap() + 1); }
        let mut fishes_incubating : HashMap<usize, usize> = HashMap::new();
        for life_value in 1..(LIFEPOINTS_AFTER_SPAWN - LIFEPOINTS_AFTER_RESET + 1) { fishes_incubating.insert(life_value as usize, 0); }

        ExponentialSeaFloor {
            days_done: 0,
            fishes: fishes,
            incubating: fishes_incubating,
        }
    }

    pub fn get_days_done(&self) -> Day { return self.days_done; }
    pub fn number_of_fishes(&self) -> usize {

        let mut count = 0;
        for (_, value) in self.fishes.iter() { count = count + value }
        for (_, value) in self.incubating.iter() { count = count + value }
        return count;
    }

    pub fn do_iteration(&mut self) {

        let turn = self.days_done % ( LIFEPOINTS_AFTER_RESET + 1);
        let &new_incubations = self.fishes.get(&(turn as usize)).unwrap();
        // Update incubations
        self.fishes.insert(turn as usize, self.fishes.get(&(turn as usize)).unwrap() + self.incubating.get(&1).unwrap());
        self.incubating.insert(1, *self.incubating.get(&2).unwrap());
        self.incubating.insert(2, new_incubations);
        
        self.days_done = self.days_done + 1;
    }
}

impl fmt::Display for SeaFloor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        let mut line : String = String::new();
        if self.days_done == 0 { line = format!("Initial state: ") }
        else { line = format!("After {: >2} day: ", self.days_done) }

        let lifes_part = self.fishes.iter()
            .map(|fish| fish.get_lifepoints().to_string())
            .collect::<Vec<String>>().join(",");
        line = format!("{}{}", line, lifes_part);

        return write!(f, "{}", line);
    }
}