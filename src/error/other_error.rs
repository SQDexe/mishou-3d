use {
    vulkano::{
        Validated,
        VulkanError,
        buffer::AllocateBufferError,
        image::AllocateImageError
        },
    thiserror::Error,
    tobj::LoadError
    };



/** Error indicating that a model loading operation is already in progress. */
#[derive(Debug, Clone, Copy, Error)]
#[error("A model is currently being loaded in the background")]
pub struct LoadingInProgressError;

/** Enumeration of errors that can occur whilst loading a 3-D model. */
#[derive(Debug, Error)]
pub enum LoadModelError {
    /** Error encountered during the allocation of a Vulkan memory buffer for model data. */
    #[error("Failed to allocate Vulkan memory buffer for model geometry: {0}")]
    BufferAlloc(#[from] Validated<AllocateBufferError>),
    /** Failure to parse or read the model file using the underlying object loader. */
    #[error("Failed to load or parse the model file: {0}")]
    Tobj(#[from] LoadError),
    /** General failure or disconnection of the background loading thread. */
    #[error("A critical error occurred and the background loading process failed")]
    LoadingFailure,
    /** The loaded file did not contain any valid mesh or geometric data. */
    #[error("No valid mesh or geometric data was found within the loaded model file")]
    NoModel
    }

/** Enumeration of errors that can occur when resizing or recreating the depth buffer image. */
#[derive(Debug, Error)]
pub enum DepthImageResizeError {
    /** Error encountered dynamically allocating image memory for the new depth buffer. */
    #[error("Failed to allocate Vulkan image memory for the resized depth buffer: {0}")]
    ValidatedImageAlloc(#[from] Validated<AllocateImageError>),
    /** Core Vulkan execution error intercepted during the image recreation process. */
    #[error("Validated Vulkan error whilst resizing the depth buffer: {0}")]
    ValidatedVulkan(#[from] Validated<VulkanError>)
    }