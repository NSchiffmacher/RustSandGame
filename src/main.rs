mod ui;
mod app;
mod sandsim;

use app::App;

pub fn main() {
    let mut app = App::new();
    app.run();
}