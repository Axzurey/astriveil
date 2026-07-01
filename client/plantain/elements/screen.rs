use std::iter::FilterMap;

use glyphon::FontSystem;
use itertools::Itertools;
use nalgebra::Vector2;
use slab::Slab;
use wgpu::{Buffer, RenderPass, RenderPipeline, util::DeviceExt};
use winit::event::{ElementState, KeyEvent, MouseButton};

use crate::plantain::{elements::element::{EventProcessResult, InputElement, UiElement}, render_queue::PipelineRefs, vertices::ui_vertex::UiVertex};


#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct CameraUniform {
    pub ortho: [[f32; 4]; 4],
}

fn ortho(width: f32, height: f32) -> [[f32; 4]; 4] {
    [
        [2. / width, 0., 0., 0.],
        [0., -2. / height, 0., 0.],
        [0., 0., 1., 0.],
        [-1., 1., 0., 1.]
    ]
}

#[derive(Hash, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Copy)]
pub enum UiLayer {
    Background,
    Menu,
    Widget,
    Element
}

pub struct Screen {
    children: Slab<Box<dyn UiElement>>,
    pub width: u32,
    pub height: u32,
    pub cam_buffer: wgpu::Buffer,
    pub cam_bg: wgpu::BindGroup,
    pub cam_bgl: wgpu::BindGroupLayout,
    pub focused_element: Option<usize>,
    pub hovered_element: Option<usize>
}

