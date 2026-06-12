use std::collections::HashMap;

use shared::{loaders::{model_bin::{Mesh, ModelBin}, texture::Texture, texture_bin::TextureBin}, world::entities::{entity::{AnimatedEntity, ModelEntity}, zimzam::ZimZam}};
use winit::dpi::PhysicalSize;

use crate::{gamecontroller::GameBindgroups, gameloop::{entitybin::EntityBin, world::ChunkSliceData}, nominal::camera::Camera, renderer::{skinned_model_pipeline::create_skinned_model_pipeline, terrain_pipeline::create_terrain_pipeline}};

pub struct Renderer {
    depth_texture: Texture,
    surface_renderer_pipeline: wgpu::RenderPipeline,
    texture_format: wgpu::TextureFormat,
    scr_width: u32,
    scr_height: u32,
    skinned_render_pipeline: wgpu::RenderPipeline
}

impl Renderer {
    pub fn new(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        texture_format: wgpu::TextureFormat,
        scr_width: u32,
        scr_height: u32,
        texture_bin: &mut TextureBin,
        model_bin: &ModelBin
    ) -> Self {
        let texture_bindgroup_layout = &texture_bin.block_texture_bindgroup_layout;

        let global_bindgroup_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("global bindgroup layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }
            ]
        });

        let depth_texture = Texture::from_empty("Depth Texture", device, wgpu::TextureFormat::Depth32Float, scr_width, scr_height, wgpu::FilterMode::Linear);

        let surface_renderer_pipeline = create_terrain_pipeline(device, queue, vec![Some(texture_bindgroup_layout), Some(&global_bindgroup_layout)], texture_format);

        let skinned_render_pipeline = create_skinned_model_pipeline(device, queue, vec![Some(&global_bindgroup_layout), Some(&texture_bin.free_texture_bindgroup_layout), Some(&model_bin.instance_bind_layout)], texture_format);
        
        Self {
            scr_height,
            scr_width,
            depth_texture,
            texture_format,
            surface_renderer_pipeline,
            skinned_render_pipeline
        }
    }

    pub fn resize(&mut self, size: PhysicalSize<u32>, device: &wgpu::Device) {
        self.scr_height = size.height;
        self.scr_width = size.width;

        self.depth_texture = Texture::from_empty("Depth Texture", device, wgpu::TextureFormat::Depth32Float, self.scr_width, self.scr_height, wgpu::FilterMode::Linear);
    }

    pub fn render_chunks(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        output_texture: &mut wgpu::SurfaceTexture,
        output_view: &wgpu::TextureView,
        encoder: &mut wgpu::CommandEncoder,
        camera: &Camera,
        bindgroups: &GameBindgroups,
        chunk_buffs: &HashMap<u32, [Option<ChunkSliceData>; 16]>
    ) {
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("chunk renderpass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: output_view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: 0.1, g: 0.1, b: 0.1, a: 1.0
                    }),
                    store: wgpu::StoreOp::Store
                },
                depth_slice: None
            })],
            depth_stencil_attachment: Some(
                wgpu::RenderPassDepthStencilAttachment {
                    view: &self.depth_texture.view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: wgpu::StoreOp::Store
                    }),
                    stencil_ops: None
                }
            ),
            timestamp_writes: None,
            occlusion_query_set: None,
            multiview_mask: None
        });

        render_pass.set_pipeline(&self.surface_renderer_pipeline);
        render_pass.set_bind_group(0, Some(&bindgroups.texture_bin.block_texture_bindgroup), &[]);
        render_pass.set_bind_group(1, Some(&camera.bindgroup), &[]);

        for (index, data) in chunk_buffs.iter() {
            for slice_u in data {
                if slice_u.is_none() {continue};

                let (vertices, indices, len, slice_buffer) = slice_u.as_ref().unwrap();

                if *len == 0 {continue};

                render_pass.set_index_buffer(indices.slice(..), wgpu::IndexFormat::Uint32);
                render_pass.set_vertex_buffer(0, vertices.slice(..));
                render_pass.set_vertex_buffer(1, slice_buffer.slice(..));

                render_pass.draw_indexed(0..*len, 0, 0..1);
            }
        }
    }
    pub fn render_entities(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        dt: f32,
        output_texture: &mut wgpu::SurfaceTexture,
        output_view: &wgpu::TextureView,
        encoder: &mut wgpu::CommandEncoder,
        camera: &Camera,
        bindgroups: &GameBindgroups,
        model_bin: &ModelBin,
        entity_bin: &mut EntityBin
    ) {
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("entity renderpass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: output_view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: 0.1, g: 0.1, b: 0.1, a: 1.0
                    }),
                    store: wgpu::StoreOp::Store
                },
                depth_slice: None
            })],
            depth_stencil_attachment: Some(
                wgpu::RenderPassDepthStencilAttachment {
                    view: &self.depth_texture.view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.),
                        store: wgpu::StoreOp::Store
                    }),
                    stencil_ops: None
                }
            ),
            timestamp_writes: None,
            occlusion_query_set: None,
            multiview_mask: None
        });

        render_pass.set_pipeline(&self.skinned_render_pipeline);
        render_pass.set_bind_group(0, Some(&camera.bindgroup), &[]);

        entity_bin.get_entities_with_mut::<ZimZam>().for_each(|entity| {
            let model = model_bin.get_model(entity.get_model());
            
            // TODO
            if let Some(skeleton) = &model.skeleton {
                let animator = entity.get_animator();
                animator.update(dt, model.animations.first().unwrap(), &skeleton);

                let flat = animator.skin_matrices.iter()
                    .flat_map(|v| v.as_slice())
                    .copied().collect::<Vec<_>>();

                let buffer = entity.get_bone_buffer();
                
                queue.write_buffer(buffer, 0, bytemuck::cast_slice(&flat));
            }

            for mesh in &model.meshes {
                let material_name = match mesh {
                    Mesh::Skinned(v) => &v.material_name,
                    Mesh::Static(v) => &v.material_name,
                };
                


                let texture = bindgroups.texture_bin.get_free_texture(material_name);

                render_pass.set_bind_group(1, Some(&texture.bindgroup), &[]);
                render_pass.set_bind_group(2, Some(entity.get_bindgroup()), &[]);
                
                match mesh {
                    Mesh::Skinned(v) => {
                        render_pass.set_vertex_buffer(0, v.vertex_buffer.slice(..));
                        render_pass.set_index_buffer(v.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
                        render_pass.draw_indexed(0..v.index_count as u32,0,0..1);
                    }

                    Mesh::Static(_) => {
                        // TODO
                    }
                }
            }
        });
    }
}