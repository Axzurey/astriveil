use std::{sync::Arc, time::UNIX_EPOCH};

use instant::Instant;
use nalgebra::Vector2;
use pollster::FutureExt;
use winit::{application::ApplicationHandler, dpi::{PhysicalSize, Size}, event::WindowEvent, window::{Window, WindowAttributes}};

use crate::gamecontroller::{self, GameController};

pub struct ScreenDisplay<'a> {
    pub window: Option<Arc<Window>>,
    pub gamecontroller: Option<GameController<'a>>,
    pub last_frame: Instant,
}

impl<'a> Default for ScreenDisplay<'a> {
    fn default() -> Self {
        Self {
            window: None,
            gamecontroller: None,
            last_frame: Instant::now()
        }
    }
}

impl<'a> ApplicationHandler for ScreenDisplay<'a> {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let mut win_attr = WindowAttributes::default();
        win_attr = win_attr.with_inner_size(Size::Physical(PhysicalSize::new(1280, 720)));

        self.window = Some(Arc::new(event_loop.create_window(win_attr).unwrap()));

        let win = self.window.as_ref().unwrap().clone();
    
        self.gamecontroller = Some(GameController::new(win.inner_size(), win).block_on());
    }

    fn window_event(
            &mut self,
            event_loop: &winit::event_loop::ActiveEventLoop,
            window_id: winit::window::WindowId,
            eventm: winit::event::WindowEvent,
        ) {
        match eventm.clone() {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            },
            WindowEvent::Resized(size) => {
                self.gamecontroller.as_mut().unwrap().on_window_resized(size);
            },
            WindowEvent::RedrawRequested => {
                //todo: change to actually use the delta
                let dt = Instant::now().duration_since(self.last_frame).as_secs_f32();
                self.last_frame = Instant::now();
                
                self.gamecontroller.as_mut().unwrap().on_window_update(dt, self.window.clone().unwrap());
                self.window.as_ref().unwrap().request_redraw();
            },
            WindowEvent::MouseInput { device_id, state, button } => {
                let gamecontroller = self.gamecontroller.as_mut().unwrap();
                let response = gamecontroller.egui_state.on_window_event(
                    self.window.as_ref().unwrap(),
                    &eventm
                );
                if response.consumed {
                    return;
                }
                gamecontroller.on_window_mouse_event(button, state);
            }
            WindowEvent::KeyboardInput { device_id, event, is_synthetic } => {
                let gamecontroller = self.gamecontroller.as_mut().unwrap();
                let response = gamecontroller.egui_state.on_window_event(
                    self.window.as_ref().unwrap(),
                    &eventm
                );
                if response.consumed {
                    return;
                }
                gamecontroller.on_window_key_press(event);
            },
            WindowEvent::CursorMoved { device_id, position } => {
                let gamecontroller = self.gamecontroller.as_mut().unwrap();
                let response = gamecontroller.egui_state.on_window_event(
                    self.window.as_ref().unwrap(),
                    &eventm
                );
                if response.consumed {
                    return;
                }
                gamecontroller.set_mouse_position(Vector2::new(position.x as f32, position.y as f32));
            },
            | WindowEvent::MouseWheel { .. }
            | WindowEvent::Touch { .. } => {
                let gc = self.gamecontroller.as_mut().unwrap();
                let response = gc.egui_state.on_window_event(
                    self.window.as_ref().unwrap(),
                    &eventm
                );
                if response.consumed {
                    return;
                }
            }
            _ => ()
        }
    }

    fn device_event(
            &mut self,
            event_loop: &winit::event_loop::ActiveEventLoop,
            device_id: winit::event::DeviceId,
            event: winit::event::DeviceEvent,
        ) {
        match event {
            winit::event::DeviceEvent::MouseMotion { delta } => {
                let gamecontroller = self.gamecontroller.as_mut().unwrap();
                gamecontroller.on_window_mouse_motion(delta.0, delta.1);
            },
            _ => {}
        }
    }
}