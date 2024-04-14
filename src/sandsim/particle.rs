use sdl2::pixels::Color;

use crate::color;
use crate::sandsim::behavior::*;
use crate::sandsim::particle_action::ParticleAction;
use crate::sandsim::grid::Position;

pub const SAND_CELL_COLOR: Color = Color { r: 246, g: 215, b: 176, a: 255 };
pub const EMPTY_CELL_COLOR: Color = Color { r: 0, g: 0, b: 0, a: 255 };
pub const WOOD_CELL_COLOR: Color = Color { r: 68, g: 48, b: 34, a: 255 };
pub const SMOKE_CELL_COLOR: Color = Color { r: 76, g: 74, b: 77, a: 255 };

pub type ParticleId = u8;
pub const EMPTY_ID: ParticleId = 0;
pub const SAND_ID: ParticleId = 1;
pub const WOOD_ID: ParticleId = 2;
pub const SMOKE_ID: ParticleId = 3;

pub struct Particle {
    color: Color,
    particle_id: ParticleId,
    behaviors_ids: BehaviorId,
    behaviors: Vec<Box<dyn Behavior>>,
    position: Position,

    modified: bool,
}

impl Particle {
    pub fn update(&mut self, dt: f64, grid: &mut Vec<Vec<ParticleId>>, behaviors_grid: &mut Vec<Vec<BehaviorId>>) -> bool { 
        let mut actions = vec![];
        self.modified = false;

        for behavior in self.behaviors.iter_mut() {
            actions.extend(behavior.update(dt, grid, behaviors_grid));
        }

        for action in &actions {
            self.handle_action(action);
        }

        self.modified
    }

    pub fn handle_action(&mut self, action: &ParticleAction) {
        match action {
            // ParticleAction::SetColor{color} => {
            //     self.color = *color;
            //     self.modified = true;
            // },
            ParticleAction::SetPosition { position } =>  {
                self.position = *position;
                self.modified = true;
            },
        }
    }

    pub fn get_color(&self) -> Color {
        self.color
    }

    pub fn get_id(&self) -> ParticleId {
        self.particle_id
    }

    pub fn get_behaviors_ids(&self) -> BehaviorId {
        self.behaviors_ids
    }

    pub fn get_position(&self) -> Position {
        self.position
    }

    pub fn new(position: Position, color: Color, particle_id: ParticleId, behaviors: Vec<Box<dyn Behavior>>) -> Self {
        // Construct behaviors id
        let mut behaviors_ids = 0;
        for behavior in &behaviors {
            behaviors_ids |= behavior.get_id();
        }
        
        Self {
            color,
            position,
            particle_id,
            modified: false,
            behaviors,
            behaviors_ids,
        }
    }

    pub fn new_sand(position: Position) -> Self {
        let behaviors = vec![
            MoveDown::boxed(position, 8.0 * 60., 0.004 * 60. * 60.),
        ];
        Self::new(position, color::vary_color(SAND_CELL_COLOR, 10), SAND_ID, behaviors)
    }

    pub fn new_empty(position: Position) -> Self {
        let behaviors = vec![
            AirLike::boxed(),
        ];
        Self::new(position, EMPTY_CELL_COLOR, EMPTY_ID, behaviors)
    }

    pub fn new_wood(position: Position) -> Self {
        Self::new(
            position,
            color::vary_color(WOOD_CELL_COLOR, 10),
            WOOD_ID,
            vec![])
    }

    pub fn new_smoke(position: Position) -> Self {
        let behaviors = vec![
            MoveDown::boxed(position, 0.05 * 60., -0.001 * 60. * 60.),
            AirLike::boxed(),
        ];
        Self::new(position, color::vary_color(SMOKE_CELL_COLOR, 3), SMOKE_ID, behaviors)
    }
}
