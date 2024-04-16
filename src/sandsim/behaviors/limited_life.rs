use crate::sandsim::behaviors::*;
use crate::color::darken_color;

pub struct LimitedLife {
    elapsed_time: f64,

    lifetime: f64,
}

impl Behavior for LimitedLife {
    fn update(&mut self, state: &ParticleState, dt: f64, _grid: &mut Vec<Vec<ParticleId>>, _behaviors_grid: &mut Vec<Vec<BehaviorId>>) -> Vec<ParticleAction> {
        self.elapsed_time = self.lifetime.min(self.elapsed_time + dt);
        
        let t = (self.elapsed_time / self.lifetime) as f32; // t = 0 => start, t = 1 => end
        let lightness = 100. * (1. - t);
        let color = darken_color(state.color, lightness);

        let mut actions = vec![ParticleAction::SetColor { color }];

        if self.elapsed_time >= self.lifetime {
            actions.push(ParticleAction::KillParticle { position: state.position });
        } 

        actions
    }

    fn get_id(&self) -> BehaviorId {
        LIMITED_LIFE_ID
    }
}

impl LimitedLife {
    pub fn boxed(lifetime: f64) -> Box<dyn Behavior> {
        Box::new(Self {
            elapsed_time: 0.,
            lifetime,
        })
    }
}