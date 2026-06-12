use std::any::Any;

use nalgebra::{Matrix4, Quaternion, Translation3, UnitQuaternion, Vector3};

use crate::loaders::animator::Animator;
pub trait Entity {
    fn update(&mut self, device: &wgpu::Device);
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}
pub trait ModelEntity: Entity {
    fn set_model(&mut self, model: String);
    fn get_model(&self) -> &str;
    fn get_bindgroup(&self) -> &wgpu::BindGroup;
    fn get_bone_buffer(&mut self) -> &mut wgpu::Buffer;
}
pub trait AnimatedEntity: ModelEntity + Entity {
    fn get_animator(&mut self) -> &mut Animator;
}
pub trait ModelHealth: Entity {
    /*
    returns true if the entity is dead, and false if alive.
    must check that the health doesn't get below 0
     */
    fn decrease_health_checked(&mut self, decrement: u32) -> bool;
    fn set_health(&mut self, health: u32);
    fn get_health(&self) -> u32;
}
pub trait ModelLocation: Entity {
    fn set_position(&mut self, position: Vector3<f32>);
    fn get_position(&self) -> &Vector3<f32>;

    fn set_scale(&mut self, scale: Vector3<f32>);
    fn get_scale(&self) -> &Vector3<f32>;

    fn set_rotation(&mut self, rotation: Quaternion<f32>);
    fn get_rotation(&self) -> &Quaternion<f32>;
}

pub fn transform_to_matrix(position: Vector3<f32>, rotation: Quaternion<f32>, scale: Vector3<f32>) -> Matrix4<f32> {
    Translation3::from(position).to_homogeneous() 
    * UnitQuaternion::from_quaternion(rotation).to_homogeneous()
    * Matrix4::new_nonuniform_scaling(&scale)
}