use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::pixels::Color;
use sdl2::gfx::primitives::DrawRenderer; 


pub struct Grid {
    pub width: u32,
    pub height: u32,
    pub cells: Vec<Vec<u32>>,
}

impl Grid {
    pub fn new(width: u32, height: u32) -> Grid {
        let cells = vec![vec![0; height as usize]; width as usize];
        Grid {
            width,
            height,
            cells,
        }
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>) {
        for x in 0..self.width {
            for y in 0..self.height {
                let cell = self.cells[x as usize][y as usize];
                let color = Color::RGB(cell as u8, cell as u8, cell as u8);
                canvas.pixel(x as i16, y as i16, color).unwrap();
            }
        }
    }
}