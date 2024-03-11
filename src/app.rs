use crate::ui::Ui;

pub struct App {
    width: u32,
    height: u32,
    ui: Ui,

    i: u8,
}

impl App {
    pub fn new() -> App {
        let width = 800;
        let height = 800;
        let title = "Sandgame";

        let ui = Ui::new(width, height, title);
        App {
            width,
            height,

            ui,

            i: 0,
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

            self.ui.refresh();
            self.finish_frame();
        }
    }
    
    pub fn update(&mut self) {
        self.i = (self.i + 1) % 255;
    }

    pub fn draw(&mut self) {
        self.ui.canvas.set_draw_color(sdl2::pixels::Color::RGB(self.i, 64, 255-self.i));
        self.ui.canvas.clear();
    }

    pub fn finish_frame(&mut self) {
        std::thread::sleep(std::time::Duration::from_millis(10));
    }

    pub fn handle_event(&mut self, event: sdl2::event::Event) {
        match event {
            _ => (),
        }
    }
}
