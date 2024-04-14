use crate::sandsim::particle::ParticleId;
use crate::sandsim::particle_action::ParticleAction;
use crate::sandsim::grid::Position;

use sdl2::pixels::Color;
use std::collections::HashMap;


pub trait Behavior {
    fn update(&mut self, grid: &mut Vec<Vec<ParticleId>>) -> Vec<ParticleAction>;
}

pub struct MoveDown {
    max_speed: f64,
    acceleration: f64,
    velocity: f64,
}

impl Behavior for MoveDown {
    fn update(&mut self, grid: &mut Vec<Vec<ParticleId>>) -> Vec<ParticleAction> {
        vec![
            // ParticleAction::AddSwap((0, 0), (0, -1))
        ]
    }
}

impl MoveDown {
    pub fn boxed(max_speed: f64, acceleration: f64, grid_size: Position) -> Box<dyn Behavior> {
        Box::new(MoveDown { max_speed, acceleration, velocity: 0.})
    }
}