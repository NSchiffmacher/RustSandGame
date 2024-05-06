use sdl2::pixels::Color;

use crate::sandsim::behaviors::*;

pub struct AnimatedColor {
    colors: Vec<Color>,
    frequency: f64,

    elapsed_time: f64,
    last_index: usize,
}

impl Behavior for AnimatedColor {
    fn get_id(&self) -> BehaviorId {
        ANIMATED_COLOR_ID
    }

    fn update(&mut self, _state: &mut ParticleState, dt: f64, _grid: &mut Vec<Vec<ParticleId>>, _behaviors_grid: &mut Vec<Vec<BehaviorId>>) -> Vec<ParticleAction> {
        self.elapsed_time += dt;
        let mut index = (self.elapsed_time * self.frequency).floor() as usize;
        if index >= self.colors.len() {
            index = 0;
            self.elapsed_time = 0.;
        }

        if self.last_index != index {
            self.last_index = index;
            return vec![
                ParticleAction::SetColor{ color: self.colors[index] }
            ];
        }
        
        vec![]
    }
}

impl AnimatedColor {
    pub fn boxed(colors: Vec<Color>, frequency: f64) -> Box<dyn Behavior> {
        Box::new(Self { colors, frequency, elapsed_time: 0., last_index: 1, })
    }
}