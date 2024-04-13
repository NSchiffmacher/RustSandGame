use sdl2::render::Canvas;
use sdl2::video::Window;

use std::collections::HashSet;

use crate::sandsim::particle::*;
use crate::sandsim::brush_settings::BrushSettings;

pub type Position = (i32, i32);
pub const PIXEL_SIZE: i32 = 5;

pub struct Grid {
    pub width: i32,
    pub height: i32,
    pub cells: Vec<Particle>,
    pub cells_to_draw: HashSet<(i32, i32)>,
}

#[allow(dead_code)]
impl Grid {
    pub fn new(window_width: i32, window_height: i32) -> Grid {
        let width = window_width / PIXEL_SIZE;
        let height = window_height / PIXEL_SIZE;
        let cells = std::iter::repeat_with(|| Particle::new_empty()).take((width * height) as usize).collect();
        Grid {
            width,
            height,
            cells,
            cells_to_draw: HashSet::new(),
        }
    }

    pub fn clear(&mut self) {
        self.cells = std::iter::repeat_with(|| Particle::new_empty()).take((self.width * self.height) as usize).collect();
        for y in 0..self.height/PIXEL_SIZE {
            for x in 0..self.width/PIXEL_SIZE {
                self.cells_to_draw.insert((x, y));
            }
        }
    }

    pub fn set(&mut self, (x, y): Position, value: Particle) {
        if y >= self.height || x >= self.width || y < 0 || x < 0{
            return;
        }

        self.cells[(y * self.width + x) as usize] = value;
        self.cells_to_draw.insert((x, y));
    }

    pub fn set_circle(&mut self, (x, y): Position, brush_settings: &BrushSettings) {
        for i in -brush_settings.size..=brush_settings.size {
            for j in -brush_settings.size..=brush_settings.size {
                if i * i + j * j <= brush_settings.size * brush_settings.size {
                    let new_x = x + i;
                    let new_y = y + j;
                    if new_x >= 0 && new_x < self.width && new_y >= 0 && new_y < self.height && rand::random::<f32>() < brush_settings.probability {
                        let particle = (brush_settings.callback)((new_x, new_y));
                        if particle.get_id() == EMPTY_ID || self.get((new_x, new_y)).get_id() == EMPTY_ID {
                            self.set((new_x, new_y), particle);
                        }
                    }
                }
            }
        }
    }

    pub fn get(&mut self, (x, y): Position) -> &mut Particle {
        &mut self.cells[(y * self.width + x) as usize]
    }

    pub fn swap(&mut self, (x1, y1): Position, (x2, y2): Position) {
        if self.is_empty((x1, y1)) && self.is_empty((x2, y2)) { return; }

        let a = y1 * self.width + x1;
        let b = y2 * self.width + x2;
        self.cells.swap(a as usize, b as usize);
        self.cells_to_draw.insert((x1, y1));
        self.cells_to_draw.insert((x2, y2));
    }

    pub fn is_empty(&mut self, (x, y): Position) -> bool {
        self.get((x, y)).get_id() == EMPTY_ID
    }

    pub fn draw(&mut self, canvas: &mut Canvas<Window>) {
        let pos: Vec<_> = self.cells_to_draw.iter().cloned().collect();
        for (x, y) in pos {
            let color = self.get((x, y)).get_color();
            let rect = sdl2::rect::Rect::new(x * PIXEL_SIZE, y * PIXEL_SIZE, PIXEL_SIZE as u32, PIXEL_SIZE as u32);
            canvas.set_draw_color(color);
            canvas.fill_rect(rect).unwrap();
        }
        self.cells_to_draw.clear();
    }

    pub fn update_pixel(&mut self, (x, y): Position) -> Position {
        if self.is_empty((x, y)) { return (x, y); }

        if y + 1 < self.height {
            let down_left = (x - 1, y + 1);
            let down_right = (x + 1, y + 1);
            let down = (x, y + 1);

            if self.is_empty(down) {
                self.swap((x, y), down);
                return down;
            } else if x - 1 >= 0 && self.is_empty(down_left) {
                self.swap((x, y), down_left);
                return down_left;
            } else if x + 1 < self.width && self.is_empty(down_right) {
                self.swap((x, y), down_right);
                return down_right;
            }
        }

        return (x, y);
    }

    pub fn update(&mut self) {
        for y in (0..self.height).rev() {
            let (mut cur, step) = {
                if rand::random::<f32>() < 0.5 {
                    (0, 1)
                } else {
                    (self.width - 1, -1)
                }
            };

            while cur >= 0 && cur < self.width {
                let particle = self.get((cur, y));
                particle.update();
                if !particle.was_modified() {
                    cur += step;
                    continue;
                }

                let mut index = (cur, y);
                for _ in 0..particle.get_update_count() {
                    let new_index = self.update_pixel(index);
                    if new_index != index {
                        index = new_index;
                    } else {
                        // Did not move because of a collision
                        self.get(index).reset_velocity();
                    }
                }
                
                cur += step;
            }
        }
    }
}