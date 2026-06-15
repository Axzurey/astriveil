use std::mem;

use nalgebra::Vector2;
use shared::loaders::vertex::Vertex;
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct UiVertex {
    pub color: [f32; 4],
    pub border_color: [f32; 4],
    pub corner_radius: [f32; 4],
    pub position: [f32; 2],
    pub center: [f32; 2],
    pub size: [f32; 2],
    pub tex_coords: [f32; 2],
    pub rotation: f32,
    pub border_thickness: f32
}

impl UiVertex {
    pub fn create_rect(
        position: &[f32; 2], 
        size: &[f32; 2], 
        color: &[f32; 4], 
        border_color: &[f32; 4], 
        rotation: f32,
        corner_radius: &[f32; 4],
        border_thickness: f32
    ) -> (Vec<UiVertex>, Vec<u32>) {
        let expanded_size = [size[0] + border_thickness * 2.0, size[1] + border_thickness * 2.0];
        let center = [position[0] + size[0] / 2.0, position[1] + size[1] / 2.0];
        (
            vec![
                UiVertex {
                    color: color.clone(),
                    position: [position[0] - border_thickness, position[1] - border_thickness],
                    tex_coords: [0., 0.],
                    border_color: border_color.clone(),
                    rotation: rotation.to_radians(),
                    corner_radius: corner_radius.clone(),
                    size: expanded_size.clone(),
                    center: center.clone(),
                    border_thickness
                },
                UiVertex {
                    color: color.clone(),
                    position: [position[0] - border_thickness, position[1] + size[1] + border_thickness],
                    tex_coords: [0., 1.],
                    border_color: border_color.clone(),
                    rotation: rotation.to_radians(),
                    corner_radius: corner_radius.clone(),
                    size: expanded_size.clone(),
                    center: center.clone(),
                    border_thickness
                },
                UiVertex {
                    color: color.clone(),
                    position: [position[0] + size[0] + border_thickness, position[1] + size[1] + border_thickness],
                    tex_coords: [1., 1.],
                    border_color: border_color.clone(),
                    rotation: rotation.to_radians(),
                    corner_radius: corner_radius.clone(),
                    size: expanded_size.clone(),
                    center: center.clone(),
                    border_thickness
                },
                UiVertex {
                    color: color.clone(),
                    position: [position[0] + size[0] + border_thickness, position[1] - border_thickness],
                    tex_coords: [1., 0.],
                    border_color: border_color.clone(),
                    rotation: rotation.to_radians(),
                    corner_radius: corner_radius.clone(),
                    size: expanded_size.clone(),
                    center: center.clone(),
                    border_thickness
                }
            ],
            vec![0, 1, 2, 2, 3, 0]
        )
    }
}

impl Vertex for UiVertex {
    fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<UiVertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 4]>() as u64,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 8]>() as u64,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 12]>() as u64,
                    shader_location: 3,
                    format: wgpu::VertexFormat::Float32x2,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 14]>() as u64,
                    shader_location: 4,
                    format: wgpu::VertexFormat::Float32x2,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 16]>() as u64,
                    shader_location: 5,
                    format: wgpu::VertexFormat::Float32x2,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 18]>() as u64,
                    shader_location: 6,
                    format: wgpu::VertexFormat::Float32x2,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 20]>() as u64,
                    shader_location: 7,
                    format: wgpu::VertexFormat::Float32,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 21]>() as u64,
                    shader_location: 8,
                    format: wgpu::VertexFormat::Float32,
                }
            ]
        }
    }
}