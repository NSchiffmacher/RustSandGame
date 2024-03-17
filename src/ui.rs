extern crate sdl2;

use sdl2::event::Event;
use sdl2::gfx::framerate::FPSManager;
use sdl2::keyboard::Keycode;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::EventPump; // , Sdl, VideoSubsystem};

pub struct Ui {
    pub canvas: Canvas<Window>,
    // sdl_context: Sdl,
    // video_subsystem: VideoSubsystem,
    event_pump: EventPump,
    fps_manager: FPSManager,
    fps: u32,
    running: bool,
}

impl Ui {
    pub fn new(width: u32, height: u32, title: &str, fps_target: u32) -> Ui{
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
    
        let window = video_subsystem.window(title, width, height)
            .position_centered()
            .build()
            .unwrap();
    
        let canvas = window.into_canvas().build().unwrap();
        let event_pump = sdl_context.event_pump().unwrap();
        let mut fps_manager = FPSManager::new();
        fps_manager.set_framerate(fps_target).unwrap();
        Ui {
            canvas,
            // sdl_context,
            // video_subsystem,
            fps_manager,
            event_pump,
            fps: 0,
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

    pub fn finish_frame(&mut self) {
        self.canvas.present();
        self.fps = self.fps_manager.delay();
    }

    pub fn get_fps(&self) -> u32 {
        self.fps
    }
}