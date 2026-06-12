use std::{collections::HashMap, env, fs::File, io::BufReader};

use nalgebra::{Matrix4, Quaternion, Vector3};
use serde::Deserialize;
use wgpu::{ShaderStages, util::DeviceExt};

use crate::loaders::{
    animator::{AnimationChannel, AnimationClip, Keyframe, Transform},
    modelvertex::{ModelVertexSkinned, ModelVertexUnSkinned},
    texture_bin::TextureBin,
};

pub struct SkeletonBuffer {
    pub buffer: wgpu::Buffer,
}

pub struct SkinnedMesh {
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub index_count: u32,
    pub material_name: String,
}

pub struct UnSkinnedMesh {
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub index_count: u32,
    pub material_name: String,
}

pub enum Mesh {
    Static(UnSkinnedMesh),
    Skinned(SkinnedMesh),
}

pub struct Bone {
    pub name: String,
    pub node_index: usize,

    pub parent: Option<usize>,
    pub children: Vec<usize>,

    pub inverse_bind: Matrix4<f32>,
    pub local_bind: Transform,
}

pub struct Skeleton {
    pub bones: Vec<Bone>,
    pub root_bones: Vec<usize>,
}

pub struct Model {
    //really just the mesh primatives
    pub meshes: Vec<Mesh>,
    pub skeleton: Option<Skeleton>,
    pub animations: Vec<AnimationClip>,
}

pub struct ModelBin {
    models: HashMap<String, Model>,
    pub instance_bind_layout: wgpu::BindGroupLayout,
}

