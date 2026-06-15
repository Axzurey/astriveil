use glyphon::{Color, Metrics, TextArea, TextBounds};
use nalgebra::Vector2;
use wgpu::{RenderPass, util::DeviceExt};

use crate::plantain::{elements::{element::{InputElement, UiElement}, screen::UiLayer}, render_queue::PipelineRefs, vertices::ui_vertex::UiVertex};

pub struct TextLabel {
    //top left for now, but can make align modes
    position: Vector2<f32>,
    size: Vector2<f32>,
    color: [f32; 4],
    rotation: f32,
    text: String,
    layer: UiLayer,
    zindex: u32
}

impl TextLabel {
    pub fn new(text: &str) -> Box<Self> {
        Box::new(Self {
            position: Vector2::new(450., 250.),
            size: Vector2::new(450., 400.),
            color: [0., 1., 1., 1.],
            rotation: 0.0,
            text: text.to_owned(),
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

impl UiElement for TextLabel {
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

        // let mut text_buffer = glyphon::Buffer::new(pipeline_refs.font_system, Metrics::new(42.0, 20.0));
        // text_buffer.set_size(pipeline_refs.font_system, Some(600.), Some(600.));
        // text_buffer.set_text(pipeline_refs.font_system, &self.text, &glyphon::Attrs::new().family(glyphon::Family::SansSerif), glyphon::Shaping::Advanced, None);
        
        // let text_area = TextArea {
        //     buffer: &text_buffer,
        //     left: self.position.x,
        //     top: self.position.y,
        //     scale: 1.,
        //     bounds: TextBounds {
        //         left: self.position.x as i32,
        //         top: self.position.y as i32,
        //         right: self.position.x as i32 + self.size.x as i32,
        //         bottom: self.position.y as i32 + self.size.y as i32
        //     },
        //     default_color: Color::rgb(18, 18, 19),
        //     custom_glyphs: &[]
        // };

        // pipeline_refs.text_renderer.prepare(device, queue, pipeline_refs.font_system, pipeline_refs.atlas, pipeline_refs.viewport, [text_area], pipeline_refs.swash_cache).unwrap();

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
    fn as_input_element(&self) -> Option<&dyn InputElement> {
        None
    }
    fn as_input_element_mut(&mut self) -> Option<&mut dyn InputElement> {
        None
    }
}