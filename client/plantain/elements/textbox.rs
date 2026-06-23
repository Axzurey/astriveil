use glyphon::{Color, Edit, Editor, FontSystem, Metrics, TextArea, TextBounds, cosmic_text::{Align, BufferRef, Motion, Scroll}};
use nalgebra::Vector2;
use wgpu::{RenderPass, util::DeviceExt};
use winit::{event::{ElementState, MouseButton}, keyboard::{Key, NamedKey}};
use getset::{Getters, MutGetters, Setters, WithSetters};
use crate::plantain::{elements::{element::{Border, ElementDesc, ElementDims, InputElement, Timer, UiElement, is_point_in_rect}, screen::{self, UiLayer}}, render_queue::PipelineRefs, vertices::ui_vertex::UiVertex};

#[derive(Getters, Setters, WithSetters, MutGetters)]
pub struct TextBox {
    #[getset(get = "pub", set = "pub", get_mut = "pub")]
    max_text_length: usize,
    #[getset(get = "pub", set = "pub", get_mut = "pub")]
    text_size: f32,
    #[getset(get = "pub", set = "pub", get_mut = "pub")]
    line_height: f32,
    pub border: Border,
    pub desc: ElementDesc,
    pub dims: ElementDims,

    blink_on: bool,
    blink_timer: Timer,
    #[getset(get = "pub", set = "pub", get_mut = "pub")]
    text_align_x: Align,
    #[getset(get = "pub", set = "pub", get_mut = "pub")]
    text_align_y: Align,
    //if only i knew about editor earlier. it would've saved me hours upon hours of pain
    pub editor: Editor<'static>
}

impl TextBox {
    pub fn new(font_system: &mut FontSystem) -> Box<Self> {
        let buffer = glyphon::Buffer::new(font_system, Metrics::new(12., 14.));
        Box::new(Self {
            desc: ElementDesc::default(),
            dims: ElementDims::default(),
            border: Border::default(),
            max_text_length: usize::MAX,
            text_size: 12.,
            line_height: 14.,
            blink_on: false,
            blink_timer: Timer::new(500., true),
            text_align_y: Align::Center,
            text_align_x: Align::Left,
            editor: Editor::new(buffer)
        })
    }

    fn create_gpu_rect(device: &wgpu::Device, position: &[f32; 2], size: &[f32; 2], color: &[f32; 4], border_color: &[f32; 4], rotation: f32, corners: &[f32; 4], thickness: f32) -> (wgpu::Buffer, wgpu::Buffer, usize) {
        let (vertices, indices) = UiVertex::create_rect(position, size, color, border_color, rotation, corners, thickness);
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
        (vertex_buffer, index_buffer, index_length)
    }

    // set buffers and draw a rect
    fn draw_rect(render_pass: &mut RenderPass, vertex_buffer: &wgpu::Buffer, index_buffer: &wgpu::Buffer, index_length: usize) {
        render_pass.set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint32);
        render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
        render_pass.draw_indexed(0..index_length as u32, 0, 0..1);
    }
}

impl InputElement for TextBox {
    fn key_event(&mut self, event: &winit::event::KeyEvent, is_focused: bool, font_system: &mut FontSystem) -> super::element::EventProcessResult {
        if !is_focused || event.state != ElementState::Pressed {
            return super::element::EventProcessResult::Nothing;
        }

        match event.logical_key {
            Key::Named(NamedKey::Backspace) => {
                self.editor.action(font_system, glyphon::Action::Backspace);
                super::element::EventProcessResult::Sink
            },
            Key::Named(NamedKey::ArrowLeft) => {
                self.editor.action(font_system, glyphon::Action::Motion(Motion::Left));
                super::element::EventProcessResult::Sink
            },
            Key::Named(NamedKey::ArrowRight) => {
                self.editor.action(font_system, glyphon::Action::Motion(Motion::Right));
                super::element::EventProcessResult::Sink
            },
            Key::Named(NamedKey::Enter) => {
                super::element::EventProcessResult::FocusDrop
            },
            _ => {
                if let Some(text) = &event.text {
                    let printable: String = text.chars().filter(|c| !c.is_control()).collect();
                    for ch in printable.chars() {
                        self.editor.action(font_system, glyphon::Action::Insert(ch));
                        
                    }
                    return super::element::EventProcessResult::Sink;
                }
                super::element::EventProcessResult::Nothing
            }
        }
    }


