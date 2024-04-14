use crate::sandsim::behaviors::*;

pub struct LimitedLife {
    elapsed_time: f64,

    lifetime: f64,
}

impl Behavior for LimitedLife {
    fn get_id(&self) -> BehaviorId {
        LIMITED_LIFE_ID
    }

    fn update(&mut self, position: Position, dt: f64, _grid: &mut Vec<Vec<ParticleId>>, _behaviors_grid: &mut Vec<Vec<BehaviorId>>) -> Vec<ParticleAction> {
        self.elapsed_time += dt;
        
        if self.elapsed_time >= self.lifetime {
            vec![ParticleAction::KillParticle{ position }]
        } else {
            vec![]
        }
    }
}

impl LimitedLife {
    pub fn boxed(lifetime: f64) -> Box<dyn Behavior> {
        Box::new(LimitedLife {
            elapsed_time: 0.,
            lifetime,
        })
    }
}