use {
    glam::Vec3,
    crate::camera::matrix_unit::MatrixUnit
    };



/** Represents the discrete directional movement intent of the camera across the three primary axes. */
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CameraMovement {
    /** Movement intent along the lateral X-axis. */
    pub x: MatrixUnit,
    /** Movement intent along the vertical Y-axis. */
    pub y: MatrixUnit,
    /** Movement intent along the longitudinal Z-axis. */
    pub z: MatrixUnit
    }

impl CameraMovement {
    /** Initialises a new camera movement state with no active motion across any axis. */
    pub const fn new() -> Self {
        Self {
            x: MatrixUnit::Zero,
            y: MatrixUnit::Zero,
            z: MatrixUnit::Zero
            }
        }

    /** Converts the discrete matrix unit movement states into a continuous 3-D floating-point vector. */
    pub const fn as_vec_f32(&self) -> Vec3 {
        Vec3 {
            x: self.x as i8 as f32,
            y: self.y as i8 as f32,
            z: self.z as i8 as f32
            }
        }
    }

impl Default for CameraMovement {
    fn default() -> Self {
        Self::new()
        }
    }

impl From<CameraMovement> for Vec3 {
    fn from(value: CameraMovement) -> Self {
        value.as_vec_f32()
        }
    }