use std::thread::spawn;

use pollster::FutureExt;

use crate::network::network::NetworkController;

pub struct GameController {
    network_controller: NetworkController  
}

impl GameController {
    pub fn new() -> Self {
        let network_controller = NetworkController::new().block_on();

        Self {
            network_controller
        }
    }
    pub fn update(&mut self) {
        self.network_controller.update();
    }
}