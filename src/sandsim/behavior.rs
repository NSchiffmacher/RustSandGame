use crate::sandsim::particle::ParticleId;
use crate::sandsim::particle_action::ParticleAction;
use crate::color;

use sdl2::pixels::Color;


pub trait Behavior {
    fn update(&mut self, grid: &Vec<ParticleId>) -> Vec<ParticleAction>;
}

pub struct MoveDown {
    max_speed: f64,
    acceleration: f64,
    velocity: f64,
}

impl Behavior for MoveDown {
    fn update(&mut self, grid: &Vec<ParticleId>) -> Vec<ParticleAction> {
        vec![
        ]
    }
}

impl MoveDown {
    pub fn boxed(max_speed: f64, acceleration: f64) -> Box<dyn Behavior> {
        Box::new(MoveDown { max_speed, acceleration, velocity: 0. })
    }
}