use crate::sandsim::behaviors::*;

pub struct Igniter {}

impl Behavior for Igniter {
    fn get_id(&self) -> BehaviorId {
        IGNITER_ID
    }

    fn update(&mut self, _state: &mut ParticleState, _dt: f64, _grid: &mut Vec<Vec<ParticleId>>, _behaviors_grid: &mut Vec<Vec<BehaviorId>>) -> Vec<ParticleAction> {
        vec![]
    }
}

impl Igniter {
    pub fn boxed() -> Box<dyn Behavior> {
        Box::new(Self {})
    }
}