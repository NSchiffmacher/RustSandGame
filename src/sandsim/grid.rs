use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::pixels::Color;

use std::collections::HashSet;

pub type Cell = Option<Color>;
pub const EMPTY_CELL_COLOR: Color = Color { r: 0, g: 0, b: 0, a: 255 };
pub const SAND_CELL_COLOR: Color = Color { r: 246, g: 215, b: 176, a: 255 };
pub const PIXEL_SIZE: i32 = 3;

pub struct Grid {
    pub width: i32,
    pub height: i32,
    pub cells: Vec<Cell>,
    pub cells_to_draw: HashSet<(i32, i32)>,
}

#[allow(dead_code)]
impl Grid {
    pub fn new(window_width: i32, window_height: i32) -> Grid {
        let width = window_width / PIXEL_SIZE;
        let height = window_height / PIXEL_SIZE;
        let cells = vec![None; (width * height) as usize];
        Grid {
            width,
            height,
            cells,
            cells_to_draw: HashSet::new(),
        }
    }

    pub fn clear(&mut self) {
        self.cells = vec![None; (self.width * self.height) as usize];
        for y in 0..self.height/PIXEL_SIZE {
            for x in 0..self.width/PIXEL_SIZE {
                self.cells_to_draw.insert((x, y));
            }
        }
    }

    pub fn set(&mut self, x: i32, y: i32, value: Cell) {
        if y >= self.height || x >= self.width || y < 0 || x < 0{
            return;
        }

        self.cells[(y * self.width + x) as usize] = value;
        self.cells_to_draw.insert((x, y));
    }

    pub fn get(&self, x: i32, y: i32) -> Cell {
        if y >= self.height || x >= self.width || y < 0 || x < 0{
            return None;
        }

        self.cells[(y * self.width + x) as usize]
    }

    pub fn swap(&mut self, x1: i32, y1: i32, x2: i32, y2: i32) {
        let a = y1 * self.width + x1;
        let b = y2 * self.width + x2;
        let temp = self.cells[a as usize];
        self.cells[a as usize] = self.cells[b as usize];
        self.cells[b as usize] = temp;
        self.cells_to_draw.insert((x1, y1));
        self.cells_to_draw.insert((x2, y2));
    }

    pub fn is_empty(&self, x: i32, y: i32) -> bool {
        self.get(x, y).is_none()
    }

    pub fn draw(&mut self, canvas: &mut Canvas<Window>) {
        for (x, y) in &self.cells_to_draw {
            let color = self.get(*x, *y).unwrap_or(EMPTY_CELL_COLOR);
            let rect = sdl2::rect::Rect::new(*x * PIXEL_SIZE, *y * PIXEL_SIZE, PIXEL_SIZE as u32, PIXEL_SIZE as u32);
            canvas.set_draw_color(color);
            canvas.fill_rect(rect).unwrap();
        }
        self.cells_to_draw.clear();
    }

    pub fn update_pixel(&mut self, x: i32, y: i32) {
        if self.is_empty(x, y+1) {
            self.swap(x, y, x, y+1);
        }
    }

    pub fn update(&mut self) {
        for y in 0..self.height {
            for x in 0..self.width {
                self.update_pixel(x, y);
            }
        }
    }
}