impl ModelBin {
    pub fn new(device: &wgpu::Device) -> Self {
        let instance_bind_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("model & bones layout"),
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: ShaderStages::VERTEX,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: ShaderStages::VERTEX,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Storage { read_only: true },
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    },
                ],
            });

        Self {
            models: HashMap::new(),
            instance_bind_layout,
        }
    }
    pub fn get_model(&self, name: &str) -> &Model {
        self.models
            .get(name)
            .expect(&format!("Model {} does not exist", name))
    }
    pub fn load_model(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        name: String,
        texture_bin: &mut TextureBin,
        path: String,
    ) -> Option<()> {
        let (document, buffers, images) = gltf::import(&path).ok()?;
        document.textures().for_each(|v| {
            let image = &images[v.source().index()];
            texture_bin.add_texture_free(
                v.name()
                    .expect("All model textures should be named")
                    .to_owned(),
                device,
                queue,
                &image.pixels,
                match v
                    .sampler()
                    .mag_filter()
                    .unwrap_or(gltf::texture::MagFilter::Linear)
                {
                    gltf::texture::MagFilter::Linear => wgpu::FilterMode::Linear,
                    gltf::texture::MagFilter::Nearest => wgpu::FilterMode::Nearest,
                },
                image.width,
                image.height,
            )
        });

        //only support 1 mesh for now.
        let mesh = document
            .meshes()
            .next()
            .expect(&format!("File {} should have at least 1 mesh!", path));
        //primitive is like a subpart of the mesh, (like if a mesh has multiple different textures.)
        let meshes = mesh
            .primitives()
            .map(|primitive| {
                let reader = primitive.reader(|b| Some(&buffers[b.index()]));

                let vertex_positions = reader.read_positions().unwrap();

                let indices = reader
                    .read_indices()
                    .unwrap()
                    .into_u32()
                    .collect::<Vec<_>>();

                let is_skinned =
                    reader.read_joints(0).is_some() && reader.read_weights(0).is_some();

                let material_name = primitive.material().name().unwrap_or("texture").to_owned();

                if is_skinned {
                    let vertex_count = vertex_positions.len();

                    let mut positions = vertex_positions;

                    let normals: Vec<[f32; 3]> = reader
                        .read_normals()
                        .map(|n| n.collect())
                        .unwrap_or_else(|| vec![[0.0, 1.0, 0.0]; vertex_count]);

                    let tex_coords: Vec<[f32; 2]> = reader
                        .read_tex_coords(0)
                        .map(|t| t.into_f32().collect())
                        .unwrap_or_else(|| vec![[0.0, 0.0]; vertex_count]);

                    let tangents: Vec<[f32; 4]> = reader
                        .read_tangents()
                        .map(|t| t.map(|v| [v[0], v[1], v[2], v[3]]).collect())
                        .unwrap_or_else(|| vec![[1.0, 0.0, 0.0, 0.0]; vertex_count]);

                    let colors: Vec<[f32; 4]> = reader
                        .read_colors(0)
                        .map(|c| c.into_rgba_f32().collect())
                        .unwrap_or_else(|| vec![[1.0, 1.0, 1.0, 1.0]; vertex_count]);

                    let joints: Vec<[u16; 4]> = reader
                        .read_joints(0)
                        .unwrap()
                        .into_u16()
                        .collect();

                    let weights: Vec<[f32; 4]> = reader
                        .read_weights(0)
                        .unwrap()
                        .into_f32()
                        .collect();

                    let mut vertices = Vec::with_capacity(vertex_count);

                    for i in 0..vertex_count {
                        let position = positions.next().unwrap();

                        let vertex = ModelVertexSkinned {
                            position,
                            normal: normals[i],
                            tex_coords: tex_coords[i],
                            tangent: [tangents[i][0], tangents[i][1], tangents[i][2]],

                            color: colors[i],

                            bone_indices: [
                                joints[i][0] as u32,
                                joints[i][1] as u32,
                                joints[i][2] as u32,
                                joints[i][3] as u32,
                            ],

                            bone_weights: weights[i],
                        };

                        vertices.push(vertex);
                    }

                    let v_buff = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                        label: Some("skinned vertex buffer"),
                        contents: bytemuck::cast_slice(&vertices),
                        usage: wgpu::BufferUsages::VERTEX,
                    });

                    let i_buff = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                        label: Some("index buffer"),
                        contents: bytemuck::cast_slice(&indices),
                        usage: wgpu::BufferUsages::INDEX,
                    });

                    Mesh::Skinned(SkinnedMesh {
                        vertex_buffer: v_buff,
                        index_buffer: i_buff,
                        index_count: indices.len() as u32,
                        material_name,
                    })
                }
                else {
                    todo!();
                }
            })
            .collect::<Vec<_>>();

        //only using first skin for now

        let skeleton = document.skins().next().map(|skin| {
            
            let reader = skin.reader(|b| Some(&buffers[b.index()]));
            let inverse_binds: Vec<_> = reader
                .read_inverse_bind_matrices()
                .unwrap()
                .map(|m| {
                    Matrix4::from_column_slice(&[
                        m[0][0], m[0][1], m[0][2], m[0][3], m[1][0], m[1][1], m[1][2], m[1][3],
                        m[2][0], m[2][1], m[2][2], m[2][3], m[3][0], m[3][1], m[3][2], m[3][3],
                    ])
                })
                .collect();

            let mut node_to_bone = HashMap::new();
            let mut bones = Vec::new();

            for (i, joint) in skin.joints().enumerate() {
                node_to_bone.insert(joint.index(), i);

                let (t, r, s) = joint.transform().decomposed();

                bones.push(Bone {
                    name: joint.name().unwrap_or("bone").to_string(),
                    node_index: joint.index(),

                    parent: None,
                    children: Vec::new(),

                    inverse_bind: inverse_binds[i],
                    local_bind: Transform {
                        translation: t.into(),
                        rotation: r.into(),
                        scale: s.into(),
                    },
                });
            }

            for joint in skin.joints() {
                
                let bone_index = node_to_bone[&joint.index()];

                for child in joint.children() {
                    if let Some(&child_index) = node_to_bone.get(&child.index()) {
                        bones[bone_index].children.push(child_index);
                        bones[child_index].parent = Some(bone_index);
                    }
                }
            }

            let root_bones = bones
                .iter()
                .enumerate()
                .filter(|(_, b)| b.parent.is_none())
                .map(|(i, _)| i)
                .collect();

            Skeleton { bones, root_bones }
        });

        let bone_lookup: HashMap<usize, usize> = skeleton
            .as_ref()
            .map(|s| {
                s.bones
                    .iter()
                    .enumerate()
                    .map(|(i, b)| (b.node_index, i))
                    .collect()
            })
            .unwrap_or_default();

        let animations = document
            .animations()
            .map(|anim| {
                let mut channels = Vec::new();
                let mut duration = 0.0f32;

                for channel in anim.channels() {
                    let node_index = channel.target().node().index();

                    if !bone_lookup.contains_key(&node_index) {
                        continue;
                    }

                    let bone_index = bone_lookup[&node_index];

                    let reader = channel.reader(|b| Some(&buffers[b.index()]));
                    let times: Vec<f32> = reader.read_inputs().unwrap().collect();

                    if let Some(last) = times.last() {
                        duration = duration.max(*last);
                    }

                    let mut anim_channel = AnimationChannel {
                        bone_index,
                        translations: Vec::new(),
                        rotations: Vec::new(),
                        scales: Vec::new(),
                    };

                    match reader.read_outputs().unwrap() {
                        gltf::animation::util::ReadOutputs::Translations(v) => {
                            for (time, value) in times.iter().zip(v) {
                                anim_channel.translations.push(Keyframe {
                                    time: *time,
                                    value: Vector3::new(value[0], value[1], value[2]),
                                });
                            }
                        }

                        gltf::animation::util::ReadOutputs::Rotations(v) => {
                            for (time, value) in times.iter().zip(v.into_f32()) {
                                anim_channel.rotations.push(Keyframe {
                                    time: *time,
                                    value: Quaternion::new(value[3], value[0], value[1], value[2]),
                                });
                            }
                        }

                        gltf::animation::util::ReadOutputs::Scales(v) => {
                            for (time, value) in times.iter().zip(v) {
                                anim_channel.scales.push(Keyframe {
                                    time: *time,
                                    value: Vector3::new(value[0], value[1], value[2]),
                                });
                            }
                        }

                        _ => {}
                    }

                    channels.push(anim_channel);
                }

                AnimationClip {
                    name: anim.name().unwrap_or("anim").to_string(),
                    duration,
                    channels,
                }
            })
            .collect::<Vec<_>>();

        let model = Model {
            animations,
            meshes,
            skeleton,
        };

        self.models.insert(name, model);

        Some(())
    }
    
    pub fn load_models(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        format: wgpu::TextureFormat,
        texture_bin: &mut TextureBin,
    ) {
        let mut model_manifest_path = env::current_dir().unwrap();
        model_manifest_path.push("res/client/data/model_manifest.json");

        let file = File::open(model_manifest_path).expect("Unable to open model manifest");
        let reader = BufReader::new(file);
        let data: Vec<ModelLoadData> =
            serde_json::from_reader(reader).expect("Unable to read model_manifest file");

        for def in data {
            let mut model_path = env::current_dir().unwrap();
            model_path.push(&format!("res/client/models/{}", def.path));

            self.load_model(
                device,
                queue,
                def.name.clone(),
                texture_bin,
                model_path.to_str().unwrap().to_owned(),
            );
        }
    }
}

#[derive(Deserialize)]
struct ModelLoadData {
    name: String,
    path: String,
}
