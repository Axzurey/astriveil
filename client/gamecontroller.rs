use std::sync::Arc;

use nalgebra::{Point3, Vector2};
use shared::{loaders::{model_bin::ModelBin, texture_bin::TextureBin}, world::entities::zimzam::ZimZam};
use wgpu::{BackendOptions, CurrentSurfaceTexture, MemoryBudgetThresholds, TextureFormat};
use winit::{dpi::PhysicalSize, event::{ElementState, KeyEvent, MouseButton}, keyboard::PhysicalKey};

use crate::{gameloop::{entitybin::EntityBin, world::World}, nominal::camera::Camera, plantain::{elements::{element::{DimD2, UiElement}, frame::Frame, image_frame::ImageFrame, screen::{Screen, UiLayer}, textbox::TextBox, textlabel::TextLabel}, render_queue::UiRenderQueue}, renderer::renderer::Renderer};

struct WorldState {
    pub world: World,
    pub camera: Camera,
    pub entity_bin: EntityBin
}

enum GameState {
    MainMenu,
    WorldSelect,
    InWorld(WorldState)
}

impl GameState {
    pub fn join_new_world(aspect_ratio: f32, fov: f32, device: &wgpu::Device, layout: &wgpu::BindGroupLayout, texture_bin: &TextureBin, model_bin: &ModelBin) -> Self {
        let world = World::create_new(device, texture_bin);
        let camera = Camera::new(Point3::new(15., 100., 0.), 0., 0., aspect_ratio, fov, device, layout);

        let mut entity_bin = EntityBin::new();

        let zim = ZimZam::new(device, model_bin, "zimzam".to_string());

        entity_bin.add_entity(zim);



        GameState::InWorld(WorldState {
            world,
            camera,
            entity_bin
        })
    }
}

pub struct GameBindgroups {
    pub texture_bin: TextureBin,
    pub camera_layout: wgpu::BindGroupLayout
}

pub struct GameController<'a> {
    game_state: GameState,
    renderer: Renderer,
    surface: wgpu::Surface<'a>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    surface_config: wgpu::SurfaceConfiguration,
    window_size: winit::dpi::PhysicalSize<u32>,
    surface_format: wgpu::TextureFormat,
    game_bindgroups: GameBindgroups,
    ui_renderqueue: UiRenderQueue,
    ui_screen: Screen,
    model_bin: ModelBin,
    mouse_position: Vector2<f32>
}

