extern crate sdl2;

use crate::sandsim::particle::*;
use crate::ui::Ui;
use crate::sandsim::brush_settings::*;
use crate::sandsim::grid::{Grid, PIXEL_SIZE};

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::collections::HashMap;

pub struct App {
    // width: i32,
    // height: i32,
    ui: Ui,
    
    grid: Grid,
    brush_settings_map: HashMap<ParticleId, BrushSettings>,
    selected_brush: ParticleId,
}

impl App {
    pub fn new() -> App {
        let width = 800;
        let height = 800;
        let title = "Sandgame";
        let fps_target = 90; //TODO Create our own FPS manager because the one from SDL is not working

        let ui = Ui::new(width, height, title, fps_target); 
        App {
            // width,
            // height,
            ui,

            grid: Grid::new(width, height),
            brush_settings_map: make_default_brush_settings_map(),
            selected_brush: SAND_ID,
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
            println!("FPS: {}", self.ui.fps);
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

            self.grid.set_circle(
                (grid_x, grid_y),
                self.brush_settings_map.get(&self.selected_brush).unwrap());
        }

        if mouse_state.right() {
            let x = mouse_state.x();
            let y = mouse_state.y();

            let grid_x = x / PIXEL_SIZE;
            let grid_y = y / PIXEL_SIZE;

            self.grid.set_circle((grid_x, grid_y), self.brush_settings_map.get(&EMPTY_ID).unwrap());
        }

        // Logic
        self.grid.update();
    }

    pub fn draw(&mut self) {
        self.grid.draw(&mut self.ui.canvas);
    }

    pub fn handle_event(&mut self, event: Event) {
        match event {
            Event::KeyDown { keycode: Some(Keycode::Num1), .. } => { self.selected_brush = SAND_ID; println!("Selected sand");  },
            Event::KeyDown { keycode: Some(Keycode::Num2), .. } => { self.selected_brush = WOOD_ID; println!("Selected wood");  },
            Event::KeyDown { keycode: Some(Keycode::Num3), .. } => { self.selected_brush = EMPTY_ID; println!("Selected empty"); },
            _ => (),
        }
    }
}
