use winit::event_loop::EventLoop;

use crate::screendisplay::ScreenDisplay;

mod meshing;
mod renderer;
mod nominal;
mod screendisplay;
mod gamecontroller;
mod gameloop;
mod network;
mod plantain;

#[tokio::main]
pub async fn main() {
    let event_loop = EventLoop::new().unwrap();

    event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);

    let mut display = ScreenDisplay::default();

    event_loop.run_app(&mut display).unwrap();
}