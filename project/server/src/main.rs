mod controller;
mod grid;
mod room;

use crate::controller::Controller;

fn main() {
    let mut controller = Controller::new();
    controller.run();
}