impl<'a> GameController<'a> {
    pub async fn new(size: winit::dpi::PhysicalSize<u32>, window: Arc<winit::window::Window>) -> Self {
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            flags: wgpu::InstanceFlags::empty(),
            backends: wgpu::Backends::VULKAN,
            memory_budget_thresholds: MemoryBudgetThresholds::default(),
            backend_options: BackendOptions::from_env_or_default(),
            display: None
        });

        let surface = instance.create_surface(window).unwrap();

        let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            compatible_surface: Some(&surface),
            force_fallback_adapter: false
        }).await.unwrap();

        let (device, queue) = adapter.request_device(&wgpu::DeviceDescriptor {
            required_features: wgpu::Features::TEXTURE_BINDING_ARRAY
             | wgpu::Features::SAMPLED_TEXTURE_AND_STORAGE_BUFFER_ARRAY_NON_UNIFORM_INDEXING 
             | wgpu::Features::DEPTH_CLIP_CONTROL,
            required_limits: wgpu::Limits {
                max_bind_groups: 5,
                max_binding_array_elements_per_shader_stage: 256,
                max_binding_array_sampler_elements_per_shader_stage: 256,
                ..Default::default()
            },
            label: None,
            memory_hints: wgpu::MemoryHints::Performance,
            experimental_features: wgpu::ExperimentalFeatures::disabled(),
            trace: wgpu::Trace::Off
        }).await.unwrap();

        let surface_capabilities = surface.get_capabilities(&adapter);

        let surface_format = TextureFormat::Rgba8UnormSrgb; //make this more robust.

        let surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_DST,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_capabilities.present_modes[0],
            alpha_mode: surface_capabilities.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2
        };

        surface.configure(&device, &surface_config);

        let mut texture_bin = TextureBin::new(&device, surface_format);
        texture_bin.load_textures_world(&device, &queue, surface_format);

        let mut model_bin = ModelBin::new(&device);
        model_bin.load_models(&device, &queue, surface_format, &mut texture_bin);

        let renderer = Renderer::new(&device, &queue, surface_format.clone(), size.width, size.height, &mut texture_bin, &model_bin);

        let game_bindgroups = {
            let camera_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None
                        },
                        count: None,
                    }
                ],
                label: Some("Camera Layout")
            });

            GameBindgroups {
                camera_layout,
                texture_bin,
            }
        };

        let game_state = GameState::join_new_world(size.width as f32 / size.height as f32, 70., &device, &game_bindgroups.camera_layout, &game_bindgroups.texture_bin, &model_bin);

        let mut ui_screen = Screen::new(&device, size.width, size.height);

        let mut ui_renderqueue = UiRenderQueue::new(&device, &queue, vec![Some(&game_bindgroups.texture_bin.block_texture_bindgroup_layout), Some(&ui_screen.cam_bgl)], surface_format);

        //ui_screen.add_child(Frame::new());
        //ui_screen.add_child(ImageFrame::new());
        let mut label = TextBox::new(&mut ui_renderqueue.font_system);
        label.set_layer(UiLayer::Menu);
        label.border.set_corner_radius([5., 5., 5., 5.]);
        label.border.set_color([1., 1., 1., 1.]);
        label.border.set_thickness(2.);
        label.dims.set_position(DimD2::from_scale(0.5, 0.5));
        label.dims.set_size(DimD2::from_offset(320., 54.));
        label.set_text_size(42.);
        label.set_line_height(42.);
        label.set_text_align_x(glyphon::cosmic_text::Align::Left);
        label.set_text_align_y(glyphon::cosmic_text::Align::Left);

        ui_screen.add_child(label);

        Self {
            game_state,
            renderer,
            device,
            queue,
            surface,
            surface_config,
            surface_format,
            window_size: size,
            game_bindgroups,
            ui_renderqueue,
            ui_screen,
            model_bin,
            mouse_position: Vector2::identity()
        }
    }

    pub fn set_mouse_position(&mut self, pos: Vector2<f32>) {
        self.mouse_position = pos;
    }

    pub fn on_window_update(&mut self, dt: f32) {
        let mut output = match self.surface.get_current_texture() {
            CurrentSurfaceTexture::Suboptimal(output) => {
                self.surface.configure(&self.device, &self.surface_config);
                output
            },
            CurrentSurfaceTexture::Success(output) => {
                output
            },
            CurrentSurfaceTexture::Timeout => return,
            CurrentSurfaceTexture::Occluded => return,
            CurrentSurfaceTexture::Outdated => {
                self.surface.configure(&self.device, &self.surface_config);
                return
            },
            CurrentSurfaceTexture::Lost => {
                return
            },
            CurrentSurfaceTexture::Validation => {
                return
            }
        };

        let mut view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
        
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("primary encoder")
        });

        self.ui_screen.get_children_mut_of_type::<Frame>().for_each(|v| {
            let rot = v.get_rotation() + 0.001;
            v.set_rotation(rot);
        });

        match &mut self.game_state {
            GameState::InWorld(worldstate) => {
                worldstate.camera.update_camera(dt);
                worldstate.camera.update_matrices(&self.queue);

                worldstate.world.render_tick(&self.device, &self.queue, dt, &mut self.renderer, &mut output, &mut view, &mut encoder, &self.game_bindgroups, &worldstate.camera, &self.model_bin, &mut worldstate.entity_bin);
            },
            _ => {}
        }

        self.ui_renderqueue.render(&self.device, &self.queue, &mut output, &mut view, &mut encoder, &mut self.ui_screen, &self.game_bindgroups, &self.mouse_position);

        self.queue.submit([encoder.finish()]);
        output.present();
        self.ui_renderqueue.atlas.trim();
    }

    pub fn on_window_resized(&mut self, new_size: PhysicalSize<u32>) {
        self.surface_config.width = new_size.width;
        self.surface_config.height = new_size.height;
        
        self.surface.configure(&self.device, &self.surface_config);

        self.renderer.resize(new_size, &self.device);
        self.ui_screen.update_dims(&self.queue, new_size.width, new_size.height);

        match &mut self.game_state {
            GameState::InWorld(worldstate) => {
                if new_size.width == 0 || new_size.height == 0 {
                    return;
                }
                worldstate.camera.set_aspect_ratio(new_size.width as f32 / new_size.height as f32);
            },
            _ => {}
        }
    }

    pub fn on_window_key_press(&mut self, event: KeyEvent) {
        if self.ui_screen.key_event(&event, &mut self.ui_renderqueue.font_system) { //event consumed by ui
            return;
        }
        match &mut self.game_state {
            GameState::InWorld(worldstate) => {
                match event.physical_key {
                    PhysicalKey::Code(code) => {worldstate.camera.controller.process_keyboard_input(code, event.state);},
                    _ => {}
                };
            },
            _ => {}
        }
    }
    pub fn on_window_mouse_event(&mut self, button: MouseButton, state: ElementState) {
        if self.ui_screen.mouse_event(&button, &state, &self.mouse_position) { //event consumed by ui
            return;
        }
    }
    pub fn on_window_mouse_motion(&mut self, dx: f64, dy: f64) {
        match &mut self.game_state {
            GameState::InWorld(worldstate) => {
                worldstate.camera.controller.process_mouse_input(dx, dy);
                
            },
            _ => {}
        }
    }
}