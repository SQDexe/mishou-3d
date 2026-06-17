use {
    glam::{
        Vec2,
        Vec3,
        Mat4
        },
    std::time::Duration,
    crate::{
        camera::{
            CameraConfig,
            CameraInput
            },
        consts::{
            default::{
                CAMERA_POSITION,
                CAMERA_ROTATION,
                CAMERA_SPEED,
                CAMERA_SENSITIVITY,
                FOV,
                Z_NEAR,
                Z_FAR
                },
            ranges::PITCH_RANGE_LIMIT
            }
        }
    };



/** Representation of the virtual camera navigating the three-dimensional scene. */
#[derive(Debug, Clone)]
pub struct Camera {
    /** Flag indicating whether the cursor is currently captured for view rotation. */
    grabbed: bool,
    /** State of the directional movement inputs. */
    inputs: CameraInput,
    /** Spatial coordinates of the camera. */
    pub position: Vec3,
    /** Orientation angles represented as yaw and pitch. */
    pub rotation: Vec2,
    /** Multiplier for the camera's movement velocity. */
    pub speed: f32,
    /** Multiplier for the camera's rotation responsiveness to mouse movements. */
    pub sensitivity: f32,
    /** Vertical field of view angle in degrees. */
    pub fov: f32,
    /** Distance to the near clipping plane. */
    pub z_near: f32,
    /** Distance to the far clipping plane. */
    pub z_far: f32
    }

impl Camera {
    /** Initialises a new camera instance using the provided configuration parameters. */
    pub fn with_config(config: CameraConfig) -> Self {
        let CameraConfig {
            position,
            rotation,
            speed,
            sensitivity,
            fov,
            z_near,
            z_far
            } = config;

        Self {
            grabbed: false,
            inputs: CameraInput::default(),
            position,
            rotation,
            speed,
            sensitivity,
            fov,
            z_near,
            z_far
            }
        }

    /** Captures the camera, enabling view rotation via mouse inputs. */
    pub const fn set_grabbed(&mut self) {
        self.grabbed = true;
        }

    /** Releases the camera, disabling view rotation. */
    pub const fn set_loose(&mut self) {
        self.grabbed = false;
        }

    /** Resets the camera's position and view parameters to their default values whilst preserving the current input and capture state. */
    pub fn reset(&mut self) {
        *self = Self {
            grabbed: self.grabbed,
            inputs: self.inputs,
            .. Default::default()
            };
        }

    /** Retrieves a mutable reference to the camera's movement input state. */
    pub const fn inputs_mut(&mut self) -> &mut CameraInput {
        &mut self.inputs
        }

    /** Calculates the normalised forward directional vector based on the current yaw and pitch. */
    fn forward_dir(&self) -> Vec3 {
        let Vec2 { x, y } = self.rotation;

        let (yaw_rad, pitch_rad) = (
            x.to_radians(),
            y.to_radians()
            );

        let (x, y, z) = (
            yaw_rad.cos() * pitch_rad.cos(),
            pitch_rad.sin(),
            yaw_rad.sin() * pitch_rad.cos()
            );

        Vec3::new(x, y, z).normalize()
        }

    /** Generates the view matrix required for world-to-camera coordinate transformations. */
    pub fn view_matrix(&self) -> Mat4 {
        let forward = self.forward_dir();

        Mat4::look_at_rh(self.position, self.position + forward, Vec3::Y)
        }

    /** Generates the perspective projection matrix for the given screen aspect ratio. */
    pub fn projection_matrix(&self, aspect_ratio: f32) -> Mat4 {
        /* Projection calculation */
        let mut proj = Mat4::perspective_rh(
            self.fov.to_radians(),
            aspect_ratio,
            self.z_near,
            self.z_far,
            );
        
        /* Correction due to Vulkan having inverted y-axis */
        proj.y_axis.y *= -1.0; 
        
        proj
        }

    /** Applies rotational changes to the camera based on relative mouse movements. */
    pub fn rotate_camera(&mut self, input: Vec2) {
        if ! self.grabbed {
            return;
            }

        let Vec2 { x: ref mut yaw, y: ref mut pitch } = self.rotation;
        let Vec2 { x, y } = input;

        *yaw += (x as f32) * self.sensitivity;
        let new_pitch = *pitch - (y as f32) * self.sensitivity;

        /* Clampingto prevent a gimbal lock */
        *pitch = new_pitch.clamp(
            *PITCH_RANGE_LIMIT.start(),
            *PITCH_RANGE_LIMIT.end()
            );
        }
    
    /** Updates the spatial position of the camera based on active movement inputs and the elapsed time delta. */
    pub fn move_camera(&mut self, time_delta: Duration) {
        let Vec3 { x, y, z } = self.inputs.get_input().into();

        let z_axis = self.forward_dir();
        let x_axis = z_axis.cross(Vec3::Y).normalize();
        let y_axis = x_axis.cross(z_axis).normalize();

        let move_dir = z_axis * z + x_axis * x + y_axis * y;

        self.position += move_dir * self.speed * time_delta.as_secs_f32();
        }
    }

impl Default for Camera {
    fn default() -> Self {
        Self {
            grabbed: false,
            inputs: CameraInput::default(),
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