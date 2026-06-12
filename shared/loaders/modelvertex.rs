use std::mem::{self, offset_of};

use crate::loaders::vertex::Vertex;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ModelVertexSkinned {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub tangent: [f32; 3],
    pub color: [f32; 4],
    
    pub tex_coords: [f32; 2],
    pub bone_weights: [f32; 4],
    pub bone_indices: [u32; 4],
}

impl Vertex for ModelVertexSkinned {
    fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<ModelVertexSkinned>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: offset_of!(ModelVertexSkinned, normal) as u64,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: offset_of!(ModelVertexSkinned, tangent) as u64,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: offset_of!(ModelVertexSkinned, color) as u64,
                    shader_location: 3,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: offset_of!(ModelVertexSkinned, tex_coords) as u64,
                    shader_location: 4,
                    format: wgpu::VertexFormat::Float32x2,
                },
                wgpu::VertexAttribute {
                    offset: offset_of!(ModelVertexSkinned, bone_weights) as u64,
                    shader_location: 5,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: offset_of!(ModelVertexSkinned, bone_indices) as u64,
                    shader_location: 6,
                    format: wgpu::VertexFormat::Uint32x4,
                },
            ]
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ModelVertexUnSkinned {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub tangent: [f32; 3],
    pub color: [f32; 4],
    
    pub tex_coords: [f32; 2],
}

impl Vertex for ModelVertexUnSkinned {
    fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<ModelVertexUnSkinned>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 3]>() as u64,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 6]>() as u64,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 9]>() as u64,
                    shader_location: 3,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 13]>() as u64,
                    shader_location: 4,
                    format: wgpu::VertexFormat::Float32x2,
                },
            ]
        }
    }
}