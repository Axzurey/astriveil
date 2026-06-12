use std::{thread::sleep, time::Duration};

use crate::game_controller::GameController;


mod network;
mod game_controller;
#[tokio::main]
pub async fn main() {
    let mut controller = GameController::new();
    loop {
        sleep(Duration::from_millis(1));
        controller.update();
    }
}