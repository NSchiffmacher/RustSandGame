use crate::ui::Ui;
use crate::sandsim::grid::{Grid, PIXEL_SIZE, SAND_CELL_COLOR};
use crate::color;

use sdl2::event::Event;

pub struct App {
    width: i32,
    height: i32,
    ui: Ui,
    
    grid: Grid,
}

impl App {
    pub fn new() -> App {
        let width = 800;
        let height = 800;
        let title = "Sandgame";
        let fps_target = 200; //TODO Create our own FPS manager because the one from SDL is not working

        let ui = Ui::new(width, height, title, fps_target); 
        App {
            width,
            height,
            ui,

            grid: Grid::new(width, height),
        }
    }

    pub fn run(&mut self) {
        while !self.ui.requested_app_closing(){
            self.ui.clear();
            for event in self.ui.get_events() {
                self.handle_event(event);
            }

            self.update();
            self.draw();

            self.ui.finish_frame();
        }
    }
    
    pub fn update(&mut self) {
        // Inputs
        let mouse_state = self.ui.event_pump.mouse_state();
        if mouse_state.left() {
            let x = mouse_state.x();
            let y = mouse_state.y();

            let grid_x = x / PIXEL_SIZE;
            let grid_y = y / PIXEL_SIZE;

            let color = color::vary_color(SAND_CELL_COLOR, 10);

            if self.grid.is_empty(grid_x, grid_y) {
                self.grid.set(grid_x, grid_y, Some(color));
            }
        }

        // Logic
    }

    pub fn draw(&mut self) {
        self.grid.draw(&mut self.ui.canvas);
    }

    pub fn handle_event(&mut self, event: Event) {
        match event {
            _ => (),
        }
    }
}
