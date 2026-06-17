use {
    tobj::{
        GPU_LOAD_OPTIONS,
        load_obj,
        Mesh
        },
    vulkano::{
        buffer::Subbuffer,
        memory::allocator::StandardMemoryAllocator
        },
    std::{
        path::Path,
        sync::Arc
        },
    crate::{
        consts::config::DEFAULT_NORMAL,
        error::LoadModelError,
        model::Vertex,
        utils::{
            create_vertex_buffer,
            create_index_buffer
            }
        }
    };



/**
Representation of a loaded three-dimensional model ready for rendering.

This structure encapsulates the memory buffers required by Vulkan to draw the object.
*/
#[derive(Debug)]
pub struct Model {
    /** Device memory subbuffer containing the parsed vertex data, including spatial positions and surface normals. */
    vertex_buffer: Subbuffer<[Vertex]>,
    /** Device memory subbuffer containing the sequence of indices used to construct faces from the vertices. */
    index_buffer: Subbuffer<[u32]>
    }

impl Model {
    /**
    Loads a 3-D model from a specified file path and allocates its geometry into device memory.
    
    This function parses the given object file, extracts the first available mesh, constructs the vertices 
    (pairing positions with their respective normals, or a default normal if missing), and uploads them to the GPU.
    
    # Errors
    
    Returns a `LoadModelError` if the file cannot be read, the data format is invalid, no mesh is found within the file, 
    or if the allocation of the Vulkan memory buffers fails.
    */
    pub fn from_path(path: &Path, memory_allocator: Arc<StandardMemoryAllocator>) -> Result<Self, LoadModelError> {
        let (models, _) = load_obj(path, &GPU_LOAD_OPTIONS)?;

        let Mesh { positions, normals, indices, .. } = models.into_iter()
            .next()
            .ok_or(LoadModelError::NoModel)?
            .mesh;

        let mut normals_iter = normals.as_chunks().0.into_iter();
        let vertices = positions.as_chunks()
            .0.into_iter()
            .map(|&position| Vertex::new(
                position,
                normals_iter.next().map(|&normal| normal).unwrap_or(DEFAULT_NORMAL)
                ))
            .collect();

        let vertex_buffer = create_vertex_buffer(vertices, Arc::clone(&memory_allocator))?;
        let index_buffer = create_index_buffer(indices, Arc::clone(&memory_allocator))?;

        Ok(Self { vertex_buffer, index_buffer })
        }

    /** Retrieves a reference to the device memory subbuffer containing the model's vertices. */
    pub const fn vertex_buffer(&self) -> &Subbuffer<[Vertex]> {
        &self.vertex_buffer
        }

    /** Retrieves a reference to the device memory subbuffer containing the model's drawing indices. */
    pub const fn index_buffer(&self) -> &Subbuffer<[u32]> {
        &self.index_buffer
        }
    }