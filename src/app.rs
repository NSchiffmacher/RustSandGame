use crate::ui::Ui;
use crate::sandsim::grid::Grid;

use sdl2::event::Event;
use rand::Rng;

pub struct App {
    width: u32,
    height: u32,
    ui: Ui,
    
    grid: Grid,
}

impl App {
    pub fn new() -> App {
        let width = 800;
        let height = 800;
        let title = "Sandgame";
        let fps_target = 60; //TODO Create our own FPS manager because the one from SDL is not working

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
        // let mut rng = rand::thread_rng();
        // let z = rng.gen_range(0..5);
        // if z == 0 {
        //     let x = rng.gen_range(0..self.width);
        //     let y = rng.gen_range(0..self.height);
        //     let r = rng.gen_range(0..255);
        //     let g = rng.gen_range(0..255);
        //     let b = rng.gen_range(0..255);
        //     self.grid.set(x, y, Some((r, g, b, 255).into()));
        // }
    }

    pub fn draw(&mut self) {
        self.grid.draw(&mut self.ui.canvas);
    }

    pub fn handle_event(&mut self, event: Event) {
        match event {
            // Event::MouseButtonDown { timestamp, window_id, which, mouse_btn, clicks, x, y }
            _ => (),
        }
    }
}
