use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::pixels::Color;

use std::collections::HashSet;

use crate::sandsim::particle::*;

pub type Cell = Box<dyn Particle>;
pub type Position = (i32, i32);
pub const EMPTY_CELL_COLOR: Color = Color { r: 0, g: 0, b: 0, a: 255 };
pub const PIXEL_SIZE: i32 = 5;

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
        let cells = std::iter::repeat_with(|| Empty::boxed()).take((width * height) as usize).collect();
        Grid {
            width,
            height,
            cells,
            cells_to_draw: HashSet::new(),
        }
    }

    pub fn clear(&mut self) {
        self.cells = std::iter::repeat_with(|| Empty::boxed()).take((self.width * self.height) as usize).collect();
        for y in 0..self.height/PIXEL_SIZE {
            for x in 0..self.width/PIXEL_SIZE {
                self.cells_to_draw.insert((x, y));
            }
        }
    }

    pub fn set(&mut self, (x, y): Position, value: Cell) {
        if y >= self.height || x >= self.width || y < 0 || x < 0{
            return;
        }

        self.cells[(y * self.width + x) as usize] = value;
        self.cells_to_draw.insert((x, y));
    }

    pub fn set_circle(&mut self, (x, y): Position, particle_callback: fn(i32, i32) -> Cell, radius: i32, probability: f32) {
        for i in -radius..=radius {
            for j in -radius..=radius {
                if i * i + j * j <= radius * radius {
                    let new_x = x + i;
                    let new_y = y + j;
                    if new_x >= 0 && new_x < self.width && new_y >= 0 && new_y < self.height && rand::random::<f32>() < probability{
                        let particle = particle_callback(new_x, new_y);
                        if particle.get_id() == EMPTY_ID || self.get((new_x, new_y)).get_id() == EMPTY_ID {
                            self.set((new_x, new_y), particle);
                        }
                    }
                }
            }
        }
    }

    pub fn get(&self, (x, y): Position) -> &Cell {
        &self.cells[(y * self.width + x) as usize]
    }

    pub fn swap(&mut self, (x1, y1): Position, (x2, y2): Position) {
        let a = y1 * self.width + x1;
        let b = y2 * self.width + x2;
        self.cells.swap(a as usize, b as usize);
        self.cells_to_draw.insert((x1, y1));
        self.cells_to_draw.insert((x2, y2));
    }

    pub fn is_empty(&self, (x, y): Position) -> bool {
        self.get((x, y)).get_id() == EMPTY_ID
    }

    pub fn draw(&mut self, canvas: &mut Canvas<Window>) {
        for (x, y) in &self.cells_to_draw {
            let color = self.get((*x, *y)).get_color();
            let rect = sdl2::rect::Rect::new(*x * PIXEL_SIZE, *y * PIXEL_SIZE, PIXEL_SIZE as u32, PIXEL_SIZE as u32);
            canvas.set_draw_color(color);
            canvas.fill_rect(rect).unwrap();
        }
        self.cells_to_draw.clear();
    }

    pub fn update_pixel(&mut self, (x, y): Position) {
        // Gravity
        if !self.is_empty((x, y)) && y + 1 < self.height {
            let down_left = (x - 1, y + 1);
            let down_right = (x + 1, y + 1);
            let down = (x, y + 1);

            if self.is_empty(down) {
                self.swap((x, y), down);
            } else if x - 1 >= 0 && self.is_empty(down_left) {
                self.swap((x, y), down_left);
            } else if x + 1 < self.width && self.is_empty(down_right) {
                self.swap((x, y), down_right);
            }
        }
    }

    pub fn update(&mut self) {
        for y in (0..self.height).rev() {
            for x in 0..self.width {
                self.update_pixel((x, y));
            }
        }
    }
}