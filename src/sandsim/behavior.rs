use crate::sandsim::particle::{ParticleId};

pub trait Behavior {
    fn update(&mut self, grid: &Vec<ParticleId>);
}

// pub struct MoveDown {
//     max_speed: f64,
//     acceleration: f64,
//     velocity: f64,
// }

// impl Behavior for MoveDown {
//     fn update(&mut self, grid: &Vec<ParticleId>) {
//     }
// }

// impl MoveDown {
//     pub fn boxed(max_speed: f64, acceleration: f64, velocity: f64) -> Box<dyn Behavior> {
//         Box::new(MoveDown { max_speed, acceleration, velocity })
//     }
// }