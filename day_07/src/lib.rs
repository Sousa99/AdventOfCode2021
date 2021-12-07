
// ================================================== STRUCTS ==================================================

type HorizontalPosition = i64;
type Fuel = f64;

pub enum CrabEngineering { Constant, Incremental }
pub struct CrabArmy {
    engineering:        CrabEngineering,
    crab_positions:     Vec<HorizontalPosition>,
}

// ================================================== AUX FUNCTIONS ==================================================

// ================================================== IMPLEMENTATIONS ==================================================

impl CrabArmy {

    pub fn new(engineering: CrabEngineering, positions: &Vec<HorizontalPosition>) -> CrabArmy {
        CrabArmy {
            engineering:        engineering,
            crab_positions:     positions.clone(),
        }
    }

    fn fuel_used_to_align(&self, position: HorizontalPosition) -> Fuel {

        let mut fuel : Fuel = 0.0;
        for &crab_position in self.crab_positions.iter() {

            match self.engineering {
                CrabEngineering::Constant => { fuel = fuel + (crab_position - position).abs() as Fuel; }
                CrabEngineering::Incremental => {

                    let n = (crab_position - position).abs() as Fuel;
                    let first : Fuel = 1.0;
                    let last : Fuel = (crab_position - position).abs() as Fuel;
                    let sub_fuel = ( n / 2.0 ) * ( first + last );
                    
                    fuel = fuel + sub_fuel;
                }
            }
        }

        return fuel;
    }

    pub fn minimum_align_position(&self) -> Option<(HorizontalPosition, Fuel)> {

        let (mut minimum_position, mut maximum_position) = (self.crab_positions[0], self.crab_positions[0]);
        for &crab_position in self.crab_positions.iter() {
            minimum_position = std::cmp::min(minimum_position, crab_position);
            maximum_position = std::cmp::max(maximum_position, crab_position);
        }

        let mut minimum_info : Option<(HorizontalPosition, Fuel)> = None;
        for position in minimum_position..(maximum_position + 1) {
            
            let fuel_needed = self.fuel_used_to_align(position);
            if minimum_info.is_none() || fuel_needed < minimum_info.unwrap().1 {
                minimum_info = Some((position, fuel_needed));
            }
        }

        return minimum_info;
    }
}