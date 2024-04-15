use crate::sandsim::particle::*;
use crate::sandsim::particle_action::ParticleAction;
use crate::sandsim::grid::Position;

pub type FloatPosition = (f64, f64);
pub type BehaviorId = u8;

pub const MOVE_DOWN_ID: BehaviorId = 1 << 1;
pub const AIR_LIKE_ID: BehaviorId = 1 << 2;
pub const LIMITED_LIFE_ID: BehaviorId = 1 << 3;
pub const ANIMATED_COLOR_ID: BehaviorId = 1 << 4;

mod move_down;
mod air_like;
mod limited_life;
mod animated_color;

pub use move_down::MoveDown;
pub use air_like::AirLike;
pub use limited_life::LimitedLife;
pub use animated_color::AnimatedColor;


pub trait Behavior {
    fn update(&mut self, state: &ParticleState, dt: f64, grid: &mut Vec<Vec<ParticleId>>, behaviors_grid: &mut Vec<Vec<BehaviorId>>) -> Vec<ParticleAction>;
    fn get_id(&self) -> BehaviorId;
}

fn has_behavior(position: Position, behaviors_grid: &Vec<Vec<BehaviorId>>, behavior_id: BehaviorId) -> bool {
    behaviors_grid[position.1 as usize][position.0 as usize] & behavior_id != 0
}
