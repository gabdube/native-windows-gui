extern crate native_windows_gui as nwg;
extern crate native_windows_derive as nwd;
extern crate nalgebra_glm as glm;

use nwd::NwgUi;
use nwg::NativeUi;
use nwg::stretch::{style::{*, Dimension::*}, geometry::*};
use std::{slice, mem, time::Duration, cell::RefCell, borrow::Cow, ops::Range};
use core::num::NonZeroU64;

mod glb;

const MODELS: &'static [&'static str; 3] = &[
    "box.glb",
    "suzanne.glb",
    "teapot.glb"
];

const MATERIALS: &'static [&'static str; 5] = &[
    "Green Plastic",
    "Poopy Bronze",
    "Grape Juice",
    "Darker Grape Juice",
    "Snow",
];

/// A 3D model loaded in GPU memory
pub struct Model {
    buffer: wgpu::Buffer,
    
    index: Range<wgpu::BufferAddress>,
    positions: Range<wgpu::BufferAddress>,
    normals: Range<wgpu::BufferAddress>,

    index_count: u32,
}

/// Uniforms buffer & related gpu resources
pub type Vec4 = [f32; 4];
pub type Mat4 = glm::TMat4<f32>;

#[repr(C)]
#[derive(Copy, Clone, Default)]
struct PhongView {
    mvp: Mat4,
    model: Mat4,
    normal: Mat4,
}

#[repr(C)]
#[derive(Copy, Clone, Default)]
struct PhongMaterial {
    color: Vec4,
    spec: Vec4, // [0]: spec strength / [1] spec shininess
}

#[repr(C)]
#[derive(Copy, Clone, Default)]
struct PhongLight {
    position: Vec4,
    color: Vec4,
    view_pos: Vec4,
}


struct Uniforms {
    buffer: wgpu::Buffer,
    
    view_offset: wgpu::BufferAddress,
    view: PhongView,

    material_offset: wgpu::BufferAddress,
    material: PhongMaterial,

    light_offset: wgpu::BufferAddress,
    light: PhongLight,

    main_bind_group: wgpu::BindGroup,
    light_bind_group: wgpu::BindGroup,
}

#[derive(Default)]
struct IoState {
    dragging_offset: (i32, i32),
    dragging_left: bool,
    dragging_right: bool,
}


#[allow(dead_code)]
/// Depth attachment in the render pass
struct DepthTexture {
    texture: wgpu::Texture,
    view: wgpu::TextureView,
    sampler: wgpu::Sampler,
}

/// Render pipeline & dependencies
#[allow(dead_code)]
struct CanvasRender {
    shader_vert: wgpu::ShaderModule,
    shader_frag: wgpu::ShaderModule,

    depth_attachment: DepthTexture,

    main_layout: wgpu::BindGroupLayout,
    light_layout: wgpu::BindGroupLayout,
    pipeline_layout: wgpu::PipelineLayout,
    render_pipeline: wgpu::RenderPipeline,
}

/// Rendering context
#[allow(dead_code)]
struct CanvasData {
    instance: wgpu::Instance,
    surface: wgpu::Surface,
    adapter: wgpu::Adapter,
    device: wgpu::Device,
    queue: wgpu::Queue,

    swapchain_description: wgpu::SwapChainDescriptor,
    swapchain: wgpu::SwapChain,

    uniforms: Uniforms,
    render: CanvasRender,

    materials: Vec<PhongMaterial>,
    models: Vec<Model>,

    io: IoState,

    current_material: usize,
    current_model: usize,
    model_rotation: [f32; 2],
    light_position: [f32; 4],
}


/// GUI application
#[derive(Default, NwgUi)]
pub struct CanvasTest {
    canvas_data: RefCell<Option<CanvasData>>,

    #[nwg_control(size: (1000, 800), center: true, title: "WGPU canvas", flags: "MAIN_WINDOW")]
    #[nwg_events( OnInit: [CanvasTest::init_default_scene], OnWindowClose: [nwg::stop_thread_dispatch()] )]
    window: nwg::Window,

    #[nwg_layout(parent: window, min_size: [500, 200])]
    layout: nwg::GridLayout,

