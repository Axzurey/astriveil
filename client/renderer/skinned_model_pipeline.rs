use std::fs::read_to_string;

use wgpu::{BindGroupLayout, PipelineCompilationOptions, PipelineLayoutDescriptor, RenderPipeline, RenderPipelineDescriptor, ShaderModuleDescriptor, VertexState};

use crate::meshing::vertex::{chunk_vertex::ChunkDataVertex, surface_vertex::SurfaceVertex};
use shared::loaders::{modelvertex::ModelVertexSkinned, vertex::Vertex};

pub fn create_skinned_model_pipeline(device: &wgpu::Device, queue: &wgpu::Queue, bindgroup_layouts: Vec<Option<&BindGroupLayout>>, texture_format: wgpu::TextureFormat) -> RenderPipeline {
    let mut world_shader_path = std::env::current_dir().unwrap();
    world_shader_path.push("./res/client/shaders/skinned_model_shader.wgsl");

    let world_shader_str = read_to_string(world_shader_path).expect("Unable to find skinned Shader");
    
    let world_shader = device.create_shader_module(ShaderModuleDescriptor {
        label: Some("World Shader"),
        source: wgpu::ShaderSource::Wgsl(std::borrow::Cow::Borrowed(&world_shader_str))
    });

    let color_targetstate = [Some(wgpu::ColorTargetState {
        format: texture_format,
        blend: Some(wgpu::BlendState {
            alpha: wgpu::BlendComponent::OVER,
            color: wgpu::BlendComponent {
                src_factor: wgpu::BlendFactor::SrcAlpha,
                dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
                operation: wgpu::BlendOperation::Add
            },
        }),
        write_mask: wgpu::ColorWrites::ALL,
    })];
    
    let pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
        label: Some("skinned Render Pipeline"),
        layout: Some(&device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("skinned pipeline layout"),
            bind_group_layouts: &bindgroup_layouts,
            immediate_size: 0
        })),
        vertex: VertexState {
            module: &world_shader,
            entry_point: Some("vs_main"),
            compilation_options: PipelineCompilationOptions::default(),
            buffers: &[ModelVertexSkinned::desc()]
        },
        fragment: Some(wgpu::FragmentState {
            module: &world_shader,
            entry_point: Some("fs_main"),
            targets: &color_targetstate,
            compilation_options: wgpu::PipelineCompilationOptions::default(),
        }),
        primitive: wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleList,
            strip_index_format: None,
            front_face: wgpu::FrontFace::Ccw,
            cull_mode: None,
            polygon_mode: wgpu::PolygonMode::Fill,
            unclipped_depth: true,
            conservative: false,
        },
        depth_stencil: Some(wgpu::DepthStencilState {
            format: wgpu::TextureFormat::Depth32Float,
            depth_write_enabled: Some(true),
            depth_compare: Some(wgpu::CompareFunction::Less),
            stencil: wgpu::StencilState::default(),
            bias: wgpu::DepthBiasState::default(),
        }),
        multisample: wgpu::MultisampleState::default(),
        cache: None,
        multiview_mask: None
    });

    pipeline
}