
use sdl2::pixels::Color;
use crate::color;

pub const SAND_CELL_COLOR: Color = Color { r: 246, g: 215, b: 176, a: 255 };
pub const EMPTY_CELL_COLOR: Color = Color { r: 0, g: 0, b: 0, a: 255 };

pub const EMPTY_ID: u8 = 0;
pub const SAND_ID: u8 = 1;

pub trait Particle {
    fn get_color(&self) -> Color;
    fn update(&mut self);
    fn get_id(&self) -> u8;
}

pub struct Empty {
}

pub struct Sand {
    color: Color,
}

impl Particle for Sand {
    fn get_color(&self) -> Color {
        self.color
    }

    fn update(&mut self) {
        //TODO Implement sand update
    }

    fn get_id(&self) -> u8 {
        SAND_ID
    }
}

impl Sand {
    pub fn boxed() -> Box<dyn Particle> {
        Box::new(Sand {
            color: color::vary_color(SAND_CELL_COLOR, 10),
        })
    }
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