    fn mouse_event(&mut self, button: &winit::event::MouseButton, state: &winit::event::ElementState, is_focused: bool, screen_dims: [f32; 2], mouse_pos: &Vector2<f32>) -> super::element::EventProcessResult {
        let abs_pos = self.dims.position().calculate_absolute(screen_dims);
        let abs_size = self.dims.size().calculate_absolute(screen_dims);

        let expanded_size = [abs_size[0] + self.border.thickness() * 2.0, abs_size[1] + self.border.thickness() * 2.0];
        let center = [abs_pos[0] + abs_size[0] / 2.0, abs_pos[1] + abs_size[1] / 2.0];

        let inside = is_point_in_rect(&[mouse_pos.x, mouse_pos.y], &center, &expanded_size, *self.dims.rotation(), self.border.corner_radius());

        if inside && *button == MouseButton::Left && state.is_pressed() {
            super::element::EventProcessResult::Focus
        } else if !inside && *button == MouseButton::Left && is_focused && state.is_pressed() {
            super::element::EventProcessResult::FocusDrop
        } else {
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

    fn draw(&mut self, device: &wgpu::Device, queue: &wgpu::Queue, render_pass: &mut RenderPass, pipeline_refs: &mut PipelineRefs, is_focused: bool) {
        let abs_pos = self.dims.position().calculate_absolute(pipeline_refs.screen_dims);
        let abs_size = self.dims.size().calculate_absolute(pipeline_refs.screen_dims);

        // draw background rect
        let (vb, ib, il) = Self::create_gpu_rect(
            device, &abs_pos, &abs_size,
            &self.desc.background_color(), &self.border.color(),
            *self.dims.rotation(), &self.border.corner_radius(), *self.border.thickness()
        );

        let (text_start, text_end) = match self.text_align_y {
            Align::Center => (
                [abs_pos[0], abs_pos[1] + ((abs_size[1] - self.text_size) / 2.0)],
                [abs_pos[0] + abs_size[0], abs_pos[1] + abs_size[1] - ((abs_size[1] - self.text_size) / 2.0)]
            ),
            _ => (abs_pos, [abs_pos[0] + abs_size[0], abs_pos[1] + abs_size[1]])
        };

        let text_col = self.desc.color().map(|v| (v * 255.0) as u8);
        self.editor.shape_as_needed(pipeline_refs.font_system, false);
        let buff = match self.editor.buffer_ref_mut() {
            BufferRef::Arc(_) => unreachable!(), //it is not.
            BufferRef::Borrowed(_) => unreachable!(),
            BufferRef::Owned(u) => u 
        };

        buff.set_size(pipeline_refs.font_system, Some(abs_size[0]), Some(abs_size[1]));
        buff.set_metrics(pipeline_refs.font_system, Metrics::new(self.text_size, self.line_height));

        let text_area = TextArea {
            buffer: buff,
            left: text_start[0],
            top: text_start[1],
            scale: 1.,
            bounds: TextBounds {
                left: text_start[0] as i32,
                top: text_start[1] as i32,
                right: text_end[0] as i32,
                bottom: text_end[1] as i32
            },
            default_color: Color::rgba(text_col[0], text_col[1], text_col[2], text_col[3]),
            custom_glyphs: &[]
        };

        pipeline_refs.text_renderer.prepare(device, queue, pipeline_refs.font_system, pipeline_refs.atlas, pipeline_refs.viewport, [text_area], pipeline_refs.swash_cache).unwrap();

        render_pass.set_pipeline(pipeline_refs.color_pipeline);
        Self::draw_rect(render_pass, &vb, &ib, il);

        // blink the cursor
        if self.blink_timer.check() {
            self.blink_on = !self.blink_on;
        }

        // if is_focused && self.blink_on {
        //     let (cx, cy, cyb) = self.get_cursor_pos(pipeline_refs.font_system, abs_size);

        //     // offset cursor x by scroll_offset to match scrolled text
        //     let position = [
        //         abs_pos[0] + cx, 
        //         abs_pos[1] + cy + self.border.thickness()
        //     ];
        //     let size = [3.0, cyb - cy];

        //     let (vb, ib, il) = Self::create_gpu_rect(
        //         device, &position, &size,
        //         &[1., 1., 1., 1.], &[1., 1., 1., 1.],
        //         0., &[0., 0., 0., 0.], 0.
        //     );
        //     Self::draw_rect(render_pass, &vb, &ib, il);
        // }
    }

    fn as_any(&self) -> &dyn std::any::Any { self }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any { self }
    fn get_layer(&self) -> &UiLayer { &self.dims.layer() }
    fn get_zindex(&self) -> u32 { *self.dims.zindex() }
    fn set_layer(&mut self, layer: UiLayer) { self.dims.set_layer(layer); }
    fn set_zindex(&mut self, zindex: u32) { self.dims.set_zindex(zindex); }
}