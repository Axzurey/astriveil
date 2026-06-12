use glyphon::{Color, Cursor, Metrics, TextArea, TextBounds};
use nalgebra::Vector2;
use wgpu::{RenderPass, util::DeviceExt};
use winit::{event::{ElementState, MouseButton}, keyboard::{Key, NamedKey}};
use getset::{Getters, MutGetters, Setters, WithSetters};
use crate::plantain::{elements::{element::{Border, ElementDesc, ElementDims, InputElement, UiElement, is_point_in_rect}, screen::UiLayer}, render_queue::PipelineRefs, vertices::ui_vertex::UiVertex};

#[derive(Getters, Setters, WithSetters, MutGetters)]
pub struct TextBox {
    #[getset(get = "pub")]
    text: String,
    cursor: usize,
    #[getset(get = "pub", set = "pub", get_mut = "pub")]
    max_text_length: usize,
    pub border: Border,
    pub desc: ElementDesc,
    pub dims: ElementDims
}

impl TextBox {
    pub fn new(text: &str) -> Box<Self> {
        Box::new(Self {
            desc: ElementDesc::default(),
            dims: ElementDims::default(),
            border: Border::default(),
            text: text.to_owned(),
            cursor: text.len(),
            max_text_length: usize::MAX,
        })
    }
}

impl InputElement for TextBox {
    fn key_event(&mut self, event: &winit::event::KeyEvent, is_focused: bool) -> super::element::EventProcessResult {
        if is_focused && event.state == ElementState::Pressed {
            match event.logical_key {
                Key::Named(NamedKey::Backspace) => {
                    if self.cursor > 0 {
                        self.cursor -= 1;
                        self.text.remove(self.cursor);
                    }
                    return super::element::EventProcessResult::Sink;
                },
                Key::Named(NamedKey::ArrowLeft) => {
                    if self.cursor > 0 {
                        self.cursor -= 1;
                    }
                    return super::element::EventProcessResult::Sink;
                },
                Key::Named(NamedKey::ArrowRight) => {
                    if self.cursor < self.text.len() {
                        self.cursor += 1;
                    }
                    return super::element::EventProcessResult::Sink;
                },
                Key::Named(NamedKey::Enter) => {
                    return super::element::EventProcessResult::FocusDrop;
                },
                _ => {
                    if self.text.len() >= self.max_text_length {
                        return super::element::EventProcessResult::Sink;
                    }
                    if let Some(text) = &event.text {
                        let printable: String = text.chars().filter(|c| !c.is_control()).collect();
                        if !printable.is_empty() {
                            self.text.insert_str(self.cursor, &printable);
                            self.cursor += printable.len();
                        }
                        return super::element::EventProcessResult::Sink;
                    }
                }
            }
            super::element::EventProcessResult::Nothing
        }
        else {
            super::element::EventProcessResult::Nothing
        }
    }
    fn mouse_event(&mut self, button: &winit::event::MouseButton, state: &winit::event::ElementState, is_focused: bool, screen_dims: [f32; 2], mouse_pos: &Vector2<f32>) -> super::element::EventProcessResult {
        let abs_pos = self.dims.position().calculate_absolute(screen_dims);
        let abs_size = self.dims.size().calculate_absolute(screen_dims);

        let center = [abs_pos[0] + abs_size[0] / 2.0, abs_pos[1] + abs_size[1] / 2.0];
        
        let inside = is_point_in_rect([mouse_pos.x, mouse_pos.y], center, abs_size, *self.dims.rotation());
        
        if inside && *button == MouseButton::Left && state.is_pressed() {
            println!("INSIDE!");
            super::element::EventProcessResult::Focus
        }
        else if !inside && *button == MouseButton::Left && is_focused && state.is_pressed() { //if out of bounds
            println!("OUTSIDE (FOCUS)");
            super::element::EventProcessResult::FocusDrop
        }
        else {
            println!("OUTSIDE");
            super::element::EventProcessResult::Nothing
        }
    }
}

