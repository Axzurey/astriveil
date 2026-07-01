use std::fs::read_to_string;
use wgpu::{BindGroupLayout, PipelineCompilationOptions, PipelineLayoutDescriptor, RenderPipeline, RenderPipelineDescriptor, ShaderModuleDescriptor, VertexState};
use crate::plantain::vertices::{image_vertex::ImageVertex, ui_vertex::UiVertex};
use shared::loaders::vertex::Vertex;
pub fn create_ui_render_pipeline(device: &wgpu::Device, queue: &wgpu::Queue, bindgroup_layouts: Vec<Option<&BindGroupLayout>>, texture_format: wgpu::TextureFormat) -> RenderPipeline {
    let mut ui_shader_path = std::env::current_dir().unwrap();
    ui_shader_path.push("./res/client/shaders/color_shader.wgsl");

    let ui_shader_str = read_to_string(ui_shader_path).expect("Unable to find Quad Shader");
    
    let world_shader = device.create_shader_module(ShaderModuleDescriptor {
        label: Some("Plantain Shader"),
        source: wgpu::ShaderSource::Wgsl(std::borrow::Cow::Borrowed(&ui_shader_str))
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
        label: Some("FRAME UI Render Pipeline"),
        layout: Some(&device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("frame UI pipeline layout"),
            bind_group_layouts: &bindgroup_layouts,
            immediate_size: 0
        })),
        vertex: VertexState {
            module: &world_shader,
            entry_point: Some("vs_main"),
            compilation_options: PipelineCompilationOptions::default(),
            buffers: &[UiVertex::desc()]
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
            cull_mode: Some(wgpu::Face::Back),
            polygon_mode: wgpu::PolygonMode::Fill,
            unclipped_depth: true,
            conservative: false,
        },
        depth_stencil: None,
        multisample: wgpu::MultisampleState::default(),
        cache: None,
        multiview_mask: None
    });

    pipeline
}

pub fn create_ui_image_pipeline(device: &wgpu::Device, queue: &wgpu::Queue, bindgroup_layouts: Vec<Option<&BindGroupLayout>>, texture_format: wgpu::TextureFormat) -> RenderPipeline {
    let mut ui_shader_path = std::env::current_dir().unwrap();
    ui_shader_path.push("./res/client/shaders/image_shader.wgsl");

    let ui_shader_str = read_to_string(ui_shader_path).expect("Unable to find Quad Shader");
    
    let world_shader = device.create_shader_module(ShaderModuleDescriptor {
        label: Some("Plantain Shader"),
        source: wgpu::ShaderSource::Wgsl(std::borrow::Cow::Borrowed(&ui_shader_str))
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
        label: Some("IMAGE UI Render Pipeline"),
        layout: Some(&device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("image UI pipeline layout"),
            bind_group_layouts: &bindgroup_layouts,
            immediate_size: 0
        })),
        vertex: VertexState {
            module: &world_shader,
            entry_point: Some("vs_main"),
            compilation_options: PipelineCompilationOptions::default(),
            buffers: &[ImageVertex::desc()]
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
            cull_mode: Some(wgpu::Face::Back),
            polygon_mode: wgpu::PolygonMode::Fill,
            unclipped_depth: true,
            conservative: false,
        },
        depth_stencil: None,
        multisample: wgpu::MultisampleState::default(),
        cache: None,
        multiview_mask: None
    });

    pipeline
}