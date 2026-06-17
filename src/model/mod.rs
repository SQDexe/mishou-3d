/*! 3D asset loading, parsing, and memory management. */

/** Manager responsible for the asynchronous loading and storage of 3D models. */
mod model_manager;
/** Representation of a loaded three-dimensional model ready for rendering. */
mod model;
/** Representation of a single vertex in three-dimensional space. */
mod vertex;



pub use model_manager::ModelManager;
pub use model::Model;
pub use vertex::Vertex;