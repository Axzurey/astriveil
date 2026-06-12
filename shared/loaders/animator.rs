use std::collections::HashMap;

use nalgebra::{Matrix4, Quaternion, UnitQuaternion, Vector3};

use crate::loaders::model_bin::Skeleton;

pub struct Keyframe<T> {
    pub time: f32,
    pub value: T
}

pub struct Transform {
    pub translation: Vector3<f32>,
    pub rotation: Quaternion<f32>,
    pub scale: Vector3<f32>
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            translation: Vector3::zeros(),
            rotation: Quaternion::identity(),
            scale: Vector3::new(0., 0., 0.)
        }
    }
}

pub struct AnimationClip {
    pub name: String,
    pub duration: f32,
    pub channels: Vec<AnimationChannel>
}

#[derive(Default)]
pub struct Animator {
    pub time: f32,
    pub local_pose: Vec<Transform>,
    pub global_pose: Vec<Matrix4<f32>>,
    pub skin_matrices: Vec<Matrix4<f32>>
}

fn local_to_matrix(t: &Transform) -> Matrix4<f32> {
    let translation = Matrix4::new_translation(&t.translation);
    let rotation = UnitQuaternion::new_normalize(t.rotation).to_homogeneous();
    let scale = Matrix4::new_nonuniform_scaling(&t.scale);
    translation * rotation * scale
}

impl Animator {
    pub fn new(bone_count: usize) -> Self {
        Self {
            time: 0.0,
            local_pose: (0..bone_count).map(|_| Transform::default()).collect(),
            global_pose: vec![Matrix4::identity(); bone_count],
            skin_matrices: vec![Matrix4::identity(); bone_count]
        }
    }
    pub fn play(&mut self) {
        self.time = 0.;
    }
    pub fn update(&mut self, dt: f32, clip: &AnimationClip, skeleton: &Skeleton) {

        self.time = (self.time + dt) % clip.duration;

        for (i, bone) in skeleton.bones.iter().enumerate() {
            self.local_pose[i] = Transform {
                translation: bone.local_bind.translation,
                rotation: bone.local_bind.rotation,
                scale: bone.local_bind.scale,
            };
        }

        for channel in &clip.channels {
            let pose = &mut self.local_pose[channel.bone_index];
            if let Some(t) = sample_vec3(&channel.translations, self.time) {
                pose.translation = t;
            }
            if let Some(r) = sample_quaternion(&channel.rotations, self.time) {
                pose.rotation = r;
            }
            if let Some(s) = sample_vec3(&channel.scales, self.time) {
                pose.scale = s;
            }
        }

        let mut stack = skeleton.root_bones.clone();
        while let Some(i) = stack.pop() {
            let local = local_to_matrix(&self.local_pose[i]);
            self.global_pose[i] = match skeleton.bones[i].parent {
                Some(parent) => self.global_pose[parent] * local,
                None => local,
            };
            self.skin_matrices[i] = self.global_pose[i] * skeleton.bones[i].inverse_bind;
            for &child in &skeleton.bones[i].children {
                stack.push(child);
            }
        }
    }
}

pub struct AnimationChannel {
    pub bone_index: usize,
    pub translations: Vec<Keyframe<Vector3<f32>>>,
    pub rotations: Vec<Keyframe<Quaternion<f32>>>,
    pub scales: Vec<Keyframe<Vector3<f32>>>,
}

fn sample_vec3(keyframes: &[Keyframe<Vector3<f32>>], time: f32) -> Option<Vector3<f32>> {
    if keyframes.is_empty() { return None; }
    if keyframes.len() == 1 { return Some(keyframes[0].value); }

    let next = keyframes.partition_point(|k| k.time <= time).min(keyframes.len() - 1);
    let prev = next.saturating_sub(1);

    if prev == next {
        return Some(keyframes[prev].value);
    }

    let t0 = keyframes[prev].time;
    let t1 = keyframes[next].time;
    let t = (time - t0) / (t1 - t0);

    Some(keyframes[prev].value.lerp(&keyframes[next].value, t))
}

fn sample_quaternion(keyframes: &[Keyframe<Quaternion<f32>>], time: f32) -> Option<Quaternion<f32>> {
    if keyframes.is_empty() { return None; }
    if keyframes.len() == 1 { return Some(keyframes[0].value); }

    let next = keyframes.partition_point(|k| k.time <= time).min(keyframes.len() - 1);
    let prev = next.saturating_sub(1);

    if prev == next {
        return Some(keyframes[prev].value);
    }

    let t0 = keyframes[prev].time;
    let t1 = keyframes[next].time;
    let t = (time - t0) / (t1 - t0);

    let a = UnitQuaternion::new_normalize(keyframes[prev].value);
    let b = UnitQuaternion::new_normalize(keyframes[next].value);
    Some(a.slerp(&b, t).into_inner())
}