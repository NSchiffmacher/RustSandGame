use crate::sandsim::particle::*;
use crate::sandsim::particle_action::ParticleAction;
use crate::sandsim::grid::Position;

pub type FloatPosition = (f64, f64);


pub trait Behavior {
    fn update(&mut self, dt: f64, grid: &mut Vec<Vec<ParticleId>>) -> Vec<ParticleAction>;
}

pub struct MoveDown {
    acceleration: f64,
    max_velocity: f64,
    velocity: f64,

    float_position: FloatPosition,
    integer_position: Position,
}

impl Behavior for MoveDown {
    fn update(&mut self, dt: f64, grid: &mut Vec<Vec<ParticleId>>) -> Vec<ParticleAction> {
        // Regular update
        self.velocity += self.max_velocity.min(self.acceleration * dt);
        self.float_position.1 += self.velocity * dt;

        // Check if we changed grid cell 
        let new_position = self.to_integer_position();
        if new_position == self.integer_position {
            return vec![]; // We did not move (or moved within the same cell)
        }

        // We moved to a new cell, check if its in bounds
        if new_position.1 < 0 || new_position.1 >= grid.len() as i32 { // Only check vertical axis because can only fall down
            self.velocity = 0.;
            self.float_position.1 = self.integer_position.1 as f64; // Snap back to the integer position ? Not sure
            return vec![]; // We are out of bounds
        }

        // We are in bounds, check if the cell is empty
        if grid[new_position.1 as usize][new_position.0 as usize] != EMPTY_ID {
            // The cell is not empty, we need to stop moving
            self.velocity = 0.;
            self.float_position.1 = self.integer_position.1 as f64; // Snap back to the integer position ? Not sure

            return vec![];
        }

        grid[new_position.1 as usize][new_position.0 as usize] = grid[self.integer_position.1 as usize][self.integer_position.0 as usize];
        grid[self.integer_position.1 as usize][self.integer_position.0 as usize] = EMPTY_ID;

        // The cell is empty, we can move there
        self.integer_position = new_position;
        vec![ParticleAction::SetPosition { position: new_position }]
    }
}

impl MoveDown {
    pub fn boxed(position: Position, max_velocity: f64, acceleration: f64) -> Box<dyn Behavior> {
        let float_position = (position.0 as f64, position.1 as f64);
        Box::new(MoveDown { float_position, integer_position: position, max_velocity, acceleration, velocity: 0.})
    }

    fn to_integer_position(&self) -> Position {
        (self.float_position.0.round() as i32, self.float_position.1.round() as i32)
    }
}