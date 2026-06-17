/*! General utility functions and Vulkan helper abstractions. */

use {
    color_art::Color,
    egui_winit_vulkano::GuiConfig,
    glam::{
        Mat4,
        Vec2,
        Vec3
        },
    egui_file_dialog::{
        FileDialogConfig,
        OpeningMode
        },
    vulkano::{
        VulkanLibrary,
        Version,
        VulkanError,
        Validated,
        single_pass_renderpass,
        buffer::{
            AllocateBufferError,
            Buffer,
            BufferCreateInfo,
            BufferUsage,
            Subbuffer
            },
        command_buffer::{
            SubpassContents,
            RenderPassBeginInfo,
            SubpassBeginInfo,
            },
        device::{
            Device,
            DeviceFeatures
            },
        format::{
            ClearValue,
            Format
            },
        image::{
            Image,
            ImageType,
            ImageUsage,
            ImageCreateInfo,
            view::ImageView
            },
        instance::{
            InstanceCreateInfo,
            InstanceCreateFlags
            },
        memory::allocator::{
            AllocationCreateInfo,
            MemoryTypeFilter,
            StandardMemoryAllocator
            },
        pipeline::{
            DynamicState,
            GraphicsPipeline,
            PipelineLayout,
            PipelineShaderStageCreateInfo,
            graphics::{
                GraphicsPipelineCreateInfo,
                color_blend::{
                    ColorBlendAttachmentState,
                    ColorBlendState
                    },
                depth_stencil::{
                    DepthState,
                    DepthStencilState
                    },
                input_assembly::InputAssemblyState,
                multisample::MultisampleState,
                rasterization::{
                    RasterizationState,
                    PolygonMode
                    },
                vertex_input::{
                    Vertex as VertexInfo,
                    VertexInputState,
                    VertexDefinition
                    },
                viewport::{
                    Viewport,
                    ViewportState
                    }
                },
            layout::PipelineDescriptorSetLayoutCreateInfo
            },
        render_pass::{
            RenderPass,
            Subpass,
            Framebuffer,
            FramebufferCreateInfo
            },
        shader::{
            EntryPoint,
            ShaderModule
            },
        swapchain::{
            Surface,
            SwapchainCreateInfo
            }
        },
    vulkano_util::{
        context::VulkanoConfig,
        window::WindowDescriptor
        },
    winit::{
        dpi::PhysicalSize,
        event_loop::EventLoop,
        window::Icon
        },
    std::sync::Arc,
    core::ops::RangeInclusive,
    crate::{
        camera::Camera,
        consts::{
            labels::{
                APP_NAME,
                ENGINE_NAME,
                PROJECT_VERSION,
                STYLISED_APP_NAME_WITH_VERSION
                },
            config::{
                VULKAN_LAYER_IDS,
                DEPTH_CLEAR_VALUE,
                },
            ranges::{
                SIZE_CONSTRAINTS,
                Z_PLANE_RANGE_LIMIT
                }
            },
        error::{
            AppCreateError,
            AppResumeError,
            DepthImageResizeError
            },
        model::Vertex,
        render::PushConstantData,
        state::SceneConfig
        }
    };

#[cfg(windows)]
use {
    winit::platform::windows::IconExtWindows,
    crate::consts::config::{
        ICON_RESOURCE_NAME,
        ICON_SIZE
        }
    };



/** Retrieves a list of available Vulkan validation layers supported by the system. */
pub fn get_available_layers(library: &VulkanLibrary) -> Result<Box<[String]>, VulkanError> {
    let layers = library.layer_properties()?
        .map(|layer| layer.name().to_owned())
        .collect();

    Ok(layers)
    }

/** Filters the available Vulkan layers against the required layer identifiers for the application. */
fn get_enabled_layers(available_layers: &[String]) -> Option<Vec<String>> {
    VULKAN_LAYER_IDS.into_iter()
        .map(|&layer_id| {
            let layer_id = layer_id.to_owned();
            available_layers.contains(&layer_id)
                .then_some(layer_id)
            })
        .collect()
    }

/**
Constructs the configuration payload required to initialise a new Vulkan instance.

# Errors

Returns an `AppCreateError` if required windowing extensions cannot be resolved or if the requested validation layers are missing.
*/
fn create_instance_info(library: &VulkanLibrary, event_loop: &EventLoop<()>) -> Result<InstanceCreateInfo, AppCreateError> {
    let enabled_extensions = Surface::required_extensions(&event_loop)?;
    
    let available_layers = get_available_layers(&library)?;

    let enabled_layers = get_enabled_layers(&available_layers)
        .ok_or(AppCreateError::NoExtensionLayers)?;

    let instance_info = InstanceCreateInfo {
        flags: InstanceCreateFlags::ENUMERATE_PORTABILITY,
        application_name: Some(APP_NAME.to_owned()),
        application_version: PROJECT_VERSION,
        engine_name: Some(ENGINE_NAME.to_owned()),
        engine_version: PROJECT_VERSION,
        max_api_version: Some(Version::HEADER_VERSION),
        enabled_extensions,
        enabled_layers,
        .. Default::default()
        };

    Ok(instance_info)
    }

