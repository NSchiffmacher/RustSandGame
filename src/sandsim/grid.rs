use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::pixels::Color;
use sdl2::gfx::primitives::DrawRenderer; 

type Cell = Option<Color>;
const EMPTY_CELL_COLOR: Color = Color { r: 0, g: 0, b: 0, a: 255 };

pub struct Grid {
    pub width: u32,
    pub height: u32,
    pub cells: Vec<Cell>,
}

#[allow(dead_code)]
impl Grid {
    pub fn new(width: u32, height: u32) -> Grid {
        let cells = vec![None; (width * height) as usize];
        Grid {
            width,
            height,
            cells,
        }
    }

    pub fn clear(&mut self) {
        self.cells = vec![None; (self.width * self.height) as usize];
    }

    pub fn set(&mut self, x: u32, y: u32, value: Cell) {
        self.cells[(y * self.width + x) as usize] = value;
    }

    pub fn get(&self, x: u32, y: u32) -> Cell {
        self.cells[(y * self.width + x) as usize]
    }

    pub fn swap(&mut self, a: u32, b: u32) {
        let temp = self.cells[a as usize];
        self.cells[a as usize] = self.cells[b as usize];
        self.cells[b as usize] = temp;
    }

    pub fn is_empty(&self, x: u32, y: u32) -> bool {
        self.get(x, y).is_none()
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>) {
        for y in 0..self.height {
            for x in 0..self.width {
                let color = self.get(x, y).unwrap_or(EMPTY_CELL_COLOR);
                canvas.pixel(x as i16, y as i16, color).unwrap();
            }
        }
    }
}