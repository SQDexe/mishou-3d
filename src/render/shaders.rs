/** Module containing the compiled vertex shader program. */
pub mod vs {
    vulkano_shaders::shader! {
        ty: "vertex",
        path: "./shaders/shader.vert"
        }
    }

/** Module containing the compiled fragment shader program. */
pub mod fs {
    vulkano_shaders::shader! {
        ty: "fragment",
        path: "./shaders/shader.frag"
        }
    }