/**
Generates the top-level Vulkano configuration context based on windowing requirements and device features.

# Errors

Returns an `AppCreateError` if the underlying instance creation payload fails to build.
*/
pub fn create_vulkano_config(library: &VulkanLibrary, event_loop: &EventLoop<()>) -> Result<VulkanoConfig, AppCreateError> {
    let instance_create_info = create_instance_info(library, event_loop)?;

    let device_features = DeviceFeatures {
        fill_mode_non_solid: true,
        .. Default::default()
        };
    
    let vulkano_config = VulkanoConfig {
        instance_create_info,
        device_features,
        .. Default::default()
        };

    Ok(vulkano_config)
    }

/**
Extracts and validates the main entry point from a compiled shader module.

# Errors

Returns an `AppCreateError` if the specified shader does not contain a valid `main` entry point.
*/
pub fn get_entry_point<F>(device: Arc<Device>, load: F) -> Result<EntryPoint, AppCreateError>
where F: Fn(Arc<Device>) -> Result<Arc<ShaderModule>, Validated<VulkanError>> {
    let entry_point = load(device)?
        .entry_point("main")
        .ok_or(AppCreateError::NoShaderEntryPoint)?;
    
    Ok(entry_point)
    }

/* -------------------------------- */

/** Configures the initial properties and size constraints for the primary application window. */
pub fn create_window_descriptor() -> WindowDescriptor {
    WindowDescriptor {
        title: STYLISED_APP_NAME_WITH_VERSION.to_owned(),
        resize_constraints: SIZE_CONSTRAINTS,
        .. Default::default()
        }
    }

/** Loads the application icon from the embedded Windows resource files. */
#[cfg(windows)]
pub fn get_icon() -> Option<Icon> {
    Icon::from_resource_name(ICON_RESOURCE_NAME, Some(ICON_SIZE))
        .ok()
    }

/** Appends required image usage flags to the swapchain configuration to allow direct memory transfers. */
pub fn configure_swapchain(swapchain_info: &mut SwapchainCreateInfo) {
    swapchain_info.image_usage |= ImageUsage::TRANSFER_DST
    }

/** Checks whether the window dimensions are strictly positive, indicating it is currently safe to render to. */
pub fn is_window_drawable(window_size: PhysicalSize<u32>) -> bool {
    match window_size {
        PhysicalSize { width: 1 .., height: 1 ..} =>
            true,
        _ => false
        }  
    }

/* -------------------------------- */

/** Constructs the primary single-pass render pass containing both colour and depth attachments. */
pub fn create_render_pass(device: Arc<Device>, swapchain_format: Format) -> Result<Arc<RenderPass>, Validated<VulkanError>> {
    single_pass_renderpass!(
        device,
        attachments: {
            color: {
                format: swapchain_format,
                samples: 1,
                load_op: Clear,
                store_op: Store,
                },
            depth: {
                format: Format::D16_UNORM,
                samples: 1,
                load_op: Clear,
                store_op: DontCare,
                }
            },
        pass: {
            color: [color],
            depth_stencil: {depth}
            }
        )
    }

/** Aggregates state configurations to form a blueprint for the graphics pipeline. */
fn create_graphic_pipeline_info(
    stages: [PipelineShaderStageCreateInfo; 2],
    vertex_input_state: VertexInputState,
    subpass: Subpass,
    layout: Arc<PipelineLayout>,
    polygon_mode: PolygonMode
) -> GraphicsPipelineCreateInfo {
    let stages = stages.into_iter().collect();
    let vertex_input_state = Some(vertex_input_state);
    let color_blend_state = Some(ColorBlendState::with_attachment_states(
        subpass.num_color_attachments(),
        ColorBlendAttachmentState::default(),
        ));
    let subpass = Some(subpass.into());

    let rasterization_state = Some(RasterizationState {
        polygon_mode,
        .. Default::default()
        });
    let dynamic_state = [DynamicState::Viewport].into_iter().collect();
    let depth_stencil_state = Some(DepthStencilState {
            depth: Some(DepthState::simple()),
            .. Default::default()
            });

    GraphicsPipelineCreateInfo {
        stages,
        vertex_input_state,
        input_assembly_state: Some(InputAssemblyState::default()),
        viewport_state: Some(ViewportState::default()),
        rasterization_state,
        multisample_state: Some(MultisampleState::default()),
        color_blend_state,
        depth_stencil_state, 
        dynamic_state,
        subpass,
        .. GraphicsPipelineCreateInfo::layout(layout)
        }
    }

