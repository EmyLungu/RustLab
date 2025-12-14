mod controller;
mod room;
mod grid;

use crate::controller::Controller;

fn main() {
    let mut controller = Controller::new();
    controller.run();
}
