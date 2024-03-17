mod ui;
mod app;
mod sandsim;
mod color;

use app::App;

pub fn main() {
    let mut app = App::new();
    app.run();
}