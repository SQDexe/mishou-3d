/*! Vulkan rendering pipelines and graphical output logic. */

/** Rendering mode selection for the scene. */
mod render_mode;
/** Encapsulates the core Vulkan rendering state and resources. */
mod render_state;
/** Module containing the compiled shaders' program. */
mod shaders;



pub use render_mode::RenderMode;
pub use render_state::RenderState;
pub use shaders::fs::load as fragment_shader_load;
pub use shaders::vs::load as vertex_shader_load;
pub use shaders::fs::PushConstantData;