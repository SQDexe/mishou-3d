use {
    vulkano::{
        Validated,
        LoadingError,
        VulkanError,
        ValidationError,
        buffer::AllocateBufferError,
        command_buffer::CommandBufferExecError,
        image::AllocateImageError,
        pipeline::layout::IntoPipelineLayoutCreateInfoError,
        // swapchain::FromWindowError
        },
    thiserror::Error,
    winit::{
        error::{
            EventLoopError,
            // OsError
            },
        raw_window_handle::HandleError
        },
    crate::error::other_error::DepthImageResizeError
    };



/** Enumeration of errors that can occur during the application's initialisation phase. */
#[derive(Debug, Error)]
pub enum AppCreateError {
    /** Error stemming from the windowing event loop creation. */
    #[error("Failed to initialise window event loop: {0}")]
    EventLoop(#[from] EventLoopError),
    /** Error encountered whilst loading the Vulkan library. */
    #[error("Failed to load Vulkan library: {0}")]
    Loading(#[from] LoadingError),
    /** Error when retrieving raw window or display handles. */
    #[error("Failed to acquire raw window handle: {0}")]
    Handle(#[from] HandleError),
    /** Core Vulkan API execution error. */
    #[error("Vulkan API error during initialisation: {0}")]
    Vulkan(#[from] VulkanError),
    /** Failure due to the absence of required Vulkan extension layers. */
    #[error("No compatible Vulkan extension layers found")]
    NoExtensionLayers,
    /** Vulkan error caught and wrapped by the validation layers. */
    #[error("Validated Vulkan error during initialisation: {0}")]
    ValidatedVulkan(#[from] Validated<VulkanError>),
    /** Failure due to a missing main entry point in the compiled shader modules. */
    #[error("No entry point function found in the compiled shader module")]
    NoShaderEntryPoint,
    /** Error encountered during the allocation of a Vulkan memory buffer. */
    #[error("Failed to allocate Vulkan memory buffer: {0}")]
    BufferAlloc(#[from] Validated<AllocateBufferError>)
}

/** Enumeration of errors that can occur when resuming the application or reconstructing the render state. */
#[derive(Debug, Error)]
pub enum AppResumeError {
    /** Failure due to a missing renderer instance during the resumption process. */
    #[error("Renderer object is missing or uninitialised")]
    NoRenderer,
    /** Vulkan state or parameter validation failure. */
    #[error("Vulkan validation error during state resumption: {0}")]
    Validation(#[from] Box<ValidationError>),
    /** Error whilst generating the pipeline layout creation information. */
    #[error("Failed to generate pipeline layout creation information: {0}")]
    PipelineLayoutInfo(#[from] IntoPipelineLayoutCreateInfoError),
    /** Vulkan execution error intercepted by validation layers. */
    #[error("Validated Vulkan error during state resumption: {0}")]
    ValidatedVulkan(#[from] Validated<VulkanError>),
    /** Failure due to the absence of a required subpass within the render pass. */
    #[error("Required render subpass is missing from the render pass")]
    NoRenderSubpass,
    /** Error encountered during the allocation of a Vulkan image memory block. */
    #[error("Failed to allocate Vulkan image memory during resumption: {0}")]
    ValidatedImageAlloc(#[from] Validated<AllocateImageError>)
    }

impl From<DepthImageResizeError> for AppResumeError {
    fn from(value: DepthImageResizeError) -> Self {
        match value {
            DepthImageResizeError::ValidatedImageAlloc(error) =>
                Self::ValidatedImageAlloc(error),
            DepthImageResizeError::ValidatedVulkan(error) =>
                Self::ValidatedVulkan(error)
            }
        }
    }

/** Enumeration of errors that can occur during the main execution loop and continuous rendering. */
#[derive(Debug, Error)]
pub enum AppRuntimeError {
    /** Standard Vulkan execution error during runtime operations. */
    #[error("Vulkan API execution error during runtime operations: {0}")]
    Vulkan(#[from] VulkanError),
    /** Vulkan runtime error flagged by validation checks. */
    #[error("Validated Vulkan error during runtime operations: {0}")]
    ValidatedVulkan(#[from] Validated<VulkanError>),
    /** Direct validation layer failure during the render loop. */
    #[error("Vulkan validation layer failure during runtime operations: {0}")]
    Validation(#[from] Box<ValidationError>),
    /** Error whilst attempting to submit and execute a Vulkan command buffer. */
    #[error("Failed to execute Vulkan command buffer: {0}")]
    CommandBufferExec(#[from] CommandBufferExecError),
    /** Error encountered dynamically allocating image memory at runtime. */
    #[error("Failed to allocate Vulkan image memory dynamically: {0}")]
    ValidatedImageAlloc(#[from] Validated<AllocateImageError>)
    }

impl From<DepthImageResizeError> for AppRuntimeError {
    fn from(value: DepthImageResizeError) -> Self {
        match value {
            DepthImageResizeError::ValidatedImageAlloc(error) =>
                Self::ValidatedImageAlloc(error),
            DepthImageResizeError::ValidatedVulkan(error) =>
                Self::ValidatedVulkan(error)
            }
        }
    }