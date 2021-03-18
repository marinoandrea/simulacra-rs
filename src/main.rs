extern crate gl;
use application::Application;

mod application;
mod events;
mod input;
mod layers;
mod logger;
mod renderer;
mod window;

fn main() {
    let mut app = Application::new();
    app.init();
    app.run();
}
