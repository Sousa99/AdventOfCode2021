
// ================================================== STRUCTS ==================================================;

type PositionUnit = i64;
type VelocityUnit = i64;
type AccelarationUnit = i64;

#[derive(Copy, Clone)]
pub struct Position {
    pub x:  PositionUnit,
    pub y:  PositionUnit,
}

#[derive(Copy, Clone)]
pub struct Velocity {
    pub x:  VelocityUnit,
    pub y:  VelocityUnit,
}

#[derive(Copy, Clone)]
pub struct Accelaration {
    pub x:  AccelarationUnit,
    pub y:  AccelarationUnit,
}

pub struct Map {
    start_interval:         Position,
    end_interval:           Position,
    // Constants
    start_probe_position:   Position,
    gravity:                AccelarationUnit,
    drag:          AccelarationUnit,
}

// ================================================== AUX FUNCTIONS ==================================================

// ================================================== IMPLEMENTATIONS ==================================================

impl Map {

    pub fn new(x_start: PositionUnit, y_start: PositionUnit, x_end: PositionUnit, y_end: PositionUnit) -> Map {

        if x_start <= 0 || x_end <= 0 { panic!("🚨  Invalid choice of x limits!") }
        if y_start >= 0 || y_end >= 0 { panic!("🚨  Invalid choice of y limits!") }

        Map {
            start_interval: Position{ x: x_start, y: y_start },
            end_interval: Position{ x: x_end, y: y_end },
            // Constants
            start_probe_position: Position{ x: 0, y: 0},
            gravity: -1,
            drag: -1,
        }
    }

    fn compute_position_velocity_x_constant(&self, mut velocity_x: VelocityUnit) -> PositionUnit {

        let mut current_position_x : PositionUnit = self.start_probe_position.x;
        while velocity_x > 0 {

            // Update to next
            current_position_x = current_position_x + velocity_x;
            if velocity_x > 0 { velocity_x = velocity_x + self.drag; }
        }

        return current_position_x;
    }

    fn check_if_velocity_hits(&self, mut velocity: Velocity) -> bool {

        let mut current_point : Position = self.start_probe_position;
        while current_point.x <= self.end_interval.x && current_point.y >= self.start_interval.y {

            if current_point.x <= self.end_interval.x && current_point.y >= self.start_interval.y &&
                current_point.x >= self.start_interval.x && current_point.y <= self.end_interval.y {
                return true;
            }

            // Update to next
            current_point.x = current_point.x + velocity.x;
            current_point.y = current_point.y + velocity.y;
            if velocity.x > 0 { velocity.x = velocity.x + self.drag; }
            velocity.y = velocity.y + self.gravity;
        }

        return false;
    }

    fn compute_heighest_point(&self, mut velocity: Velocity) -> PositionUnit {

        let mut max_height : PositionUnit = self.start_probe_position.y;
        let mut current_point : Position = self.start_probe_position;
        while velocity.y >= 0 {

            if current_point.y > max_height { max_height = current_point.y; }

            // Update to next
            current_point.x = current_point.x + velocity.x;
            current_point.y = current_point.y + velocity.y;
            if velocity.x > 0 { velocity.x = velocity.x + self.drag; }
            velocity.y = velocity.y + self.gravity;
        }

        return max_height;
    }

    pub fn compute_highest_y_velocity(&self) -> (Velocity, PositionUnit) {

        let mut x_min : VelocityUnit = 0;
        let x_max : VelocityUnit = self.end_interval.x;
        let y_min : VelocityUnit = - self.start_interval.y.abs();
        let y_max : VelocityUnit = self.start_interval.y.abs();
        
        for possible_x_min in x_max..0 {

            let stable_position = self.compute_position_velocity_x_constant(possible_x_min);
            if stable_position < self.start_interval.x { break }
            x_min = possible_x_min;
        }

        let mut max_velocity = Velocity{x: 0, y: 0};
        let mut max_height : PositionUnit = 0;

        for x_velocity in x_min..=x_max {
            for y_velocity in y_min..=y_max {

                let tmp_velocity = Velocity{ x: x_velocity, y: y_velocity };
                let hit = self.check_if_velocity_hits(tmp_velocity);
                if !hit { continue }

                let heighest_position = self.compute_heighest_point(tmp_velocity);
                if heighest_position > max_height {
                    max_height = heighest_position;
                    max_velocity = tmp_velocity;
                }
            }
        }

        return (max_velocity, max_height);
    }

    pub fn compute_number_of_shots(&self) -> usize {

        let mut x_min : VelocityUnit = 0;
        let x_max : VelocityUnit = self.end_interval.x;
        let y_min : VelocityUnit = - self.start_interval.y.abs();
        let y_max : VelocityUnit = self.start_interval.y.abs();
        
        for possible_x_min in x_max..0 {

            let stable_position = self.compute_position_velocity_x_constant(possible_x_min);
            if stable_position < self.start_interval.x { break }
            x_min = possible_x_min;
        }

        let mut hit_shots : usize = 0;
        for x_velocity in x_min..=x_max {
            for y_velocity in y_min..=y_max {

                let tmp_velocity = Velocity{ x: x_velocity, y: y_velocity };
                let hit = self.check_if_velocity_hits(tmp_velocity);
                if !hit { continue }

                hit_shots = hit_shots + 1;
            }
        }

        return hit_shots;
    }
}