    #[nwg_control(parent: Some(&data.window))]
    #[nwg_events(
        OnMouseMove: [CanvasTest::mouse_actions(SELF, EVT)],
        OnMousePress: [CanvasTest::mouse_actions(SELF, EVT)],
        OnResize: [CanvasTest::resize_canvas]
    )]
    #[nwg_layout_item(layout: layout, col: 0, row: 0, col_span: 3)]
    canvas: nwg::ExternCanvas,

    #[nwg_control(parent: window, interval: Duration::from_millis(1000/60))]
    #[nwg_events(OnTimerTick: [CanvasTest::animate])]
    timer: nwg::AnimationTimer,

    #[nwg_control(parent: window)]
    #[nwg_layout_item(layout: layout, col: 0, col: 3)]
    options_frame: nwg::Frame,

    #[nwg_layout(
        parent: options_frame,
        auto_size: false,
        flex_direction: FlexDirection::Column,
        padding: Rect { start: Points(5.0), end: Points(5.0), top: Points(5.0), bottom: Points(5.0) }
    )]
    options_layout: nwg::FlexboxLayout,

    #[nwg_control(parent: options_frame, text: "Animate")]
    #[nwg_events(OnButtonClick: [CanvasTest::update_anim])]
    #[nwg_layout_item(layout: options_layout, size: Size { width: Auto, height: Points(30.0) })]
    animate_check: nwg::CheckBox,

    #[nwg_control(parent: options_frame, text: "Models:")]
    #[nwg_layout_item(layout: options_layout, size: Size { width: Auto, height: Points(30.0) })]
    label1: nwg::Label,

    #[nwg_control(parent: options_frame, selected_index: Some(0), collection: MODELS.to_vec())]
    #[nwg_events(OnListBoxSelect: [CanvasTest::change_model])]
    #[nwg_layout_item(layout: options_layout, size: Size { width: Auto, height: Points(200.0) })]
    model_list: nwg::ListBox<&'static str>,

    #[nwg_control(parent: options_frame, text: "Materials:")]
    #[nwg_layout_item(layout: options_layout, size: Size { width: Auto, height: Points(30.0) })]
    label2: nwg::Label,

    #[nwg_control(parent: options_frame, selected_index: Some(0), collection: MATERIALS.to_vec())]
    #[nwg_events(OnListBoxSelect: [CanvasTest::change_material])]
    #[nwg_layout_item(layout: options_layout, size: Size { width: Auto, height: Points(200.0) })]
    material_list: nwg::ListBox<&'static str>,
}

impl CanvasTest {
    pub const DEPTH_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Depth32Float;

    //
    // WGPU initialization
    //

    fn init_depth_texture(&self, device: &wgpu::Device, swapchain_desc: &wgpu::SwapChainDescriptor) -> DepthTexture {
        let size = wgpu::Extent3d {
            width: swapchain_desc.width,
            height: swapchain_desc.height,
            depth: 1,
        };

        let desc = wgpu::TextureDescriptor {
            label: Some("depth_attachment"),
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: Self::DEPTH_FORMAT,
            usage: wgpu::TextureUsage::RENDER_ATTACHMENT
                | wgpu::TextureUsage::SAMPLED,
        };
        let texture = device.create_texture(&desc);

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let sampler = device.create_sampler(
            &wgpu::SamplerDescriptor { // 4.
                address_mode_u: wgpu::AddressMode::ClampToEdge,
                address_mode_v: wgpu::AddressMode::ClampToEdge,
                address_mode_w: wgpu::AddressMode::ClampToEdge,
                mag_filter: wgpu::FilterMode::Linear,
                min_filter: wgpu::FilterMode::Linear,
                mipmap_filter: wgpu::FilterMode::Nearest,
                compare: Some(wgpu::CompareFunction::LessEqual),
                lod_min_clamp: -100.0,
                lod_max_clamp: 100.0,
                ..Default::default()
            }
        );

        DepthTexture {
            texture,
            view,
            sampler,
        }
    }

