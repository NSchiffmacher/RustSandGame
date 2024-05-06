use crate::sandsim::particle::*;
use crate::sandsim::particle_action::ParticleAction;
use crate::sandsim::grid::Position;

pub type FloatPosition = (f64, f64);
pub type BehaviorId = u16;

pub const MOVE_DOWN_ID: BehaviorId = 1 << 1;
pub const AIR_LIKE_ID: BehaviorId = 1 << 2;
pub const LIMITED_LIFE_ID: BehaviorId = 1 << 3;
pub const ANIMATED_COLOR_ID: BehaviorId = 1 << 4;
pub const FLAMMABLE_ID: BehaviorId = 1 << 5;
pub const DIE_WHEN_CRUSHED_ID: BehaviorId = 1 << 6;
pub const IGNITER_ID: BehaviorId = 1 << 7;
pub const SIDEWAY_MOTION_FALLBACK: BehaviorId = 1 << 8;

mod move_down;
mod air_like;
mod limited_life;
mod animated_color;
mod flammable;
mod die_when_crushed;
mod igniter;
mod sideways_motion_fallback;

pub use move_down::MoveDown;
pub use air_like::AirLike;
pub use limited_life::LimitedLife;
pub use animated_color::AnimatedColor;
pub use flammable::Flammable;
pub use die_when_crushed::DieWhenCrushed;
pub use igniter::Igniter;
pub use sideways_motion_fallback::SidewaysMotionFallback;


pub trait Behavior {
    fn update(&mut self, state: &mut ParticleState, dt: f64, grid: &mut Vec<Vec<ParticleId>>, behaviors_grid: &mut Vec<Vec<BehaviorId>>) -> Vec<ParticleAction>;
    fn get_id(&self) -> BehaviorId;
}

fn has_behavior(position: Position, behaviors_grid: &Vec<Vec<BehaviorId>>, behavior_id: BehaviorId) -> bool {
    behaviors_grid[position.1 as usize][position.0 as usize] & behavior_id != 0
}
