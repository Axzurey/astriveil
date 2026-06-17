use glyphon::FontSystem;
use instant::Instant;
use nalgebra::Vector2;
use wgpu::RenderPass;
use winit::event::{ElementState, KeyEvent, MouseButton};
use getset::{Getters, MutGetters, Setters, WithSetters};
use crate::plantain::{elements::screen::UiLayer, render_queue::PipelineRefs, vertices::ui_vertex::UiVertex};
use std::any::Any;

pub struct Timer {
    //duration in ms
    duration: f64,
    last_tick: f64
}

impl Timer {
    pub fn new(duration_millis: f64, from_now: bool) -> Self {
        Self {
            duration: duration_millis,
            last_tick: if from_now {instant::now()} else {0.}
        }
    }
    /*Tells you whether or not the period has elapsed. */
    pub fn check(&mut self) -> bool {
        let now = instant::now();
        let diff = now - self.last_tick;

        if diff > self.duration {
            self.last_tick = now;
            true
        }
        else {
            false
        }
    }
}

pub enum AlignModeX {
    Center,
    Right,
    Left
}

pub enum AlignModeY {
    Center,
    Top,
    Bottom
}

pub trait UiElement {
    fn draw(&mut self, device: &wgpu::Device, queue: &wgpu::Queue, render_pass: &mut RenderPass, pipeline_refs: &mut PipelineRefs, is_focused: bool);
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    
    fn get_layer(&self) -> &UiLayer;
    fn get_zindex(&self) -> u32;

    fn set_layer(&mut self, layer: UiLayer);
    fn set_zindex(&mut self, zindex: u32);
    fn as_input_element(&self) -> Option<&dyn InputElement>;
    fn as_input_element_mut(&mut self) -> Option<&mut dyn InputElement>;
}

#[derive(Debug)]
pub enum EventProcessResult {
    Nothing,
    Sink,
    Focus,
    FocusDrop
}

pub trait InputElement: UiElement {
    fn key_event(&mut self, event: &KeyEvent, is_focused: bool, font_system: &mut FontSystem) -> EventProcessResult;
    fn mouse_event(&mut self, button: &MouseButton, state: &ElementState, is_focused: bool, screen_dims: [f32; 2], mouse_pos: &Vector2<f32>) -> EventProcessResult;
}

#[derive(Default)]
pub struct DimD2 {
    pub scale_x: f32,
    pub scale_y: f32,
    pub offset_x: f32,
    pub offset_y: f32
}

impl DimD2 {
    pub fn new(scale_x: f32, scale_y: f32, offset_x: f32, offset_y: f32) -> Self {
        Self {
            scale_x, scale_y, offset_x, offset_y
        }
    }

    pub fn from_scale(scale_x: f32, scale_y: f32) -> Self {
        Self {
            scale_x, scale_y, ..Default::default()
        }
    }

    pub fn from_offset(offset_x: f32, offset_y: f32) -> Self {
        Self {
            offset_x, offset_y, ..Default::default()
        }
    }

    pub fn calculate_absolute(&self, reference: [f32; 2]) -> [f32; 2] {
        [self.scale_x * reference[0] + self.offset_x, self.scale_y * reference[1] + self.offset_y]
    }
}

#[derive(Getters, Setters, WithSetters, MutGetters)]
pub struct ElementDims {
    #[getset(get = "pub", set = "pub", get_mut = "pub")]
    position: DimD2,
    #[getset(get = "pub", set = "pub", get_mut = "pub")]
    size: DimD2,
    #[getset(get = "pub", set = "pub", get_mut = "pub")]
    rotation: f32,
    #[getset(get = "pub", set = "pub", get_mut = "pub")]
    layer: UiLayer,
    #[getset(get = "pub", set = "pub", get_mut = "pub")]
    zindex: u32,
}

impl Default for ElementDims {
    fn default() -> Self {
        Self {
            position: DimD2::default(),
            size: DimD2::from_offset(200., 200.),
            rotation: 0.0,
            layer: UiLayer::Element,
            zindex: 0
        }
    }
}

#[derive(Getters, Setters, WithSetters, MutGetters)]
pub struct ElementDesc {
    #[getset(get = "pub", set = "pub", get_mut = "pub")]
    color: [f32; 4],
    #[getset(get = "pub", set = "pub", get_mut = "pub")]
    background_color: [f32; 4],
}

impl Default for ElementDesc {
    fn default() -> Self {
        Self {
            color: [0.9, 0.9, 0.9, 1.0],
            background_color: [0.1, 0.5, 0.1, 1.0]
        }
    }
}

#[derive(Getters, Setters, WithSetters, MutGetters)]
pub struct Border {
    #[getset(get = "pub", set = "pub", get_mut = "pub")]
    color: [f32; 4],
    #[getset(get = "pub", set = "pub", get_mut = "pub")]
    thickness: f32,
    #[getset(get = "pub", set = "pub", get_mut = "pub")]
    corner_radius: [f32; 4]
}

impl Default for Border {
    fn default() -> Self {
        Self {
            thickness: 5.0,
            color: [0.0, 0.0, 0.0, 1.0],
            corner_radius: [4.0, 4.0, 4.0, 4.0]
        }
    }
}

pub fn is_point_in_circle(point: &[f32; 2], origin: [f32; 2], radius: f32) -> bool {
    ((point[0] - origin[0]).powf(2.0) + (point[1] - origin[1]).powf(2.0)).sqrt() < radius
}

pub fn is_point_in_rect(point: &[f32; 2], center: &[f32; 2], size: &[f32; 2], rotation: f32, corners: &[f32; 4]) -> bool {
    // corners order: top-left, top-right, bottom-right, bottom-left

    let dx = point[0] - center[0];
    let dy = point[1] - center[1];
    let radians = rotation.to_radians();
    let cos = radians.cos();
    let sin = radians.sin();
    let rx = dx * cos + dy * sin;
    let ry = -dx * sin + dy * cos;
    let hw = size[0] / 2.0;
    let hh = size[1] / 2.0;

    //bounding check
    if rx < -hw || rx > hw || ry < -hh || ry > hh {
        return false;
    }

    //watch me start crying

    if rx < 0.0 && ry < 0.0 {
        // top left
        if rx < -hw + corners[0] && ry < -hh + corners[0] {
            return is_point_in_circle(&[rx, ry], [-hw + corners[0], -hh + corners[0]], corners[0]);
        }
    } else if rx >= 0.0 && ry < 0.0 {
        // top right
        if rx > hw - corners[1] && ry < -hh + corners[1] {
            return is_point_in_circle(&[rx, ry], [hw - corners[1], -hh + corners[1]], corners[1]);
        }
    } else if rx >= 0.0 && ry >= 0.0 {
        // bottom right
        if rx > hw - corners[2] && ry > hh - corners[2] {
            return is_point_in_circle(&[rx, ry], [hw - corners[2], hh - corners[2]], corners[2]);
        }
    } else {
        // bottom left
        if rx < -hw + corners[3] && ry > hh - corners[3] {
            return is_point_in_circle(&[rx, ry], [-hw + corners[3], hh - corners[3]], corners[3]);
        }
    }

    true
}