impl UiElement for TextBox {
    fn as_input_element(&self) -> Option<&dyn InputElement> {
        Some(self)
    }
    fn as_input_element_mut(&mut self) -> Option<&mut dyn InputElement> {
        Some(self)
    }
    fn draw(&self, device: &wgpu::Device, queue: &wgpu::Queue, render_pass: &mut RenderPass, pipeline_refs: &mut PipelineRefs, is_focused: bool) {
    
        let abs_pos = self.dims.position().calculate_absolute(pipeline_refs.screen_dims);
        let abs_size = self.dims.size().calculate_absolute(pipeline_refs.screen_dims);
        
        let (vertices, indices) = UiVertex::create_rect(
            &abs_pos,
            &abs_size,
            &self.desc.background_color(),
            &self.border.color(),
            *self.dims.rotation(),
            &self.border.corner_radius(),
            *self.border.thickness()
        );

        let index_length = indices.len();

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("ui vertex buffer"),
            contents: bytemuck::cast_slice(&vertices),
            usage: wgpu::BufferUsages::VERTEX
        });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("ui index buffer"),
            contents: bytemuck::cast_slice(&indices),
            usage: wgpu::BufferUsages::INDEX
        });

        let mut text_buffer = glyphon::Buffer::new(pipeline_refs.font_system, Metrics::new(42.0, 42.0));
        text_buffer.set_size(pipeline_refs.font_system, Some(abs_size[0]), Some(abs_size[1]));
        text_buffer.set_text(pipeline_refs.font_system, &self.text, &glyphon::Attrs::new().family(glyphon::Family::SansSerif), glyphon::Shaping::Advanced, None);
        
        let text_col = self.desc.color().map(|v| (v * 255.0) as u8);

        let text_area = TextArea {
            buffer: &text_buffer,
            left: abs_pos[0],
            top: abs_pos[1],
            scale: 1.,
            bounds: TextBounds {
                left: abs_pos[0] as i32,
                top: abs_pos[1] as i32,
                right: abs_pos[0] as i32 + abs_size[0] as i32,
                bottom: abs_pos[1] as i32 + abs_size[1] as i32
            },
            default_color: Color::rgba(text_col[0], text_col[1], text_col[2], text_col[3]),
            custom_glyphs: &[]
        };

        pipeline_refs.text_renderer.prepare(device, queue, pipeline_refs.font_system, pipeline_refs.atlas, pipeline_refs.viewport, [text_area], pipeline_refs.swash_cache).unwrap();

        render_pass.set_pipeline(pipeline_refs.color_pipeline);

        render_pass.set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint32);
        render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));

        render_pass.draw_indexed(0..index_length as u32, 0, 0..1);

        if is_focused {
            let (cx, cy) = text_buffer
                .layout_runs()
                .find_map(|run| {
                    run.glyphs.iter().nth(self.cursor).map(|g| (g.x, run.line_y - run.line_height))
                })
                .unwrap_or_else(|| {
                    text_buffer
                        .layout_runs()
                        .last()
                        .map(|run| {
                            let last_glyph = run.glyphs.last();
                            let x = last_glyph.map(|g| g.x + g.w).unwrap_or(0.0);
                            (x, run.line_y - run.line_height)
                        })
                        .unwrap_or((0.0, 0.0))
                });

            let h = text_buffer.metrics().line_height;

            let position = [abs_pos[0] + cx, abs_pos[1] + cy];

            let cursor_thickness = 3.0;

            let size = [cursor_thickness, h];

            let (vertices, indices) = UiVertex::create_rect(
                &position,
                &size,
                &[1., 1., 1., 1.],
                &[1., 1., 1., 1.],
                0.,
                &[0., 0., 0., 0.],
                0.
            );
        
            let index_length = indices.len();

            let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("ui vertex buffer"),
                contents: bytemuck::cast_slice(&vertices),
                usage: wgpu::BufferUsages::VERTEX
            });

            let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("ui index buffer"),
                contents: bytemuck::cast_slice(&indices),
                usage: wgpu::BufferUsages::INDEX
            });

            render_pass.set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint32);
            render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));

            render_pass.draw_indexed(0..index_length as u32, 0, 0..1);
        }
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
    fn get_layer(&self) -> &UiLayer {
        &self.dims.layer()
    }
    fn get_zindex(&self) -> u32 {
        *self.dims.zindex()
    }
    fn set_layer(&mut self, layer: UiLayer) {
        self.dims.set_layer(layer);
    }
    fn set_zindex(&mut self, zindex: u32) {
        self.dims.set_zindex(zindex);
    }
}