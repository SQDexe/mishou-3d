/*! Virtual camera system and spatial navigation logic. */

/** Configuration parameters defining the state and behaviour of the scene's virtual camera. */
mod camera_config;
/** Bitfield representation of currently active camera movement inputs. */
mod camera_input;
/** Represents the discrete directional movement intent of the camera across the three primary axes. */
mod camera_movement;
/** Representation of the virtual camera navigating the three-dimensional scene. */
mod camera;
/** Discrete unit representation of directional movement along a single matrix axis. */
mod matrix_unit;



pub use camera_config::CameraConfig;
pub use camera_input::CameraInput;
pub use camera::Camera;