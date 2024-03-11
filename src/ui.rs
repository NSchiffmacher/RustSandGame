extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::{EventPump, Sdl, VideoSubsystem};
use std::time::Duration;

pub struct Ui {
    pub canvas: Canvas<Window>,
    sdl_context: Sdl,
    video_subsystem: VideoSubsystem,
    event_pump: EventPump,
    running: bool,
}

impl Ui {
    pub fn new(width: u32, height: u32, title: &str) -> Ui{
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
    
        let window = video_subsystem.window(title, width, height)
            .position_centered()
            .build()
            .unwrap();
    
        let canvas = window.into_canvas().build().unwrap();
        let event_pump = sdl_context.event_pump().unwrap();
        Ui {
            canvas,
            sdl_context,
            video_subsystem,
            event_pump,
            running: true,
        }
    }

    pub fn clear(&mut self) {
        self.canvas.clear();
    }

    pub fn requested_app_closing(&self) -> bool {
        !self.running
    }

    pub fn get_events(&mut self) -> Vec<Event> {
        let mut events = Vec::new();
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => self.running = false,
                _ => events.push(event),
            };
        }
        events
    }

    pub fn refresh(&mut self) {
        self.canvas.present();
    }
}