/**
Compiles the shaders, pipeline layout, and dynamic states into an executable graphics pipeline.

# Errors

Returns an `AppResumeError` if pipeline descriptors fail to map or if the initial render subpass is missing.
*/
pub fn create_graphic_pipeline(
    device: Arc<Device>,
    vs_entry: EntryPoint,
    fs_entry: EntryPoint,
    render_pass: Arc<RenderPass>,
    polygon_mode: PolygonMode
) -> Result<Arc<GraphicsPipeline>, AppResumeError> {
    let vertex_input_state = Vertex::per_vertex()
        .definition(&vs_entry)?;

    let stages = [vs_entry, fs_entry]
        .map(PipelineShaderStageCreateInfo::new);

    let pipeline_layout_info = PipelineDescriptorSetLayoutCreateInfo::from_stages(&stages)
        .into_pipeline_layout_create_info(Arc::clone(&device))?;

    let layout = PipelineLayout::new(Arc::clone(&device), pipeline_layout_info)?;

    let subpass = Subpass::from(render_pass, 0)
        .ok_or(AppResumeError::NoRenderSubpass)?;

    let pipeline_info = create_graphic_pipeline_info(
        stages,
        vertex_input_state,
        subpass,
        layout,
        polygon_mode
        );

    let pipeline = GraphicsPipeline::new(device, None, pipeline_info)?;

    Ok(pipeline)
    }

/* -------------------------------- */

/** Constructs the basic descriptor for a 2-D depth image attachment based on window dimensions. */
fn create_image_info(extent: [u32; 2]) -> ImageCreateInfo {
    let [width, height] = extent;

    ImageCreateInfo {
        image_type: ImageType::Dim2d,
        format: Format::D16_UNORM,
        extent: [width, height, 1],
        usage: ImageUsage::DEPTH_STENCIL_ATTACHMENT | ImageUsage::TRANSIENT_ATTACHMENT,
        ..  Default::default()
        }
    }

/**
Allocates and binds device memory for the depth buffer image used during 3D rendering.

# Errors

Returns a `DepthImageResizeError` if memory allocation or image view creation fails.
*/
pub fn create_depth_image(memory_allocator: Arc<StandardMemoryAllocator>, extent: [u32; 2]) -> Result<Arc<ImageView>, DepthImageResizeError> {
    let image_info = create_image_info(extent);

    let allocation_info = create_alloc_info(MemoryTypeFilter::PREFER_DEVICE);

    let image = Image::new(memory_allocator, image_info, allocation_info)?;

    let image_view = ImageView::new_default(image)?;

    Ok(image_view)
    }

/** Packages the swapchain and depth images into a single framebuffer configuration. */
pub fn create_framebuffer_info(image: Arc<ImageView>, depth_image: Arc<ImageView>) -> FramebufferCreateInfo {
    FramebufferCreateInfo {
        attachments: Vec::from([image, depth_image]),
        .. Default::default()
        }
    }

/** Generates a rendering viewport covering the specified window extent. */
pub fn create_viewport(extent: [u32; 2]) -> Viewport {    
    Viewport {
        extent: extent.map(|n| n as f32),
        offset: [0.0, 0.0],
        depth_range: 0.0 ..= 1.0
        }
    }

/** Defines the starting parameters and background clear colours for the render pass execution. */
pub fn create_renderpass_begin_info(background_color: Vec3, framebuffer: Arc<Framebuffer>) -> RenderPassBeginInfo {
    let Vec3 { x: r, y: g, z: b } = background_color;

    RenderPassBeginInfo {
        clear_values: Vec::from([
            Some(ClearValue::Float([r, g, b, 1.0])),
            Some(DEPTH_CLEAR_VALUE)
            ]),
        .. RenderPassBeginInfo::framebuffer(framebuffer)
        }
    }

/** Configures the initial execution state for an inline subpass sequence. */
pub fn create_subpass_begin_info() -> SubpassBeginInfo {
    SubpassBeginInfo {
        contents: SubpassContents::Inline,
        .. Default::default()
        }
    }

/* -------------------------------- */

/** Generates a basic buffer descriptor for a specified usage type. */
fn create_buffer_info(usage: BufferUsage) -> BufferCreateInfo {
    BufferCreateInfo {
        usage,
        .. Default::default()
        }
    }

