use rand::Rng;
use sdl2::pixels::Color;

use crate::color;
use crate::sandsim::behaviors::*;
use crate::sandsim::particle_action::ParticleAction;
use crate::sandsim::grid::Position;

pub const SAND_CELL_COLOR: Color = Color { r: 246, g: 215, b: 176, a: 255 };
pub const EMPTY_CELL_COLOR: Color = Color { r: 0, g: 0, b: 0, a: 255 };
pub const WOOD_CELL_COLOR: Color = Color { r: 68, g: 48, b: 34, a: 255 };
pub const SMOKE_CELL_COLOR: Color = Color { r: 76, g: 74, b: 77, a: 255 };
pub const WATER_CELL_COLOR: Color = Color { r: 30, g: 120, b: 190, a: 255 };

pub type ParticleId = u8;
pub const EMPTY_ID: ParticleId = 0;
pub const SAND_ID: ParticleId = 1;
pub const WOOD_ID: ParticleId = 2;
pub const SMOKE_ID: ParticleId = 3;
pub const FIRE_ID: ParticleId = 4;
pub const WATER_ID: ParticleId = 5;

pub struct Particle {
    state: ParticleState,
    behaviors: Vec<Box<dyn Behavior>>,

    modified: bool,
    required_actions: Vec<ParticleAction>,
}

#[derive(Clone)]
pub struct ParticleState {
    pub color: Color,
    pub position: Position,
    
    pub particle_id: ParticleId,
    pub behaviors_ids: BehaviorId,
}

impl Particle {
    pub fn update(&mut self, position: Position, dt: f64, grid: &mut Vec<Vec<ParticleId>>, behaviors_grid: &mut Vec<Vec<BehaviorId>>) -> bool { 
        let mut actions = vec![];
        self.modified = false;
        self.state.position = position; // Reset position to the new one, in case it was changed my another behavior
        self.required_actions = vec![];

        for behavior in self.behaviors.iter_mut() {
            actions.extend(behavior.update(&mut self.state, dt, grid, behaviors_grid));
        }

        for action in &actions {
            self.handle_action(action);
        }

        self.modified
    }

    pub fn handle_action(&mut self, action: &ParticleAction) {
        match action {
            ParticleAction::SetPosition { position } =>  {
                self.state.position = *position;
            },
            ParticleAction::KillParticle { .. } => {
                // Pass it to the grid
                self.required_actions.push(action.clone());
            },
            ParticleAction::SetColor { color } => {
                self.state.color = *color;
            },
            ParticleAction::SpawnParticle { .. } => {
                // Pass it to the grid
                self.required_actions.push(action.clone());
            }
        }
        self.modified = true;
    }

    pub fn get_color(&self) -> Color {
        self.state.color
    }

    pub fn get_id(&self) -> ParticleId {
        self.state.particle_id
    }

    pub fn get_required_actions(&self) -> Vec<ParticleAction> {
        self.required_actions.clone()
    }

    pub fn get_behaviors_ids(&self) -> BehaviorId {
        self.state.behaviors_ids
    }

    pub fn get_position(&self) -> Position {
        self.state.position
    }

    pub fn new(position: Position, color: Color, particle_id: ParticleId, behaviors: Vec<Box<dyn Behavior>>) -> Self {
        // Construct behaviors id
        let mut behaviors_ids = 0;
        for behavior in &behaviors {
            behaviors_ids |= behavior.get_id();
        }
        
        Self {
            state: ParticleState {
                color,
                position,
                particle_id,
                behaviors_ids,
            },
            modified: false,
            behaviors,
            required_actions: vec![],
        }
    }

    pub fn new_sand(position: Position) -> Self {
        let behaviors = vec![
            MoveDown::boxed(position, 8.0 * 60., 0.1 * 60. * 60.),
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
            vec![
                Flammable::boxed(0.1, 3),
            ])
    }

    pub fn new_smoke(position: Position) -> Self {
        let lifetime = rand::thread_rng().gen_range(4.0..=7.5);

        let behaviors = vec![
            MoveDown::boxed(position, 0.5 * 60., -0.003 * 60. * 60.),
            AirLike::boxed(),
            LimitedLife::boxed(lifetime),
        ];
        Self::new(position, color::vary_color(SMOKE_CELL_COLOR, 3), SMOKE_ID, behaviors)
    }

    pub fn new_fire(position: Position) -> Self {
        let mut rng = rand::thread_rng();
        let lifetime = rng.gen_range(1.0..=3.0);
        let frequency = rng.gen_range(5.0..=10.);
        let behaviors = vec![
            LimitedLife::boxed_with_spawn(
                lifetime,
                0.85,
                |pos| Self::new_smoke(pos),
                (1, 1)),
            AnimatedColor::boxed(vec![
                color::vary_color(Color::RGBA(84, 30, 30, 255), 10),
                color::vary_color(Color::RGBA(255, 31, 31, 255), 10),
                color::vary_color(Color::RGBA(234, 90, 0, 255), 10),
                color::vary_color(Color::RGBA(255, 105, 0, 255), 10),
                color::vary_color(Color::RGBA(238, 204, 9, 255), 10),
            ], frequency),
            DieWhenCrushed::boxed(0.5),
            Igniter::boxed(),
        ];
        Self::new(position, Color::YELLOW, FIRE_ID, behaviors)
    }

    pub fn new_water(position: Position) -> Self {
        let behaviors = vec![
            MoveDown::boxed(position, 8.0 * 60., 0.1 * 60. * 60.),
            SidewaysMotionFallback::boxed(&position),
        ];
        Self::new(position, color::vary_color(WATER_CELL_COLOR, 3), WATER_ID, behaviors)
    }
}