    fn init_render(&self, device: &wgpu::Device, swapchain_format: wgpu::TextureFormat, swapchain_desc: &wgpu::SwapChainDescriptor) -> CanvasRender {
        //
        // Depth attachment
        //
        let depth_attachment = self.init_depth_texture(device, swapchain_desc);
        
        //
        // Shaders
        // Note: ShaderFlags::VALIDATION will cause a segfault
        //
        
        let vert_src = include_bytes!("phong.vert.spv");
        let (_, vert_aligned, _) = unsafe { vert_src.align_to::<u32>() };
        let frag_src = include_bytes!("phong.frag.spv");
        let (_, frag_aligned, _) = unsafe { frag_src.align_to::<u32>() };

        let shader_vert = device.create_shader_module(&wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::SpirV(Cow::Borrowed(vert_aligned)),
            flags: wgpu::ShaderFlags::empty(),
        });

        let shader_frag = device.create_shader_module(&wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::SpirV(Cow::Borrowed(frag_aligned)),
            flags: wgpu::ShaderFlags::empty(),
        });

        //
        // Pipeline layout
        //

        let main_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: None,
            entries: &[
                // View matrices
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStage::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: wgpu::BufferSize::new(64*3),
                    },
                    count: None,
                },
                // Material data
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStage::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: wgpu::BufferSize::new(32),
                    },
                    count: None,
                },
            ],
        });

        let light_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: None,
            entries: &[
                // Light data
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStage::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: wgpu::BufferSize::new(48),
                    },
                    count: None,
                },
            ],
        });
    
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[
                &main_layout,
                &light_layout
            ],
            push_constant_ranges: &[],
        });

        //
        // Pipelines
        //
        let vertex_position = wgpu::VertexBufferLayout {
            array_stride: 12,
            step_mode: wgpu::InputStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float3,
                    offset: 0,
                    shader_location: 0,
                },
            ],
        };

        let vertex_normal = wgpu::VertexBufferLayout {
            array_stride: 12,
            step_mode: wgpu::InputStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float3,
                    offset: 0,
                    shader_location: 1,
                },
            ],
        };

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader_vert,
                entry_point: "main",
                buffers: &[vertex_position, vertex_normal],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader_frag,
                entry_point: "main",
                targets: &[swapchain_format.into()],
            }),
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: Some(wgpu::DepthStencilState {
                format: Self::DEPTH_FORMAT,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Less,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
                clamp_depth: false,
            }),
            multisample: wgpu::MultisampleState::default(),
        });

        CanvasRender {
            shader_vert,
            shader_frag,
            main_layout,
            light_layout,
            pipeline_layout,

            depth_attachment,

            render_pipeline
        }
    }

    fn init_models(&self, device: &wgpu::Device, queue: &wgpu::Queue) -> Vec<Model> {
        let mut models = Vec::with_capacity(MODELS.len());
        for name in MODELS.iter() {
            let path = format!("./models/{}", name);
            let file = glb::GlbFile::open(path).expect("Failed to open model file");
            let mesh = file.simple_mesh_by_index(0)
                .expect("Failed to fetch find mesh")
                .expect("Failed to fetch find mesh");
            
            // Load the mesh data
            let acc_indices = file.accessor_data(mesh.indices).unwrap();
            let acc_positions = file.accessor_data(mesh.positions).unwrap();
            let acc_normals = file.accessor_data(mesh.normals.unwrap()).unwrap();

            let mut index = 0..0;
            let mut positions = 0..0;
            let mut normals = 0..0;

            let accessors = [&acc_indices, &acc_positions, &acc_normals];
            let mut buffer_ranges = [&mut index, &mut positions, &mut normals];
            let mut buffer_offset = 0;
            for (&acc, range) in accessors.iter().zip(buffer_ranges.iter_mut()) {
                let start = align(buffer_offset, acc.component_ty.size() as _);
                let stop = start + (acc.data.len() as wgpu::BufferAddress);
                **range = start..stop;
                
                buffer_offset = stop;
            }

            // Create and fill the buffer
            let buffer = device.create_buffer(&wgpu::BufferDescriptor {
                label: None,
                size: buffer_offset,
                usage: wgpu::BufferUsage::VERTEX | wgpu::BufferUsage::INDEX | wgpu::BufferUsage::COPY_DST,
                mapped_at_creation: false
            });

            for (acc, range) in accessors.iter().zip(buffer_ranges.iter()) {
                queue.write_buffer(&buffer, range.start, acc.data);
            }

            // Save the model
            models.push(Model {
                buffer,
                index,
                positions,
                normals,
                index_count: acc_indices.component_count,
            })
        }

        models
    }

    fn init_uniforms(&self, device: &wgpu::Device, render: &CanvasRender) -> Uniforms {
        // There's no way to get the minimum uniform buffer offset aligment with wgpu, so we use 256
        let uniform_buffer_aligment = 256;
        
        //
        // Buffer
        //
        let view = PhongView::default();
        let view_size = mem::size_of::<PhongView>() as wgpu::BufferAddress;
        let mut view_offset = 0;

        let light = PhongLight { 
            color: [1.0, 1.0, 1.0, 0.1],
            position: [0.0, 0.0, 0.0, 0.0],
            view_pos: [0.0, 0.0, 4.0, 0.0],
        };
        let light_size = mem::size_of::<PhongView>() as wgpu::BufferAddress;
        let mut light_offset = 0;

        let material = PhongMaterial::default();
        let material_size = mem::size_of::<PhongMaterial>() as wgpu::BufferAddress;
        let mut material_offset = 0;

        let mut total_offset = 0;
        for (&size, offset) in [view_size, light_size, material_size].iter().zip([&mut view_offset, &mut light_offset, &mut material_offset].iter_mut()) {
            let aligned = align(total_offset, uniform_buffer_aligment);
            **offset = aligned;
            total_offset = aligned + size;
        }

        let buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: total_offset,
            usage: wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
            mapped_at_creation: false
        });

        //
        // Bind group
        //
        let main_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &render.main_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::Buffer {
                        buffer: &buffer,
                        offset: view_offset,
                        size: unsafe { Some(NonZeroU64::new_unchecked(view_size)) },
                    },
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Buffer {
                        buffer: &buffer,
                        offset: material_offset,
                        size: unsafe { Some(NonZeroU64::new_unchecked(material_size)) },
                    },
                }
            ],
            label: Some("main_bind_group"),
        });

        let light_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &render.light_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::Buffer {
                        buffer: &buffer,
                        offset: light_offset,
                        size: unsafe { Some(NonZeroU64::new_unchecked(light_size)) },
                    },
                }
            ],
            label: Some("light_bind_group"),
        });
        
        Uniforms {
            buffer,

            view_offset,
            view,

            light_offset,
            light,

            material_offset,
            material,

            main_bind_group,
            light_bind_group,
        }
    }

    fn init_materials(&self) -> Vec<PhongMaterial> {
        vec![
            PhongMaterial { color: [0.1, 0.8, 0.0, 0.0], spec: [0.8, 64.0, 0.0, 0.0] },
            PhongMaterial { color: [0.85, 0.25, 0.0, 0.0], spec: [0.8, 1.0, 0.0, 0.0] },
            PhongMaterial { color: [0.2, 0.2, 0.7, 0.0], spec: [0.5, 32.0, 0.0, 0.0] },
            PhongMaterial { color: [0.05, 0.05, 0.16, 0.0], spec: [4.0, 128.0, 0.0, 0.0] },
            PhongMaterial { color: [1.00, 1.00, 1.00, 0.0], spec: [0.3, 64.0, 0.0, 0.0] },
        ]
    }

    async fn init_wgpu(&self, width: u32, height: u32) -> CanvasData {
        let instance = wgpu::Instance::new(wgpu::BackendBit::VULKAN);
        let surface = unsafe { instance.create_surface(&self.canvas) }; // It's that easy!

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
            })
            .await
            .expect("No suitable GPU adapters found on the system!");

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits::default(),
                },
                None,
            )
            .await
            .expect("Unable to find a suitable GPU adapter!");

        let swapchain_format = adapter.get_swap_chain_preferred_format(&surface);
        let swapchain_description = wgpu::SwapChainDescriptor {
            usage: wgpu::TextureUsage::RENDER_ATTACHMENT,
            format: swapchain_format,
            width: width,
            height: height,
            present_mode: wgpu::PresentMode::Mailbox,
        };
    
        let swapchain = device.create_swap_chain(&surface, &swapchain_description);
        let render = self.init_render(&device, swapchain_format, &swapchain_description);
        let models = self.init_models(&device, &queue);
        let uniforms = self.init_uniforms(&device, &render);
        let materials = self.init_materials();

        CanvasData {
            instance,
            surface,
            adapter,
            device,
            queue,

            swapchain_description,
            swapchain,

            uniforms,
            render,

            materials,
            models,

            io: IoState::default(),
        
            current_material: 0,
            current_model: 0,
            light_position: [0.0, 0.0, 4.0, 0.0],
            model_rotation: [0.0, 0.0],
        }
    }

    //
    // Render functions
    //

    fn init_default_scene(&self) {
        let (width, height) = self.canvas.size();
        let data = pollster::block_on(self.init_wgpu(width, height));
        *self.canvas_data.borrow_mut() = Some(data);
        
        self.update_uniforms();
        self.render();
        
        self.window.set_visible(true);
    }

    fn update_uniforms(&self) {
        use glm::vec3;

        let mut canvas_data_op = self.canvas_data.borrow_mut();
        let data = match canvas_data_op.as_mut() {
            Some(data) => data,
            None => { return; }
        };

        let (width, height) = self.canvas.size();
        let (width, height) = (width as f32, height as f32);

        let uniforms = &mut data.uniforms;

        // MVP
        let proj: Mat4 = glm::perspective_zo(width / height, (60.0f32).to_radians(), 0.1, 10.0);
        let view: Mat4 = glm::look_at_rh(&vec3(0.0, 0.0, 4.0), &vec3(0.0, 0.0, 0.0), &vec3(0.0, 1.0, 0.0));
        let model: Mat4 = glm::rotate_y(&glm::rotate_x(&glm::identity(), data.model_rotation[0]), data.model_rotation[1]);
        
        let ubo: &mut PhongView = &mut uniforms.view;
        ubo.mvp = proj*view*model;
        ubo.model = model;
        ubo.normal = glm::transpose(&glm::inverse(&ubo.model));

        data.queue.write_buffer(&uniforms.buffer, uniforms.view_offset, slice_as_bytes(slice::from_ref(&uniforms.view)));

        // Material
        uniforms.material = data.materials[data.current_material];
        data.queue.write_buffer(&uniforms.buffer, uniforms.material_offset, slice_as_bytes(slice::from_ref(&uniforms.material)));

        // Light
        uniforms.light.position = data.light_position;
        data.queue.write_buffer(&uniforms.buffer, uniforms.light_offset, slice_as_bytes(slice::from_ref(&uniforms.light)));
    }

    fn render(&self) {
        let mut canvas_data_op = self.canvas_data.borrow_mut();
        let data = match canvas_data_op.as_mut() {
            Some(data) => data,
            None => { return; }
        };

        let frame = data.swapchain
            .get_current_frame()
            .expect("Failed to acquire next swap chain texture")
            .output;

        let mut encoder =
            data.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                    attachment: &frame.view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color { r: 0.10, g: 0.03, b: 0.03, a: 1.0 }),
                        store: true,
                    },
                }],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachmentDescriptor {
                    attachment: &data.render.depth_attachment.view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: true,
                    }),
                    stencil_ops: None,
                }),
            });

            pass.set_pipeline(&data.render.render_pipeline);

            let uniforms = &data.uniforms;
            let model = &data.models[data.current_model];
            let buffer = &model.buffer;

            pass.set_index_buffer(buffer.slice(model.index.clone()), wgpu::IndexFormat::Uint16);
            pass.set_vertex_buffer(0, buffer.slice(model.positions.clone()));
            pass.set_vertex_buffer(1, buffer.slice(model.normals.clone()));
            pass.set_bind_group(0, &uniforms.main_bind_group, &[]);
            pass.set_bind_group(1, &uniforms.light_bind_group, &[]);
            pass.draw_indexed(0..model.index_count, 0, 0..1);
        }

        data.queue.submit(Some(encoder.finish()));
    }

    //
    // native-windows-gui callbacks
    //

    fn resize_canvas(&self) {
        let mut canvas_data_op = self.canvas_data.borrow_mut();
        let data = match canvas_data_op.as_mut() {
            Some(data) => data,
            None => { return; }
        };

        let (width, height) = self.canvas.size();
        data.swapchain_description.width = width;
        data.swapchain_description.height = height;
        data.swapchain = data.device.create_swap_chain(&data.surface, &data.swapchain_description);

        data.render.depth_attachment = self.init_depth_texture(&data.device, &data.swapchain_description);

        drop(canvas_data_op);

        self.update_uniforms();
        self.render();
    }

    fn update_anim(&self) {
        let checked = self.animate_check.check_state();
        match checked {
            nwg::CheckBoxState::Checked => { self.timer.start(); },
            nwg::CheckBoxState::Unchecked => { self.timer.stop(); },
            _ => {  },
        }
    }

    fn animate(&self) {
        let mut canvas_data_op = self.canvas_data.borrow_mut();
        let data = match canvas_data_op.as_mut() {
            Some(data) => data,
            None => { return; }
        };

        data.model_rotation[1] -= 0.008;

        drop(canvas_data_op);

        self.update_uniforms();
        self.render();
    }

    fn change_model(&self) {
        let mut canvas_data_op = self.canvas_data.borrow_mut();
        let data = match canvas_data_op.as_mut() {
            Some(data) => data,
            None => { return; }
        };

        data.current_model = self.model_list.selection().unwrap_or(0);

        drop(canvas_data_op);

        self.render();
    }

    fn change_material(&self) {
        let mut canvas_data_op = self.canvas_data.borrow_mut();
        let data = match canvas_data_op.as_mut() {
            Some(data) => data,
            None => { return; }
        };

        data.current_material = self.material_list.selection().unwrap_or(0);
        
        drop(canvas_data_op);
        
        self.update_uniforms();
        self.render();
    }

    fn mouse_actions(&self, evt: nwg::Event) {
        let mut canvas_data_op = self.canvas_data.borrow_mut();
        let data = match canvas_data_op.as_mut() {
            Some(data) => data,
            None => { return; }
        };

        let io = &mut data.io;

        match evt {
            nwg::Event::OnMouseMove => {
                if !io.dragging_left && !io.dragging_right {
                    return;
                }

                let (offset_x, offset_y) = io.dragging_offset;
                let (x, y) = nwg::GlobalCursor::local_position(&self.canvas, None);
                let (delta_x, delta_y) = (x-offset_x, y-offset_y);

                if io.dragging_left {
                    data.model_rotation[0] += (delta_y as f32) * 0.004;
                    data.model_rotation[1] += (delta_x as f32) * 0.004;
                } else if io.dragging_right {
                    data.light_position[0] += (delta_x as f32) * 0.03;
                    data.light_position[1] += (delta_y as f32) * -0.03;
                }

                io.dragging_offset = (x, y);

                drop(canvas_data_op);
                self.update_uniforms();
                self.render();
            },
            nwg::Event::OnMousePress(btn) => match btn {
                nwg::MousePressEvent::MousePressLeftDown => { 
                    io.dragging_left = true;
                    io.dragging_right = false; 
                    io.dragging_offset = nwg::GlobalCursor::local_position(&self.canvas, None);
                },
                nwg::MousePressEvent::MousePressLeftUp => { 
                    io.dragging_left = false;
                },
                nwg::MousePressEvent::MousePressRightDown => { 
                    io.dragging_right = true;
                    io.dragging_left = false;
                    io.dragging_offset = nwg::GlobalCursor::local_position(&self.canvas, None);
                },
                nwg::MousePressEvent::MousePressRightUp => { 
                    io.dragging_right = false; 
                }
            },
            _ => unreachable!()
        }

    }

}

pub fn align(addr: wgpu::BufferAddress, align: wgpu::BufferAddress) -> wgpu::BufferAddress {
    (((addr as isize) + ((align as isize) - 1)) & -(align as isize)) as wgpu::BufferAddress
}

pub fn slice_as_bytes<'a, D: Copy>(data: &'a [D]) -> &'a [u8] {
    unsafe {
        data.align_to().1
    }
}


fn main() {
    nwg::init().expect("Failed to init Native Windows GUI");

    let mut font = nwg::Font::default(); 
    nwg::Font::builder()
        .family("Segoe UI")
        .size(20)
        .build(&mut font)
        .expect("Failed to create default font");

    nwg::Font::set_global_default(Some(font));

    let _app = CanvasTest::build_ui(Default::default())
        .expect("Failed to build UI");

    nwg::dispatch_thread_events();
}
