use sdl2::pixels::Color;

use crate::color;
use crate::sandsim::behavior::*;
use crate::sandsim::particle_action::ParticleAction;

pub const SAND_CELL_COLOR: Color = Color { r: 246, g: 215, b: 176, a: 255 };
pub const EMPTY_CELL_COLOR: Color = Color { r: 0, g: 0, b: 0, a: 255 };
pub const WOOD_CELL_COLOR: Color = Color { r: 68, g: 48, b: 34, a: 255 };

pub type ParticleId = u8;
pub const EMPTY_ID: ParticleId = 0;
pub const SAND_ID: ParticleId = 1;
pub const WOOD_ID: ParticleId = 2;

pub struct Particle {
    color: Color,
    particle_id: ParticleId,
    modified: bool,
    behaviors: Vec<Box<dyn Behavior>>,
}

impl Particle {
    pub fn update(&mut self, grid: &Vec<ParticleId>) { 
        let mut actions = vec![];
        self.modified = false;

        for behavior in self.behaviors.iter_mut() {
            actions.extend(behavior.update(grid));
        }

        for action in &actions {
            self.handle_action(action);
        }
    }

    pub fn handle_action(&mut self, action: &ParticleAction) {
        match action {
            ParticleAction::SetColor{color} => {
                self.color = *color;
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

    pub fn get_update_count(&self) -> u8 { 
        0
    }

    pub fn was_modified(&self) -> bool { 
        self.modified
    }
    
    pub fn reset_velocity(&mut self) { 

    }

    pub fn new(color: Color, particle_id: ParticleId, behaviors: Vec<Box<dyn Behavior>>) -> Self {
        Self {
            color,
            particle_id,
            modified: false,
            behaviors,
        }
    }

    pub fn new_sand() -> Self {
        let behaviors = vec![
            MoveDown::boxed(8.0, 0.4),
        ];
        Self::new(color::vary_color(SAND_CELL_COLOR, 10), SAND_ID, behaviors)
    }

    pub fn new_empty() -> Self {
        Self::new(EMPTY_CELL_COLOR, EMPTY_ID, vec![])
    }

    pub fn new_wood() -> Self {
        Self::new(
            color::vary_color(WOOD_CELL_COLOR, 10),
            WOOD_ID,
            vec![])
    }
}

// // ===================== SAND CELL =======================
// pub struct Sand {
//     color: Color,
//     max_speed: f64,
//     acceleration: f64,
//     velocity: f64,
//     modified: bool,
// }

// impl Particle for Sand {
//     fn get_color(&self) -> Color {
//         self.color
//     }

//     fn set_color(&mut self, color: Color) {
//         self.color = color;
//     }

//     fn update(&mut self) {
//         self.update_velocity();
//     }

//     fn get_id(&self) -> ParticleId {
//         SAND_ID
//     }

//     fn get_update_count(&self) -> u8 {
//         if !self.modified { return 0; } // Would be the result of the following code but maybe speeds up ?

//         let floored = self.velocity.abs().floor();
//         let remainder = self.velocity.abs() - floored;
//         return floored as u8 + (rand::random::<f64>() < remainder) as u8;
//     }

//     fn was_modified(&self) -> bool {
//         self.modified
//     }

//     fn reset_velocity(&mut self) {
//         self.velocity = 0.0;
//     }
// }

// impl Sand {
//     pub fn boxed() -> Box<dyn Particle> {
//         Box::new(Sand {
//             color: color::vary_color(SAND_CELL_COLOR, 10),
//             max_speed: 8.0,
//             acceleration: 0.4,
//             velocity: 0.0,
//             modified: false,
//         })
//     }

//     pub fn update_velocity(&mut self) {
//         let mut new_vel = self.velocity + self.acceleration;

//         if new_vel.abs() > self.max_speed {
//             new_vel = self.max_speed * new_vel.signum();
//         }

//         self.velocity = new_vel;
//         self.modified = self.velocity != 0.;
//     }
// }
// // =================== END SAND CELL =======================

// // ===================== EMPTY CELL =======================
// pub struct Empty {
// }

// impl Particle for Empty {
//     fn get_color(&self) -> Color {
//         EMPTY_CELL_COLOR
//     }

//     fn update(&mut self) {
//     }

//     fn get_id(&self) -> ParticleId {
//         EMPTY_ID
//     }
// }

// impl Empty {
//     pub fn boxed() -> Box<dyn Particle> {
//         Box::new(Empty {})
//     }
// }
// // =================== END EMPTY CELL =====================

// // ===================== WOOD CELL =======================
// pub struct Wood {
//     color: Color,
// }

// impl Particle for Wood {
//     fn get_color(&self) -> Color {
//         self.color
//     }

//     fn update(&mut self) {
//     }

//     fn get_id(&self) -> ParticleId {
//         WOOD_ID
//     }
// }

// impl Wood {
//     pub fn boxed() -> Box<dyn Particle> {
//         Box::new(Wood {
//             color: color::vary_color(WOOD_CELL_COLOR, 10),
//         })
//     }
// }
// // =================== WOOD SAND CELL =======================