/** Specifies the memory allocation preferences and hardware filtering criteria. */
fn create_alloc_info(memory_type_filter: MemoryTypeFilter) -> AllocationCreateInfo {
    AllocationCreateInfo {
        memory_type_filter,
        .. Default::default()
        }
    }

/** Allocates device-local memory and populates it with the 3-D model's vertex data. */
pub fn create_vertex_buffer(vertices: Vec<Vertex>, memory_allocator: Arc<StandardMemoryAllocator>) -> Result<Subbuffer<[Vertex]>, Validated<AllocateBufferError>> {
    let buffer_info = create_buffer_info(BufferUsage::VERTEX_BUFFER);
    let allocation_info = create_alloc_info(MemoryTypeFilter::PREFER_DEVICE | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE);

    Buffer::from_iter(memory_allocator, buffer_info, allocation_info, vertices)
    }

/** Allocates device-local memory and populates it with the 3-D model's index data. */
pub fn create_index_buffer(indices: Vec<u32>, memory_allocator: Arc<StandardMemoryAllocator>) -> Result<Subbuffer<[u32]>, Validated<AllocateBufferError>> {
    let buffer_info = create_buffer_info(BufferUsage::INDEX_BUFFER);
    let allocation_info = create_alloc_info(MemoryTypeFilter::PREFER_DEVICE | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE);

    Buffer::from_iter(memory_allocator, buffer_info, allocation_info, indices)
    }

/* -------------------------------- */

/** Converts a colour object into a three-dimensional vector with normalised red, green, and blue channels. */
pub fn colour_into_vec3(value: Color) -> Vec3 {
    const MAX: f32 = u8::MAX as f32;

    let color_values = [value.red(), value.green(), value.blue()]
        .map(|n| n as f32 / MAX);

    Vec3::from_array(color_values)
    }

/** Converts spatial yaw and pitch angles into a normalised directional 3D vector, padded to a 4-element array for memory alignment. */
fn vec2_into_array(value: Vec2) -> [f32; 4] {
    let (yaw_rad, pitch_rad) = (
        value.x.to_radians(),
        value.y.to_radians()
        );

    let (x, y, z) = (
        pitch_rad.cos() * yaw_rad.sin(),
        pitch_rad.sin(),
        pitch_rad.cos() * yaw_rad.cos()
        );

    [x, y, z, 0.0]
    }

/** Computes the global model-view-projection matrix and packages the lighting parameters for the active shader. */
pub fn create_push_constants(aspect_ratio: f32, camera: &Camera, scene_config: &SceneConfig) -> PushConstantData {
    /* Projection calculation */
    let projection = camera.projection_matrix(aspect_ratio);

    /* View calculation */
    let view = camera.view_matrix();

    /* Model calculation */
    let model = Mat4::IDENTITY;
    // let model = Mat4::from_scale_rotation_translation(
    //     Vec3::splat(0.2),
    //     Quat::from_rotation_y(TIME.elapsed().as_secs_f32()),
    //     Vec3::ZERO
    //     );
    // let model = Mat4::from_rotation_y(scene_config.time_from_start() * scene_config.rotation_speed);

    /*
    This operation follows logic of matrix operations of:
    translation * angle * scale
    */
    let model_view_projection_matrix = projection * view * model;

    let light_dir = vec2_into_array(scene_config.light_direction);

    let [r, g, b] = scene_config.model_colour.to_array();
    let model_colour = [r, g, b, 1.0];

    PushConstantData {
        mvp: model_view_projection_matrix.to_cols_array_2d(),
        model: model.to_cols_array_2d(),
        light_dir,
        model_colour
        }
    }

/** Calculates dynamically constrained ranges for the near and far clipping planes to prevent overlap or gimbal locks. */
pub const fn get_z_plane_ranges(z_near:f32, z_far: f32) -> (RangeInclusive<f32>, RangeInclusive<f32>) {
    let near_range = *Z_PLANE_RANGE_LIMIT.start() ..= z_far.next_down();
    let far_range = z_near.next_up() ..= *Z_PLANE_RANGE_LIMIT.end();

    ( near_range, far_range )
    }

/* -------------------------------- */

/** Defines the baseline behaviour for the immediate mode user interface overlay. */
pub fn create_gui_config() -> GuiConfig {
    GuiConfig {
        is_overlay: true,
        .. Default::default()
        }
    }

/** Initialises the layout and filter configuration for the native file picker dialogue. */
pub fn create_gui_filedialog_config() -> FileDialogConfig {
    FileDialogConfig {
        opening_mode: OpeningMode::LastPickedDir,
        as_modal: true,
        default_file_name: "file".to_owned(),
        allow_file_overwrite: false,
        canonicalize_paths: true,
        .. Default::default()
        }.add_file_filter_extensions("OBJ files", vec!["obj"])
    }