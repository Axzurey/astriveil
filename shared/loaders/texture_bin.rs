use std::{collections::HashMap, env, fs::File, io::BufReader, num::NonZeroU32};

use wgpu::{Extent3d, FilterMode};
use serde::Deserialize;

use crate::loaders::texture::{Texture, load_binary_sync};
pub struct LoadedTextureInfo {
    pub name: String,
    pub texture: Texture
}

#[derive(Deserialize)]
#[serde(rename_all="snake_case")]
enum ImportFilterMode {
    Linear,
    Nearest
}

impl Into<FilterMode> for ImportFilterMode {
    fn into(self) -> FilterMode {
        match self {
            ImportFilterMode::Linear => FilterMode::Linear,
            ImportFilterMode::Nearest => FilterMode::Nearest
        }
    }
}

#[derive(Deserialize)]
struct TextureLoadData {
    pub path: String,
    pub name: String,
    pub filter: ImportFilterMode
}

pub struct FreeTexture {
    pub texture: wgpu::Texture,
    pub sampler: wgpu::Sampler,
    pub view: wgpu::TextureView,
    pub bindgroup: wgpu::BindGroup
}

pub struct TextureBin {
    block_texture_lookup_map: HashMap<String, u32>,
    pub block_texture: wgpu::Texture,
    pub block_view: wgpu::TextureView,
    next_free_layer: u32,
    pub block_texture_bindgroup_layout: wgpu::BindGroupLayout,
    pub block_texture_bindgroup: wgpu::BindGroup,

    pub format: wgpu::TextureFormat,

    pub free_texture_bindgroup_layout: wgpu::BindGroupLayout,
    free_textures: HashMap<String, FreeTexture>
}

impl TextureBin {
    pub fn new(device: &wgpu::Device, format: wgpu::TextureFormat) -> Self {
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("block-textures"),
            size: wgpu::Extent3d {
                width: 16,
                height: 16,
                depth_or_array_layers: 256
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[]
        });

        let view = texture.create_view(&wgpu::TextureViewDescriptor {
            dimension: Some(wgpu::TextureViewDimension::D2Array),
            ..Default::default()
        });

        let block_texture_bindgroup_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        multisampled: false,
                        view_dimension: wgpu::TextureViewDimension::D2Array,
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    },
                    count: NonZeroU32::new(256),
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(
                        wgpu::SamplerBindingType::Filtering
                    ),
                    count: None,
                }
            ],
            label: Some("block-texture-bindgroup-layout")
        });

        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Nearest,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::MipmapFilterMode::Nearest,
            ..Default::default()
        });

        let block_texture_bindgroup = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("block-texture-bindgroup"),
            layout: &block_texture_bindgroup_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&view)
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&sampler)
                }
            ]
        });

        let free_texture_bindgroup_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("free-texture-bindgroup-layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        multisampled: false,
                        sample_type: wgpu::TextureSampleType::Float {
                            filterable: true,
                        },
                        view_dimension: wgpu::TextureViewDimension::D2,
                    },
                    count: None
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(
                        wgpu::SamplerBindingType::Filtering,
                    ),
                    count: None,
                },
            ],
        });

        Self {
            block_texture_lookup_map: HashMap::new(),
            block_texture: texture,
            next_free_layer: 0,
            block_view: view,
            block_texture_bindgroup_layout,
            block_texture_bindgroup,
            format,
            free_texture_bindgroup_layout,
            free_textures: HashMap::new()
        }
    }
    /**
     * panics if there is no such texture
     */
    pub fn get_texture_index_by_name_block(&self, name: &str) -> u32 {
        *self.block_texture_lookup_map.get(name).unwrap()
    }
    pub fn get_free_texture(&self, name: &str) -> &FreeTexture {
        self.free_textures.get(name).expect(&format!("Texture {} does not exist", name))
    }
    pub fn add_texture_free(
        &mut self,
        name: String,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        data: &[u8],
        filter: wgpu::FilterMode,
        width: u32,
        height: u32
    ) {
        if self.free_textures.contains_key(&name) {
            println!("{}", format!("[Warning] Texture {} already exists", name));
        }

        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some(&format!("free texture {}", &name)),
            sample_count: 1,
            size: Extent3d {width, height, depth_or_array_layers: 1},
            dimension: wgpu::TextureDimension::D2,
            mip_level_count: 1,
            format: self.format,
            view_formats: &[],
            usage: wgpu::TextureUsages::COPY_DST | wgpu::TextureUsages::TEXTURE_BINDING
        });

        let view = texture.create_view(&wgpu::TextureViewDescriptor {
            label: Some(&format!("free texture {} - view", &name)),
            ..Default::default()
        });

        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some(&format!("free texture {} - sampler", &name)),
            mag_filter: filter,
            min_filter: filter,
            ..Default::default()
        });

        let bg = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("free-texture-bindgroup"),
            layout: &self.free_texture_bindgroup_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&sampler),
                },
            ],
        });

        queue.write_texture(
            wgpu::TexelCopyTextureInfo {
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            data,
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(4 * width),
                rows_per_image: Some(height),
            },
            wgpu::Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
        );

        self.free_textures.insert(name, FreeTexture { texture, sampler, view, bindgroup: bg });
    }
    pub fn add_texture_block(&mut self, name: String, queue: &wgpu::Queue, data: &[u8]) {
        self.block_texture_lookup_map.insert(name, self.next_free_layer);

        queue.write_texture(
            wgpu::TexelCopyTextureInfo {
                texture: &self.block_texture,
                mip_level: 0,
                origin: wgpu::Origin3d {
                    x: 0,
                    y: 0,
                    z: self.next_free_layer
                },
                aspect: wgpu::TextureAspect::All
            },
            data,
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(4 * 16), //block textures are 16x16, 1 byte per channel, 4 channels
                rows_per_image: Some(16)
            },
            wgpu::Extent3d {
                width: 16,
                height: 16,
                depth_or_array_layers: 1
            }
        );

        self.next_free_layer += 1;
    }
    pub fn load_textures_world(&mut self, device: &wgpu::Device, queue: &wgpu::Queue, format: wgpu::TextureFormat) {
        let encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Texture Loading Encoder")
        });

        let mut texture_manifest_path = env::current_dir().unwrap();
        texture_manifest_path.push("res/client/data/texture_manifest.json");

        let file = File::open(texture_manifest_path).expect("Unable to open texture manifest");
        let reader = BufReader::new(file);
        let data: Vec<TextureLoadData> = serde_json::from_reader(reader).expect("Unable to read texture_manifest file");

        for def in data {
            let mut texture_path = env::current_dir().unwrap();
            texture_path.push(&format!("res/client/textures/{}", def.path));

            let data = &load_binary_sync(texture_path.to_str().unwrap()).unwrap();

            self.add_texture_block(def.name.clone(), queue, data);
        }
        queue.submit(std::iter::once(encoder.finish()));
    }
}