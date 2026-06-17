use {
    glam::{
        Vec2,
        Vec3
        },
    // std::time::Instant,
    crate::consts::default::{
        // ROTATION_SPEED,
        BACKGROUND_COLOUR,
        MODEL_COLOUR,
        LIGHT_DIRECTION
        }
    };



/** Structure holding the global configuration data for the 3D scene. */
#[derive(Debug, Clone)]
pub struct SceneConfig {
    // /** Timestamp marking the start of the scene, used for time-based calculations. */
    // pub start_time: Instant,
    // /** Speed multiplier for model rotation. */
    // pub rotation_speed: f32,
    /** Clear colour of the scene's background. */
    pub background_colour: Vec3,
    /** Default tint or diffuse colour applied to the loaded models. */
    pub model_colour: Vec3,
    /** Direction of the primary global illumination, represented as yaw and pitch angles. */
    pub light_direction: Vec2
    }

impl SceneConfig {
    /** Resets the scene configuration back to its default values. */
    pub fn reset(&mut self) {
        *self = Self::default();
        }

    // /** Calculates the elapsed time in seconds since the scene was initialised. */
    // pub fn time_from_start(&self) -> f32 {
    //     self.start_time.elapsed()
    //         .as_secs_f32()
    //     }
    }

impl Default for SceneConfig {
    fn default() -> Self {
        Self {
            // start_time: Instant::now(),
            // rotation_speed: ROTATION_SPEED,
            background_colour: BACKGROUND_COLOUR,
            model_colour: MODEL_COLOUR,
            light_direction: LIGHT_DIRECTION
            }
        }
    }