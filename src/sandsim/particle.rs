
use sdl2::pixels::Color;
use crate::color;

pub const SAND_CELL_COLOR: Color = Color { r: 246, g: 215, b: 176, a: 255 };
pub const EMPTY_CELL_COLOR: Color = Color { r: 0, g: 0, b: 0, a: 255 };

pub const EMPTY_ID: u8 = 0;
pub const SAND_ID: u8 = 1;

pub trait Particle {
    fn get_color(&self) -> Color;
    fn update(&mut self) {}
    fn get_id(&self) -> u8;
    fn get_update_count(&self) -> u8 { 0 }
    fn was_modified(&self) -> bool { false }
    fn reset_velocity(&mut self) { }
}

// ===================== SAND CELL =======================
pub struct Sand {
    color: Color,
    max_speed: f64,
    acceleration: f64,
    velocity: f64,
    modified: bool,
}

impl Particle for Sand {
    fn get_color(&self) -> Color {
        self.color
    }

    fn update(&mut self) {
        self.update_velocity();
    }

    fn get_id(&self) -> u8 {
        SAND_ID
    }

    fn get_update_count(&self) -> u8 {
        if !self.modified { return 0; } // Would be the result of the following code but maybe speeds up ?

        let floored = self.velocity.abs().floor();
        let remainder = self.velocity.abs() - floored;
        return floored as u8 + (rand::random::<f64>() < remainder) as u8;
    }

    fn was_modified(&self) -> bool {
        self.modified
    }

    fn reset_velocity(&mut self) {
        self.velocity = 0.0;
    }
}

impl Sand {
    pub fn boxed() -> Box<dyn Particle> {
        Box::new(Sand {
            color: color::vary_color(SAND_CELL_COLOR, 10),
            max_speed: 8.0,
            acceleration: 0.4,
            velocity: 0.0,
            modified: false,
        })
    }

    pub fn update_velocity(&mut self) {
        let mut new_vel = self.velocity + self.acceleration;

        if new_vel.abs() > self.max_speed {
            new_vel = self.max_speed * new_vel.signum();
        }

        self.velocity = new_vel;
        self.modified = self.velocity != 0.;
    }
}
// =================== END SAND CELL =======================

// ===================== EMPTY CELL =======================
pub struct Empty {
}

impl Particle for Empty {
    fn get_color(&self) -> Color {
        EMPTY_CELL_COLOR
    }

    fn update(&mut self) {
    }

    fn get_id(&self) -> u8 {
        EMPTY_ID
    }
}

impl Empty {
    pub fn boxed() -> Box<dyn Particle> {
        Box::new(Empty {})
    }
}
// =================== END EMPTY CELL =====================
