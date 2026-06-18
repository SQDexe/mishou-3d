use {
    color_art::Color,
    clap::{
        Parser,
        CommandFactory,
        Args,
        error::ErrorKind
        },
    glam::{
        Vec2,
        Vec3
        },
    std::path::PathBuf,
    crate::{
        args::parsers::{
            parse_colour,
            parse_vec2,
            parse_vec3
            },
        camera::CameraConfig,
        consts::{
            default::{
                // SHOW_CONFIG,
                // ROTATION_SPEED,
                CLI_BACKGROUND_COLOUR,
                CLI_MODEL_COLOUR,
                CLI_LIGHT_DIRECTION,
                CLI_CAMERA_POSITION,
                CLI_CAMERA_ROTATION,
                CAMERA_SPEED,
                CAMERA_SENSITIVITY,
                FOV,
                Z_NEAR,
                Z_FAR
                },
            ranges::{
                PITCH_RANGE_LIMIT,
                CAMERA_SPEED_RANGE_LIMIT,
                CAMERA_SENSITIVITY_RANGE_LIMIT,
                FOV_RANGE_LIMIT,
                Z_PLANE_RANGE_LIMIT
                },
            },
        state::SceneConfig,
        utils::colour_into_vec3
        }
    };



/**
Command-line interface arguments for initialising the application's state. 

Parsed at startup to dictate the initial behaviour of the 3D viewer, including camera, lighting, and visual settings.
*/
#[derive(Debug, Clone, Parser)]
#[command(author, version, about, propagate_version = true)]
pub struct Arguments {
    /** The target 3D model file to load at startup. */
    #[command(flatten)]
    input_file: Inputs,
    // /** Flag indicating whether to display the parsed configuration parameters upon launch. */
    // #[arg(short = 'C', long, action, default_value_t = SHOW_CONFIG)]
    // show_config: bool,
    /** The clear colour of the scene's background. */
    #[arg(long, default_value = CLI_BACKGROUND_COLOUR, value_parser = parse_colour)]
    background_colour: Color,
    /** The default tint or diffuse colour applied to the loaded models. */
    #[arg(long, default_value = CLI_MODEL_COLOUR, value_parser = parse_colour)]
    model_colour: Color,
    /** The direction of the primary global illumination, provided as yaw and pitch angles. */
    #[arg(long, default_value = CLI_LIGHT_DIRECTION, value_parser = parse_vec2)]
    light_direction: Vec2,
    // /** The direction of the primary global illumination, provided as yaw and pitch angles. */
    // #[arg(long, default_value_t = ROTATION_SPEED)]
    // rotation_speed: f32,
    /** The initial spatial coordinates of the camera. */
    #[arg(long, default_value = CLI_CAMERA_POSITION, value_parser = parse_vec3)]
    camera_position: Vec3,
    /** The initial orientation angles of the camera, provided as yaw and pitch. */
    #[arg(long, default_value = CLI_CAMERA_ROTATION, value_parser = parse_vec2)]
    camera_rotation: Vec2,
    /** The multiplier for the camera's movement velocity. */
    #[arg(long, default_value_t = CAMERA_SPEED)]
    camera_speed: f32,
    /** The multiplier for the camera's rotational responsiveness to mouse movements. */
    #[arg(long, default_value_t = CAMERA_SENSITIVITY)]
    camera_sensitivity: f32,
    /** The vertical field of view angle in degrees. */
    #[arg(long, default_value_t = FOV)]
    fov: f32,
    /** The distance to the near clipping plane. */
    #[arg(long, default_value_t = Z_NEAR)]
    z_near: f32,
    /** The distance to the far clipping plane. */
    #[arg(long, default_value_t = Z_FAR)]
    z_far: f32
    }

/** Grouping of mutually exclusive file input methods for the command-line interface. */
#[derive(Debug, Clone, Args)]
#[group(multiple = false)]
pub struct Inputs {
    /** The input file path passed as a positional command-line argument. */
    #[arg(value_name = "MODEL_PATH")]
    positional: Option<PathBuf>,
    /** The input file path passed explicitly via a flag. */
    #[arg(short = 'i', long = "input", value_name = "MODEL_PATH")]
    flag: Option<PathBuf>
    }

impl From<Inputs> for Option<PathBuf> {
    fn from(value: Inputs) -> Self {
        value.positional.xor(value.flag)
        }
    }

impl Arguments {    
    /**
    Validates the parsed command-line arguments for logical correctness.

    Performs checks on parameters like numerical bounds to guarantee a sound application state.

    # Terminates

    Terminates the program with a non-zero exit code and prints an error if validation fails.
    */
    pub fn validate(&self) {
        let error = if ! self.light_direction.is_finite() {
            Some("'light_direction' fields must be finite")
        // } else if ! self.rotation_speed.is_finite() {
        //     error = Some("must be finite")
        } else if ! self.camera_position.is_finite() {
            Some("'camera_position' fields must be finite")
        } else if ! self.camera_rotation.x.is_finite() {
            Some("'yaw' must be finite")
        } else if ! PITCH_RANGE_LIMIT.contains(&self.camera_rotation.y) {
            Some("'pitch' must be within range of -90 to 90")
        } else if ! CAMERA_SPEED_RANGE_LIMIT.contains(&self.camera_speed) {
            Some("'camera_speed' must be within range of 0 to 16")
        } else if ! CAMERA_SENSITIVITY_RANGE_LIMIT.contains(&self.camera_sensitivity) {
            Some("'camera_sensitivity' must be within range of 0 to 1")
        } else if ! FOV_RANGE_LIMIT.contains(&self.fov) {
            Some("'fov' must be within range 10 to 150")
        } else if ! Z_PLANE_RANGE_LIMIT.contains(&self.z_near) {
            Some("'z_near' must be within range of 0.0001 to 1024")
        } else if ! Z_PLANE_RANGE_LIMIT.contains(&self.z_far) {
            Some("'z_far' must be within range of 0.0001 to 1024")
        } else if self.z_far <= self.z_near {
            Some("'z_far' must be strictly greater than 'z_near'")
        } else {
            None
            };

        if let Some(error_msg) = error {
            Self::command()
                .error(ErrorKind::ValueValidation, error_msg)
                .exit();
            }
        }
    }

impl From<Arguments> for SettingsDto {
    fn from(value: Arguments) -> Self {
        let Arguments {
            // rotation_speed,
            background_colour,
            model_colour,
            light_direction,
            camera_position,
            camera_rotation,
            camera_speed,
            camera_sensitivity,
            fov,
            z_near,
            z_far,
            input_file
            } = value;

        let scene_config = SceneConfig {
            // rotation_speed,
            background_colour: colour_into_vec3(background_colour),
            model_colour: colour_into_vec3(model_colour),
            light_direction,
            };

        let camera = CameraConfig {
            position: camera_position,
            rotation: camera_rotation,
            speed: camera_speed,
            sensitivity: camera_sensitivity,
            fov,
            z_near,
            z_far
            };

        let model_path = input_file.into();

        Self { scene_config, camera_config: camera, model_path }
        }
    }

/** Data transfer object that groups parsed command-line configurations to be passed to the application state. */
pub struct SettingsDto {
    /** Configuration parameters governing the virtual camera. */
    pub camera_config: CameraConfig,
    /** Global configuration data for the 3D scene environment. */
    pub scene_config: SceneConfig,
    /** Optional file path pointing to the target 3D model to be loaded. */
    pub model_path: Option<PathBuf>
    }