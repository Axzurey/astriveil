use std::collections::HashMap;

use nalgebra::Vector2;
use shared::{loaders::{model_bin::ModelBin, texture_bin::TextureBin}, util::xz_to_index, world::chunk::Chunk};
use wgpu::{Buffer, util::DeviceExt};

use crate::{gamecontroller::GameBindgroups, gameloop::entitybin::EntityBin, meshing::{mesher::mesh_slice, vertex::{chunk_vertex::ChunkDataVertex, surface_vertex::SurfaceVertex}}, nominal::camera::Camera, renderer::renderer::Renderer};

//vertex, index, index len, chunk slice
pub type ChunkSliceData = (Buffer, Buffer, u32, Buffer);

pub struct World {
    pub chunks: HashMap<u32, Chunk>,
    pub chunk_data: HashMap<u32, [Option<ChunkSliceData>; 16]>
}

impl World {
    pub fn create_new(device: &wgpu::Device, texture_bin: &TextureBin) -> Self {
        let mut chunks = HashMap::new();
        let mut chunk_data = HashMap::new();

        for x in -3..3 {
            for z in -3..3 {
                let chunk = Chunk::generate_terrain(Vector2::new(x, z));
                chunks.insert(xz_to_index(x, z), chunk);
            }
        }

        for x in -3..3 {
            for z in -3..3 {
                println!("{} {}", x, z);
                let mut cd = [const { None }; 16]; //since when was this a thing? vscode suggested it!
                for y_slice in 0..16 {
                    let slice = mesh_slice(x, z, y_slice, &chunks, texture_bin);

                    let chunk_slice_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                        label: Some("Chunk slice buffer"),
                        contents: bytemuck::cast_slice(&[ChunkDataVertex {
                            position_sliced: [x, y_slice as i32, z]
                        }]),
                        usage: wgpu::BufferUsages::VERTEX
                    });

                    let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                        label: Some("chunk vertex buffer"),
                        contents: bytemuck::cast_slice(&slice.0),
                        usage: wgpu::BufferUsages::VERTEX
                    });

                    let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                        label: Some("chunk index buffer"),
                        contents: bytemuck::cast_slice(&slice.1),
                        usage: wgpu::BufferUsages::INDEX
                    });

                    cd[y_slice as usize] = Some((vertex_buffer, index_buffer, slice.2, chunk_slice_buffer));
                }
                chunk_data.insert(xz_to_index(x, z), cd);
            }
        }

        Self {
            chunks,
            chunk_data
        }
    }

    pub fn render_tick(&mut self, device: &wgpu::Device, queue: &wgpu::Queue, dt: f32, renderer: &mut Renderer, 
        output_texture: &mut wgpu::SurfaceTexture, output_view: &mut wgpu::TextureView, 
        encoder: &mut wgpu::CommandEncoder, bindgroups: &GameBindgroups, camera: &Camera, 
        model_bin: &ModelBin, entity_bin: &mut EntityBin
    ) {
        //renderer.render_chunks(device, queue, output_texture, output_view, encoder, camera, bindgroups, &self.chunk_data);
        renderer.render_entities(device, queue, dt, output_texture, output_view, encoder, camera, bindgroups, model_bin, entity_bin);
    }
}