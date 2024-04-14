use crate::sandsim::particle::ParticleId;
use crate::sandsim::particle_action::ParticleAction;
use crate::sandsim::grid::Position;

use sdl2::pixels::Color;

pub type FloatPosition = (f64, f64);


pub trait Behavior {
    fn update(&mut self, grid: &mut Vec<Vec<ParticleId>>) -> Vec<ParticleAction>;
}

pub struct MoveDown {
    float_position: FloatPosition,
    max_speed: f64,
    acceleration: f64,
    velocity: f64,
}

impl Behavior for MoveDown {
    fn update(&mut self, grid: &mut Vec<Vec<ParticleId>>) -> Vec<ParticleAction> {
        vec![
            // ParticleAction::SetColor { color: Color { r: 244, g: 0, b: 0, a: 255 } }
        ]
    }
}

impl MoveDown {
    pub fn boxed(position: Position, max_speed: f64, acceleration: f64) -> Box<dyn Behavior> {
        let float_position = (position.0 as f64 + 0.5, position.1 as f64 + 0.5);
        Box::new(MoveDown { float_position, max_speed, acceleration, velocity: 0.})
    }
}