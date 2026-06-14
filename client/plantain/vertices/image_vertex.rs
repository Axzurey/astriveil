use std::mem;

use nalgebra::Vector2;

use shared::loaders::vertex::Vertex;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ImageVertex {
    pub color_mask: [f32; 4],
    pub corner_radius: [f32; 4],
    pub position: [f32; 2],
    pub center: [f32; 2],
    pub size: [f32; 2],
    pub tex_coords: [f32; 2],
    pub image_index: u32,
    pub rotation: f32,
}

impl ImageVertex {
    //creates an array of vertices, as well as the triangles
    pub fn create_rect(
        top_left: &Vector2<f32>, 
        bottom_right: &Vector2<f32>, 
        color: &[f32; 4], 
        rotation: f32, 
        corner_radius: &[f32; 4], 
        image_index: u32
    ) -> (Vec<ImageVertex>, Vec<u32>) {
        (
            vec![
                ImageVertex {
                    color_mask: color.clone(),
                    position: [top_left.x, top_left.y],
                    tex_coords: [0., 0.],
                    image_index,
                    rotation,
                    corner_radius: corner_radius.clone(),
                    size: (bottom_right - top_left).into(),
                    center: (top_left + (bottom_right - top_left) / 2.0).into()
                },
                ImageVertex {
                    color_mask: color.clone(),
                    position: [top_left.x, bottom_right.y],
                    tex_coords: [0., 1.],
                    image_index,
                    rotation,
                    corner_radius: corner_radius.clone(),
                    size: (bottom_right - top_left).into(),
                    center: (top_left + (bottom_right - top_left) / 2.0).into()
                },
                ImageVertex {
                    color_mask: color.clone(),
                    position: [bottom_right.x, bottom_right.y],
                    tex_coords: [1., 1.],
                    image_index,
                    rotation,
                    corner_radius: corner_radius.clone(),
                    size: (bottom_right - top_left).into(),
                    center: (top_left + (bottom_right - top_left) / 2.0).into()
                },
                ImageVertex {
                    color_mask: color.clone(),
                    position: [bottom_right.x, top_left.y],
                    tex_coords: [1., 0.],
                    image_index,
                    rotation,
                    corner_radius: corner_radius.clone(),
                    size: (bottom_right - top_left).into(),
                    center: (top_left + (bottom_right - top_left) / 2.0).into()
                }
            ],
            vec![0, 1, 2, 2, 3, 0]
        )
    }
}

impl Vertex for ImageVertex {
    fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<ImageVertex>() as wgpu::BufferAddress,
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
                    format: wgpu::VertexFormat::Float32x2,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 10]>() as u64,
                    shader_location: 3,
                    format: wgpu::VertexFormat::Float32x2,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 12]>() as u64,
                    shader_location: 4,
                    format: wgpu::VertexFormat::Float32x2,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 14]>() as u64,
                    shader_location: 5,
                    format: wgpu::VertexFormat::Float32x2,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 16]>() as u64,
                    shader_location: 6,
                    format: wgpu::VertexFormat::Uint32,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 17]>() as u64,
                    shader_location: 7,
                    format: wgpu::VertexFormat::Float32,
                }
            ]
        }
    }
}