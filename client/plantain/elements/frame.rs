use nalgebra::Vector2;
use wgpu::{RenderPass, util::DeviceExt};

use crate::plantain::{elements::{element::{InputElement, UiElement}, screen::UiLayer}, render_queue::PipelineRefs, vertices::ui_vertex::UiVertex};

pub struct Frame {
    //top left for now, but can make align modes
    position: Vector2<f32>,
    size: Vector2<f32>,
    color: [f32; 4],
    rotation: f32,
    layer: UiLayer,
    zindex: u32
}

impl Frame {
    pub fn new() -> Box<Self> {
        Box::new(Self {
            position: Vector2::new(450., 250.),
            size: Vector2::new(450., 400.),
            color: [0., 1., 1., 1.],
            rotation: 0.0,
            layer: UiLayer::Background,
            zindex: 0
        })
    }

    pub fn set_rotation(&mut self, rotation: f32) {
        self.rotation = rotation;
    }
    pub fn get_rotation(&self) -> f32 {
        self.rotation
    }
}

impl UiElement for Frame {
    fn as_input_element(&self) -> Option<&dyn InputElement> {
        None
    }
    fn as_input_element_mut(&mut self) -> Option<&mut dyn InputElement> {
        None
    }
    fn draw(&mut self, device: &wgpu::Device, queue: &wgpu::Queue, render_pass: &mut RenderPass, pipeline_refs: &mut PipelineRefs, is_focused: bool) {
        // let (vertices, indices) = UiVertex::create_rect(&self.position, &(self.position + self.size), &self.color, &[1., 1., 1., 1.], self.rotation, &[12., 12., 12., 12.]);

        // let index_length = indices.len();

        // let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        //     label: Some("ui vertex buffer"),
        //     contents: bytemuck::cast_slice(&vertices),
        //     usage: wgpu::BufferUsages::VERTEX
        // });

        // let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        //     label: Some("ui index buffer"),
        //     contents: bytemuck::cast_slice(&indices),
        //     usage: wgpu::BufferUsages::INDEX
        // });

        // render_pass.set_pipeline(pipeline_refs.color_pipeline);

        // render_pass.set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint32);
        // render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));

        // render_pass.draw_indexed(0..index_length as u32, 0, 0..1);
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
    fn get_layer(&self) -> &UiLayer {
        &self.layer
    }
    fn get_zindex(&self) -> u32 {
        self.zindex
    }
    fn set_layer(&mut self, layer: UiLayer) {
        self.layer = layer;
    }
    fn set_zindex(&mut self, zindex: u32) {
        self.zindex = zindex;
    }
}