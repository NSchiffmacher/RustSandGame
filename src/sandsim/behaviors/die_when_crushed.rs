use crate::sandsim::behaviors::*;

pub struct DieWhenCrushed {
    crushing_probability: f64,
}

impl Behavior for DieWhenCrushed {
    fn get_id(&self) -> BehaviorId {
        DIE_WHEN_CRUSHED_ID
    }

    fn update(&mut self, state: &ParticleState, _dt: f64, grid: &mut Vec<Vec<ParticleId>>, behaviors_grid: &mut Vec<Vec<BehaviorId>>) -> Vec<ParticleAction> {
        // If the material on top of it is not AirLike, and the probability is met, kill the particle
        let above_x = state.position.0;
        let above_y = state.position.1 - 1;

        if above_y >= 0 
        && grid[above_y as usize][above_x as usize] != state.particle_id
        && !has_behavior((above_x, above_y), behaviors_grid, AIR_LIKE_ID)
        && rand::random::<f64>() < self.crushing_probability {
            return vec![
                ParticleAction::KillParticle { position: state.position },
            ];
        }

        vec![]
    }
}

impl DieWhenCrushed {
    pub fn boxed(crushing_probability: f64) -> Box<dyn Behavior> {
        Box::new(Self {
            crushing_probability,
        })
    }
}