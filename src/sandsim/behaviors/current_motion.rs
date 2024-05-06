use crate::sandsim::behaviors::*;

/// `CurrentMotion` is a struct that implements the `Behavior` trait.
/// This behavior is responsible for simulating a water-like sideways current
pub struct CurrentMotion {
    swap_probability_per_sec: f64,
}

impl Behavior for CurrentMotion {
    fn get_id(&self) -> BehaviorId {
        CURRENT_MOTION_ID
    }

    fn update(&mut self, state: &mut ParticleState, dt: f64, grid: &mut Vec<Vec<ParticleId>>, behaviors_grid: &mut Vec<Vec<BehaviorId>>) -> Vec<ParticleAction> {
        let width = grid[0].len() as i32;

        // Pick a random side
        let dx = if rand::random::<f64>() < 0.5 { -1 } else { 1 };
        let x = state.position.0;
        let nx = x + dx;

        // If both particle are the same time, and given the swap_probability, swap them
        if nx >= 0 && nx < width && rand::random::<f64>() < self.swap_probability_per_sec * dt {
            // Expect them to have the same behavior ID and particle ID
            if grid[state.position.1 as usize][nx as usize] != grid[state.position.1 as usize][x as usize] {
                return vec![];
            }
            if behaviors_grid[state.position.1 as usize][nx as usize] != behaviors_grid[state.position.1 as usize][x as usize] {
                return vec![];
            }

            vec![ParticleAction::SetPosition { position: (nx, state.position.1)}]
        } else {
            vec![]
        }
    }
}

impl CurrentMotion {
    pub fn boxed(swap_probability_per_sec: f64) -> Box<dyn Behavior> {
        Box::new(Self {
            swap_probability_per_sec
        })
    }
}