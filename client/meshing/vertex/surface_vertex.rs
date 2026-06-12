use std::{mem, ops::BitOrAssign};

use shared::{blocks::block::BlockFace, loaders::texture_bin::TextureBin};
use shared::loaders::vertex::Vertex;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct SurfaceVertex {
    pub d0: u32,
    pub d1: u32,
    pub illumination: u32
}

impl SurfaceVertex {
    pub fn new(local_position: [u32; 3], texture_index: u32, face: BlockFace, nth: u32) -> SurfaceVertex {
        
        let faceval: u32 = face.into();
        
        let mut d0 = local_position[0];

        d0.bitor_assign(local_position[1] << 5);
        d0.bitor_assign(local_position[2] << 10);
        d0.bitor_assign(faceval << 15);
        d0.bitor_assign(nth << 18);

        let d1 = texture_index;
        let illumination = 15 << 24;
    
        SurfaceVertex {
            d0, d1, illumination
        }
    }
}

impl Vertex for SurfaceVertex {
    fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<SurfaceVertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Uint32,
                },
                wgpu::VertexAttribute {
                    offset: 4,//u32 4 bytes
                    shader_location: 1,
                    format: wgpu::VertexFormat::Uint32,
                }
                ,
                wgpu::VertexAttribute {
                    offset: 8,//u32 4 bytes
                    shader_location: 2,
                    format: wgpu::VertexFormat::Uint32,
                }
            ]
        }
    }
}