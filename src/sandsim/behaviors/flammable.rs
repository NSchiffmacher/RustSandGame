use crate::sandsim::behaviors::*;

pub struct Flammable {
    ignition_rate: f64, // The rate at which "current_ignite_probability" increases for each FIRE_ID in the given radius (Unit: prob/second)
    ignition_radius: i32, // The radius in which to check for FIRE_ID (Actually, checks on a square of side 2*ignition_radius + 1) 
    
    current_ignite_probability: f64, // The current probability of igniting
    num_cell_in_radius: f64, // The number of cells in the given radius
}

impl Behavior for Flammable {
    fn get_id(&self) -> BehaviorId {
        FLAMMABLE_ID
    }

    fn update(&mut self, state: &ParticleState, dt: f64, grid: &mut Vec<Vec<ParticleId>>, _behaviors_grid: &mut Vec<Vec<BehaviorId>>) -> Vec<ParticleAction> {
        // Increase ignite probability based on the number of FIRE_ID in the given radius
        for i in -self.ignition_radius..=self.ignition_radius {
            for j in -self.ignition_radius..=self.ignition_radius {
                if i == 0 && j == 0 {
                    continue;
                }
                // Check if there is a FIRE_ID in the given radius
                // If there is, increase the current_ignite_probability
                if grid[(state.position.1 + j) as usize][(state.position.0 + i) as usize] == FIRE_ID {
                    self.current_ignite_probability += self.ignition_rate * dt / self.num_cell_in_radius;
                }

            }
        }

        // Check if the current cell catches fire
        if rand::random::<f64>() < self.current_ignite_probability {
            return vec![
                ParticleAction::KillParticle { position: state.position },
                ParticleAction::SpawnParticle { callback: |pos| Particle::new_fire(pos), position: state.position },
            ];
        }

        vec![]
    }
}

impl Flammable {
    pub fn boxed(ignition_rate: f64, ignition_radius: i32) -> Box<dyn Behavior> {
        Box::new(Self {
            ignition_radius,
            ignition_rate,

            current_ignite_probability: 0.,
            num_cell_in_radius: ((2 * ignition_radius + 1) * (2 * ignition_radius + 1)) as f64,
        })
    }
}