mod controller;
mod grid;
mod room;

use crate::controller::Controller;

fn main() {
    match Controller::new() {
        Ok(mut controller) => controller.run(),
        Err(e) => eprintln!("Failed starting server: ({})", e),
    }
}
