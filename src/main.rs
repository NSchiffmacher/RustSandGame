mod ui;
mod app;

use app::App;

pub fn main() {
    let mut app = App::new();
    app.run();
}