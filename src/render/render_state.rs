use {
    vulkano::{
        device::Device,
        image::view::ImageView,
        memory::allocator::StandardMemoryAllocator,
        pipeline::{
            GraphicsPipeline,
            graphics::rasterization::PolygonMode
            },
        render_pass::RenderPass,
        shader::EntryPoint
        },
    vulkano_util::renderer::VulkanoWindowRenderer,
    std::sync::Arc,
    crate::{
        error::{
            AppResumeError,
            DepthImageResizeError
            },
        render::RenderMode,
        utils::{
            create_render_pass,
            create_graphic_pipeline,
            create_depth_image
            }
        }
    };


/**
Encapsulates the core Vulkan rendering state and resources.

This structure manages the render pass, graphics pipelines for different rendering modes, and the depth buffer.
*/
pub struct RenderState {
    /** Currently active rendering mode. */
    render_mode: RenderMode,
    /** Vulkan render pass detailing the attachments and subpasses. */
    render_pass: Arc<RenderPass>,
    /** Graphics pipeline configured for solid polygon rendering. */
    pipeline_polygone: Arc<GraphicsPipeline>,
    /** Graphics pipeline configured for wireframe rendering. */
    pipeline_wireframe: Arc<GraphicsPipeline>,
    /** Image view serving as the depth buffer attachment. */
    depth_image: Arc<ImageView>
    }

impl RenderState {
    /**
    Initialises a new rendering state with its associated Vulkan resources.
    
    # Errors
    
    Returns an `AppResumeError` if the creation of the render pass, pipelines, or the depth image fails.
    */
    pub fn new(
        device: Arc<Device>,
        memory_allocator: Arc<StandardMemoryAllocator>,
        renderer: &VulkanoWindowRenderer,
        vertex_shader: EntryPoint,
        fragment_shader: EntryPoint
    ) -> Result<Self, AppResumeError> {
        let render_pass = create_render_pass(
            Arc::clone(&device),
            renderer.swapchain_format()
            )?;

        let pipeline_polygone = create_graphic_pipeline(
            Arc::clone(&device),
            vertex_shader.clone(),
            fragment_shader.clone(),
            Arc::clone(&render_pass),
            PolygonMode::Fill
            )?;

        let pipeline_wireframe = create_graphic_pipeline(
            Arc::clone(&device),
            vertex_shader,
            fragment_shader,
            Arc::clone(&render_pass),
            PolygonMode::Line
            )?;

        let depth_image = create_depth_image(
            memory_allocator,
            renderer.window().inner_size().into()
            )?;

        Ok(Self {
            render_mode: RenderMode::default(),
            render_pass,
            pipeline_polygone,
            pipeline_wireframe,
            depth_image
            })
        }

    /**
    Recreates the depth buffer image to match a new set of dimensions.
    
    This is typically called during a window resize event to ensure the depth buffer size matches the new swapchain extents.
    
    # Errors
    
    Returns a `DepthImageResizeError` if the new depth image allocation or view creation fails.
    */
    pub fn try_recreate_depth_image(&mut self, memory_allocator: Arc<StandardMemoryAllocator>, extent: [u32; 2]) -> Result<(), DepthImageResizeError> {
        self.depth_image = create_depth_image(memory_allocator, extent)?;

        Ok(()) 
        }

    /** Retrieves a mutable reference to the current rendering mode. */
    pub const fn render_mode_mut(&mut self) -> &mut RenderMode {
        &mut self.render_mode
        }

    /** Retrieves a reference to the underlying render pass. */
    pub const fn render_pass(&self) -> &Arc<RenderPass> {
        &self.render_pass
        }

    /** Retrieves a reference to the graphics pipeline corresponding to the currently selected rendering mode. */
    pub const fn pipeline(&self) -> &Arc<GraphicsPipeline> {
        match self.render_mode {
            RenderMode::Polygone =>
                &self.pipeline_polygone,
            RenderMode::Wireframe =>
                &self.pipeline_wireframe
            }
        }

    /** Retrieves a reference to the current depth buffer image view. */
    pub const fn depth_image(&self) -> &Arc<ImageView> {
        &self.depth_image
        }
    }