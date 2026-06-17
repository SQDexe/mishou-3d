use vulkano::{
    buffer::BufferContents,
    pipeline::graphics::vertex_input::Vertex as VertexInfo
    };



/** Representation of a single vertex in three-dimensional space. */
#[derive(Debug, Clone, Default, PartialEq, BufferContents, VertexInfo)]
#[repr(C)]
pub struct Vertex {
    /** Spatial coordinates of the vertex represented as a 3-D vector. */
    #[format(R32G32B32_SFLOAT)]
    position: [f32; 3],
    /** Normal vector of the vertex used for lighting and shading calculations. */
    #[format(R32G32B32_SFLOAT)]
    normal: [f32; 3]
    }

impl Vertex {
    /** Creates a new vertex instance from the given position and normal vectors. */
    pub const fn new(position: [f32; 3], normal: [f32; 3]) -> Self {
        Self { position, normal }
        }
    }