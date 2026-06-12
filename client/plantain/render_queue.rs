use glyphon::{Cache, FontSystem, Metrics, Resolution, SwashCache, TextAtlas, TextRenderer, Viewport};
use nalgebra::Vector2;
use wgpu::{BindGroupLayout, MultisampleState, RenderPipeline};

use crate::{gamecontroller::GameBindgroups, plantain::{elements::screen::Screen, render_pipeline::{create_ui_image_pipeline, create_ui_render_pipeline}}};

pub struct UiRenderQueue {
    color_pipeline: RenderPipeline,
    image_pipeline: RenderPipeline,
    cache: Cache,
    font_system: FontSystem,
    viewport: Viewport,
    pub atlas: TextAtlas,
    text_renderer: TextRenderer,
    swash_cache: SwashCache
}

pub struct PipelineRefs<'a> {
    pub color_pipeline: &'a RenderPipeline,
    pub image_pipeline: &'a RenderPipeline,
    pub cache: &'a Cache,
    pub font_system: &'a mut FontSystem,
    pub viewport: &'a Viewport,
    pub atlas: &'a mut TextAtlas,
    pub text_renderer: &'a mut TextRenderer,
    pub swash_cache: &'a mut SwashCache,
    pub screen_dims: [f32; 2],
    pub mouse_pos: Vector2<f32>
}

impl UiRenderQueue {
    pub fn new(device: &wgpu::Device, queue: &wgpu::Queue, bindgroup_layouts: Vec<Option<&BindGroupLayout>>, texture_format: wgpu::TextureFormat) -> Self {
        
        let font_system = FontSystem::new();
        let swash_cache = SwashCache::new();
        let cache = Cache::new(device);
        let viewport = Viewport::new(device, &cache);
        let mut atlas = TextAtlas::new(device, queue, &cache, texture_format);
        let text_renderer = TextRenderer::new(&mut atlas, device, MultisampleState::default(), None);

        Self {
            color_pipeline: create_ui_render_pipeline(device, queue, bindgroup_layouts.clone(), texture_format),
            image_pipeline: create_ui_image_pipeline(device, queue, bindgroup_layouts, texture_format),
            atlas,
            font_system,
            text_renderer,
            cache,
            viewport,
            swash_cache
        }
    }

    pub fn render(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        output_texture: &mut wgpu::SurfaceTexture,
        output_view: &wgpu::TextureView,
        encoder: &mut wgpu::CommandEncoder,
        screen: &Screen,
        bindgroups: &GameBindgroups,
        mouse_pos: &Vector2<f32>
    ) {
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("chunk renderpass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: output_view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Load,
                    store: wgpu::StoreOp::Store
                },
                depth_slice: None
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
            multiview_mask: None
        });

        self.viewport.update(queue, Resolution {width: output_texture.texture.width(), height: output_texture.texture.height()});

        render_pass.set_bind_group(0, Some(&bindgroups.texture_bin.block_texture_bindgroup), &[]);
        render_pass.set_bind_group(1, Some(&screen.cam_bg), &[]);

        let mut pipeline_refs = PipelineRefs {
            color_pipeline: &self.color_pipeline,
            image_pipeline: &self.image_pipeline,
            cache: &self.cache,
            font_system: &mut self.font_system,
            viewport: &self.viewport,
            atlas: &mut self.atlas,
            text_renderer: &mut self.text_renderer,
            swash_cache: &mut self.swash_cache,
            screen_dims: [output_texture.texture.width() as f32, output_texture.texture.height() as f32],
            mouse_pos: mouse_pos.clone()
        };

        screen.draw_to_buffers(device, queue, &mut render_pass, &mut pipeline_refs);

        self.text_renderer.render(&self.atlas, &self.viewport, &mut render_pass).unwrap();
    }
}