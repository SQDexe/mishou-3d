use {
    glam::{
        Vec2,
        Vec3
        },
    crate::consts::default::{
        CAMERA_POSITION,
        CAMERA_ROTATION,
        CAMERA_SPEED,
        CAMERA_SENSITIVITY,
        FOV,
        Z_NEAR,
        Z_FAR
        }
    };



/** Configuration parameters defining the state and behaviour of the scene's virtual camera. */
#[derive(Debug, Clone)]
pub struct CameraConfig {
    /** Spatial coordinates of the camera within the 3-D environment. */
    pub position: Vec3,
    /** Orientation of the camera, typically represented as yaw and pitch angles. */
    pub rotation: Vec2,
    /** Movement speed multiplier for camera navigation. */
    pub speed: f32,
    /** Mouse sensitivity multiplier for adjusting the camera's viewing angle. */
    pub sensitivity: f32,
    /** Vertical field of view angle expressed in degrees. */
    pub fov: f32,
    /** Distance to the near clipping plane, defining the closest visible surface. */
    pub z_near: f32,
    /** Distance to the far clipping plane, defining the furthest visible surface. */
    pub z_far: f32
    }

impl Default for CameraConfig {
    fn default() -> Self {
        Self {
            position: CAMERA_POSITION,
            rotation: CAMERA_ROTATION,
            speed: CAMERA_SPEED,
            sensitivity: CAMERA_SENSITIVITY,
            fov: FOV,
            z_near: Z_NEAR,
            z_far: Z_FAR
            }
        }
    }