impl Screen {
    pub fn new(device: &wgpu::Device, width: u32, height: u32) -> Self {

        let camera_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Camera BGL"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });

        let camera_uniform = CameraUniform {
            ortho: ortho(width as f32, height as f32),
        };

        let cam_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Camera Buffer"),
            contents: bytemuck::cast_slice(&[camera_uniform]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let camera_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Camera BG"),
            layout: &camera_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: cam_buffer.as_entire_binding(),
            }],
        });

        Self {
            children: Slab::new(),
            width,
            height,
            cam_buffer,
            cam_bg: camera_bind_group,
            cam_bgl: camera_bind_group_layout,
            focused_element: None,
            hovered_element: None
        }
    }

    pub fn update_dims(&mut self, queue: &wgpu::Queue, width: u32, height: u32) {
        self.width = width;
        self.height = height;

        let camera_uniform = CameraUniform {
            ortho: ortho(width as f32, height as f32),
        };

        queue.write_buffer(
            &self.cam_buffer,
            0,
            bytemuck::cast_slice(&[camera_uniform])
        );
    }

    pub fn add_child(&mut self, child: Box<dyn UiElement>) -> usize {
        self.children.insert(child)
    }

    pub fn get_child(&mut self, index: usize) -> Option<&Box<dyn UiElement>> {
        self.children.get(index)
    }
    pub fn get_child_mut(&mut self, index: usize) -> Option<&mut Box<dyn UiElement>> {
        self.children.get_mut(index)
    }

    pub fn get_children(&self) -> &Slab<Box<dyn UiElement>> {
        &self.children
    }
    pub fn get_children_mut(&mut self) -> &mut Slab<Box<dyn UiElement>> {
        &mut self.children
    }
    pub fn get_children_of_type<T: UiElement + 'static>(&self) -> impl std::iter::Iterator<Item = &T> {
        self.children.iter().filter_map(|e| e.1.as_any().downcast_ref::<T>())
    }
    pub fn get_children_mut_of_type<T: UiElement + 'static>(&mut self) -> impl std::iter::Iterator<Item = &mut T> {
        self.children.iter_mut().filter_map(|e| e.1.as_any_mut().downcast_mut::<T>())
    }

    pub fn key_event(&mut self, event: &KeyEvent, font_system: &mut FontSystem) -> bool {
        let mut groups = self.children.iter_mut().into_group_map_by(|v| v.1.get_layer().clone());
        for (layer, elements) in groups.iter_mut().sorted_by(|a, b| a.0.cmp(b.0)) {
            let iter = elements.iter_mut()
                .filter_map(|v| {
                    v.1.as_input_element_mut().map(|x| (v.0, x))
                })
                .sorted_by(|a, b| a.1.get_zindex().cmp(&b.1.get_zindex()));
            for (id, item) in iter {
                let is_focused = if let Some(target) = self.focused_element {
                    target == id
                } else {false};
                let res = item.key_event(&event, is_focused, font_system);
                match res {
                    EventProcessResult::Nothing => {},
                    EventProcessResult::Sink => {
                        return true;
                    },
                    EventProcessResult::Focus => {
                        self.focused_element = Some(id);
                        return true;
                    },
                    EventProcessResult::FocusDrop => {
                        if is_focused {
                            self.focused_element = None;
                            return true;
                        }
                    },
                    _ => {}
                }
            }
        }
        false
    }

    pub fn mouse_motion(&mut self, mouse_pos: &Vector2<f32>) -> bool {
        let mut groups = self.children.iter_mut().into_group_map_by(|v| v.1.get_layer().clone());
        for (layer, elements) in groups.iter_mut().sorted_by(|a, b| a.0.cmp(b.0)) {
            let iter = elements.iter_mut()
                .filter_map(|v| {
                    v.1.as_input_element_mut().map(|x| (v.0, x))
                })
                .sorted_by(|a, b| a.1.get_zindex().cmp(&b.1.get_zindex()));
            
            for (id, item) in iter {
                let is_hovered = if let Some(target) = self.hovered_element {
                    target == id
                } else {false};
                let res = item.mouse_move(mouse_pos, [self.width as f32, self.height as f32], is_hovered);
                match res {
                    EventProcessResult::Nothing => {},
                    EventProcessResult::HoverDrop => {
                        if is_hovered {
                            self.hovered_element = None;
                            return true;
                        }
                    },
                    EventProcessResult::HoverTake => {
                        self.hovered_element = Some(id);
                        return true;
                    }
                    _ => {}
                }
            }
        }
        false
    }
    
    pub fn mouse_event(&mut self, button: &MouseButton, state: &ElementState, mouse_pos: &Vector2<f32>) -> bool {
        let mut groups = self.children.iter_mut().into_group_map_by(|v| v.1.get_layer().clone());
        for (layer, elements) in groups.iter_mut().sorted_by(|a, b| a.0.cmp(b.0)) {
            let iter = elements.iter_mut()
                .filter_map(|v| {
                    v.1.as_input_element_mut().map(|x| (v.0, x))
                })
                .sorted_by(|a, b| a.1.get_zindex().cmp(&b.1.get_zindex()));
            
            for (id, item) in iter {
                let is_focused = if let Some(target) = self.focused_element {
                    target == id
                } else {false};
                let res = item.mouse_event(button, state, is_focused, [self.width as f32, self.height as f32], mouse_pos);  
                match res {
                    EventProcessResult::Nothing => {},
                    EventProcessResult::Sink => {
                        return true;
                    },
                    EventProcessResult::Focus => {
                        self.focused_element = Some(id);
                        return true;
                    },
                    EventProcessResult::FocusDrop => {
                        if is_focused {
                            self.focused_element = None;
                            return true;
                        }
                    },
                    _ => {}
                }
            }
        }
        false
    }

    pub fn draw_to_buffers(&mut self, device: &wgpu::Device, queue: &wgpu::Queue, render_pass: &mut RenderPass, pipeline_refs: &mut PipelineRefs) {
        println!("Q START");
        let mut groups = self.children.iter_mut().into_group_map_by(|v| *v.1.get_layer());
        for (layer, elements) in groups.iter_mut().sorted_by(|a, b| a.0.cmp(b.0)) {
            let e = elements.iter_mut()
                .sorted_by(|a, b| a.1.get_zindex().cmp(&b.1.get_zindex()));
            for (id, element) in e {
                let is_focused = if let Some(target) = self.focused_element {
                    target == *id
                } else {false};
                println!("Iter.");
                render_pass.set_bind_group(1, Some(&pipeline_refs.bindgroups.texture_bin.block_texture_bindgroup), &[]);
                render_pass.set_bind_group(0, Some(&self.cam_bg), &[]);
                element.draw(device, queue, render_pass, pipeline_refs, is_focused);
                //TODO i guess we have to accumilate text
            }
        }
        println!("Q END");
    }
}
