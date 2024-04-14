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
    pub cells: Vec<Vec<Particle>>,
    // pub cell_types: Vec<Vec<ParticleId>>,
    pub cells_to_draw: HashSet<(i32, i32)>,
}

#[allow(dead_code)]
impl Grid {
    pub fn new(window_width: i32, window_height: i32) -> Grid {
        let width = window_width / PIXEL_SIZE;
        let height = window_height / PIXEL_SIZE;
        let cells = Self::make_grid(width, height, |pos| Particle::new_empty(pos));
        // let cell_types = Self::make_grid(width, height, |_pos| EMPTY_ID);
        Grid {
            width,
            height,
            cells,
            // cell_types,
            cells_to_draw: HashSet::new(),
        }
    }

    fn make_grid<T>(width: i32, height: i32, default: fn(Position) -> T) -> Vec<Vec<T>> {
        let mut res = Vec::with_capacity(height as usize);
        for y in 0..height {
            let mut row = Vec::with_capacity(width as usize);
            for x in 0..width {
                row.push(default((x, y)));
            }
            res.push(row);
        }
        res
    }

    pub fn clear(&mut self) {
        self.cells = Self::make_grid(self.width, self.height, |pos| Particle::new_empty(pos));
        // self.cell_types = Self::make_grid(self.width, self.height, |_pos| EMPTY_ID);
        for y in 0..self.height {
            for x in 0..self.width {
                self.cells_to_draw.insert((x, y));
            }
        }
    }

    pub fn set(&mut self, (x, y): Position, value: Particle) {
        if y >= self.height || x >= self.width || y < 0 || x < 0{
            return;
        }

        // self.cell_types[y as usize][x as usize] = value.get_id();
        self.cells[y as usize][x as usize]= value;
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
        &mut self.cells[y as usize][x as usize]
    }

    // pub fn get_type(&self, (x, y): Position) -> ParticleId {
    //     self.cell_types[y as usize][x as usize]
    // }

    pub fn swap(&mut self, (mut x1, mut y1): Position, (mut x2, mut y2): Position) {
        if self.is_empty((x1, y1)) && self.is_empty((x2, y2)) { return; }

        // Swap the particles
        if y1 == y2 {
            self.cells[y1 as usize].swap(x1 as usize, x2 as usize);
        } else {
            if y2 < y1 {
                (y1, y2) = (y2, y1);
                (x1, x2) = (x2, x1);
            }

            let (a, b) = self.cells.split_at_mut(y2 as usize);
            std::mem::swap(&mut a[y1 as usize][x1 as usize], &mut b[0][x2 as usize]);
        }

        // Swap the types
        // let tmp = self.cell_types[y1 as usize][x1 as usize];
        // self.cell_types[y1 as usize][x1 as usize] = self.cell_types[y2 as usize][x2 as usize];
        // self.cell_types[y2 as usize][x2 as usize] = tmp;

        // Force the redraw on both
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

    pub fn update(&mut self, dt: f64) {
        let mut cell_types = self.build_cell_types();
        let mut cell_behaviors = self.build_cell_behaviors();

        // let mut cell_types = self.cell_types.clone();
        for y in (0..self.height).rev() {
            let (mut x, step) = {
                if rand::random::<f32>() < 0.5 {
                    (0, 1)
                } else {
                    (self.width - 1, -1)
                }
            };

            while x >= 0 && x < self.width {
                // Swaps are relative to the current cell
                let modified = self.get((x, y)).update((x, y), dt, &mut cell_types, &mut cell_behaviors);
                
                if modified {
                    let new_position = self.get((x, y)).get_position();
                    if new_position != (x, y) {
                        self.swap((x, y), new_position);
                    }

                    self.cells_to_draw.insert((x, y));
                } 
                

                x += step;
            }
        }
    }

    fn build_cell_types(&mut self) -> Vec<Vec<ParticleId>> {
        let mut res = vec![vec![EMPTY_ID; self.width as usize]; self.height as usize];
        for y in 0..self.height {
            for x in 0..self.width {
                res[y as usize][x as usize] = self.get((x, y)).get_id();
            }
        }
        res
    }

    fn build_cell_behaviors(&mut self) -> Vec<Vec<ParticleId>> {
        let mut res = vec![vec![0; self.width as usize]; self.height as usize];
        for y in 0..self.height {
            for x in 0..self.width {
                res[y as usize][x as usize] = self.get((x, y)).get_behaviors_ids();
            }
        }
        res
    }
}