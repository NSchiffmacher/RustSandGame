use sdl2::pixels::Color;
use crate::sandsim::behaviors::*;

pub struct LimitedLife {
    elapsed_time: f64,
    initial_color: Color,
    target_color: Color,

    lifetime: f64,
}

impl Behavior for LimitedLife {
    fn get_id(&self) -> BehaviorId {
        LIMITED_LIFE_ID
    }

    fn update(&mut self, state: &ParticleState, dt: f64, _grid: &mut Vec<Vec<ParticleId>>, _behaviors_grid: &mut Vec<Vec<BehaviorId>>) -> Vec<ParticleAction> {
        self.elapsed_time = self.lifetime.min(self.elapsed_time + dt);
        
        let prc = self.elapsed_time / self.lifetime;
        let mut actions = vec![ParticleAction::SetColor { color: color_interpo(self.initial_color, self.target_color, prc)}];

        if self.elapsed_time >= self.lifetime {
            actions.push(ParticleAction::KillParticle { position: state.position });
        } 

        actions
    }
}

impl LimitedLife {
    pub fn boxed(lifetime: f64, initial_color: Color, target_color: Color) -> Box<dyn Behavior> {
        Box::new(Self {
            elapsed_time: 0.,
            initial_color, 
            target_color,
            lifetime,
        })
    }
}

fn color_interpo(a: Color, b: Color, t: f64) -> Color {
    Color {
        r: (a.r as f64 + (b.r as f64 - a.r as f64) * t).floor() as u8,
        g: (a.g as f64 + (b.g as f64 - a.g as f64) * t).floor() as u8,
        b: (a.b as f64 + (b.b as f64 - a.b as f64) * t).floor() as u8,
        a: (a.a as f64 + (b.a as f64 - a.a as f64) * t).floor() as u8,
    }
}