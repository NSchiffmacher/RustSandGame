use crate::sandsim::behaviors::*;

pub struct AirLike {}

impl Behavior for AirLike {
    fn get_id(&self) -> BehaviorId {
        AIR_LIKE_ID
    }

    fn update(&mut self, _state: &ParticleState, _dt: f64, _grid: &mut Vec<Vec<ParticleId>>, _behaviors_grid: &mut Vec<Vec<BehaviorId>>) -> Vec<ParticleAction> {
        vec![]
    }
}

impl AirLike {
    pub fn boxed() -> Box<dyn Behavior> {
        Box::new(Self {})
    }
}