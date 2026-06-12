use std::any::Any;

use nalgebra::{Matrix4, Quaternion, Vector3};
use wgpu::util::{BufferInitDescriptor, DeviceExt};

use crate::{loaders::{animator::Animator, model_bin::{Model, ModelBin}}, world::entities::entity::{AnimatedEntity, Entity, ModelEntity, ModelLocation, transform_to_matrix}};

pub struct ZimZam {
    position: Vector3<f32>,
    rotation: Quaternion<f32>,
    scale: Vector3<f32>,
    model: String,
    model_buffer: wgpu::Buffer,
    bone_buffer: wgpu::Buffer,
    bind_group: wgpu::BindGroup,
    animator: Animator
}

impl ZimZam {
    pub fn new(device: &wgpu::Device, model_bin: &ModelBin, model: String) -> Box<ZimZam> {

        let model_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("zimzam model buffer"),
            contents: bytemuck::cast_slice(&transform_to_matrix(
                Vector3::new(0., 100., 0.),
                Quaternion::identity(),
                Vector3::new(1., 1., 1.)
            ).as_slice()),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST
        });

        let identity = Matrix4::<f32>::identity();

        let initial_bones = vec![identity; 128];

        let bone_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("zimzam bone buffer"),
            contents: bytemuck::cast_slice(
                &initial_bones.iter().flat_map(|m| m.as_slice()).copied().collect::<Vec<_>>()
            ),
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("entity bind"),
            layout: &model_bin.instance_bind_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: model_buffer.as_entire_binding()
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: bone_buffer.as_entire_binding()
                }
            ]
        });

        let animator = Animator::new(128);

        Box::new(Self {
            position: Vector3::new(0., 100., 0.),
            rotation: Quaternion::identity(),
            scale: Vector3::new(1., 1., 1.),
            model,
            model_buffer,
            bone_buffer,
            bind_group,
            animator
        })
    }
}

impl AnimatedEntity for ZimZam {
    fn get_animator(&mut self) -> &mut crate::loaders::animator::Animator {
        &mut self.animator
    }
}

impl Entity for ZimZam {
    fn update(&mut self, device: &wgpu::Device) {
        
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl ModelEntity for ZimZam {
    fn get_model(&self) -> &str {
        &self.model
    }
    fn set_model(&mut self, model: String) {
        self.model = model;
    }
    fn get_bindgroup(&self) -> &wgpu::BindGroup {
        &self.bind_group
    }
    fn get_bone_buffer(&mut self) -> &mut wgpu::Buffer {
        &mut self.bone_buffer
    }
}

impl ModelLocation for ZimZam {
    fn get_position(&self) -> &Vector3<f32> {
        &self.position
    }
    fn get_rotation(&self) -> &Quaternion<f32> {
        &self.rotation
    }
    fn get_scale(&self) -> &Vector3<f32> {
        &self.scale
    }
    fn set_position(&mut self, position: Vector3<f32>) {
        self.position = position;
    }
    fn set_rotation(&mut self, rotation: Quaternion<f32>) {
        self.rotation = rotation;
    }
    fn set_scale(&mut self, scale: Vector3<f32>) {
        self.scale = scale;
    }
}