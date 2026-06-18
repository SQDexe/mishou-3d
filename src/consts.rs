/*! Global constant values and configuration limits. */

/** Application identification labels and versioning constants. */
pub mod labels {
    use {
        const_str::{
            concat as const_concat,
            parse
            },
        vulkano::Version
        };

    /** Major version number of the package retrieved from the environment. */
    const PKG_MAJOR_NUMBER: &str = env!("CARGO_PKG_VERSION_MAJOR");
    /** Minor version number of the package retrieved from the environment. */
    const PKG_MINOR_NUMBER: &str = env!("CARGO_PKG_VERSION_MINOR");
    /** Patch version number of the package retrieved from the environment. */
    const PKG_PATCH_NUMBER: &str = env!("CARGO_PKG_VERSION_PATCH");

    /** Version of the package retrieved from the environment. */
    pub const PKG_VERSION: &str = env!("CARGO_PKG_VERSION");
    /** Name of the package retrieved from the environment. */
    pub const PKG_NAME: &str = env!("CARGO_PKG_NAME");

    /**
    Version object parsed from environment value.

    This can be done using trait [`FromStr`] implemented on Version type,  
    but doing so is currently `const` unstable.  
    This could be further overcome with [`LazyLock`],  
    but such complicated structure isn't worth for such minute pay-off.

    [`FromStr`]: https://doc.rust-lang.org/stable/core/str/trait.FromStr.html
    [`LazyLock`]: https://doc.rust-lang.org/stable/std/sync/struct.LazyLock.html
    */
    pub const PROJECT_VERSION: Version = Version {
        major: parse!(PKG_MAJOR_NUMBER, u32),
        minor: parse!(PKG_MINOR_NUMBER, u32),
        patch: parse!(PKG_PATCH_NUMBER, u32)
        };

    /** Standard application name. */
    pub const APP_NAME: &str = PKG_NAME;
    /** Engine name. */
    pub const ENGINE_NAME: &str = const_concat!(PKG_NAME, "-engine");
    /** Stylised application name. */
    pub const STYLISED_APP_NAME: &str = "Mishō";
    /** Stylised application name with version number attached. */
    pub const STYLISED_APP_NAME_WITH_VERSION: &str = const_concat!(STYLISED_APP_NAME, " ", PKG_VERSION);
    }



/** Important configuration, and runtime constants. */
pub mod config {
    use {
        vulkano::format::ClearValue,
        winit::dpi::PhysicalSize,
        std::time::Duration,
        core::num::NonZeroU16
        };

    /** Boolean value indicating whether the binary was compiled in debug mode. */
    pub const DEBUG_MODE: bool = cfg!(debug_assertions);

    /** Identifier for the Vulkan validation layer used in debug builds. */
    const VALIDATION_LAYER_ID: &str = "VK_LAYER_KHRONOS_validation";
    /** Collection of Vulkan layer identifiers to be enabled for this project. */
    pub const VULKAN_LAYER_IDS: &[&str] = &[
        #[cfg(debug_assertions)]
        VALIDATION_LAYER_ID,
        ];

    /** Maximum duration to wait for a frame to render before timing out. */
    pub const RENDER_TIMEOUT: Duration = Duration::from_millis(10);
    /** Correction duration applied to sleep calculations to account for thread wake-up inaccuracies. */
    pub const SLEEP_BIAS_CORRECTION: Duration = Duration::from_micros(1500);
    /** Time interval in seconds between clock frames per second counter refreshes. */
    pub const REFRESH_TIME: f32 = 1.0;

    /** Clear value applied to the depth buffer at the start of a render pass. */
    pub const DEPTH_CLEAR_VALUE: ClearValue = ClearValue::Depth(1.0);
    /** Default normal vector value, pointing along the positive Z-axis. */
    pub const DEFAULT_NORMAL: [f32; 3] = [0.0, 0.0, 1.0];

    /** Resource identifier used to access the stored binary icon data. */
    pub const ICON_RESOURCE_NAME: &str = "APP_ICON";
    /** Dimensions of the application icon in pixels. */
    pub const ICON_SIZE: PhysicalSize<u32> = PhysicalSize {
        width: 16,
        height: 16
        };

    /** Increment step size for standard user interface slider widgets. */
    pub const SLIDER_STEP_SIZE: f64 = 0.125;

    /** Target frame rate representation for 30 frames per second. */
    pub const FPS_30_VALUE: NonZeroU16 = NonZeroU16::new(30)
        .expect("value should not be zero");
    /** Target frame rate representation for 60 frames per second. */
    pub const FPS_60_VALUE: NonZeroU16 = NonZeroU16::new(60)
        .expect("value should not be zero");
    /** Target frame rate representation for 120 frames per second. */
    pub const FPS_120_VALUE: NonZeroU16 = NonZeroU16::new(120)
        .expect("value should not be zero");
    }



/** Important logging, and panic related constants. */
pub mod log {
    use rfd::{
        MessageLevel,
        MessageButtons
        };
    
    /** Dialog box message level for warnigns. */
    pub const DIALOG_WARNING_LEVEL: MessageLevel = MessageLevel::Warning;
    /** Dialog box message level for errors. */
    pub const DIALOG_ERROR_LEVEL: MessageLevel = MessageLevel::Error;
    /** Dialog box buttons' layout. */
    pub const DIALOG_BUTTONS_LAYOUT: MessageButtons = MessageButtons::Ok;
    /** Dialog box title for warnings. */
    pub const DIALOG_WARNING_TITLE: &str = "Warning";
    /** Dialog box title for errors. */
    pub const DIALOG_ERROR_TITLE: &str = "Error";
    /** Dialog box title for critical errors. */
    pub const DIALOG_CRITICAL_TITLE: &str = "Critical Error";
    /** Default value used in case of a missing payload information. */
    pub const PANIC_DEFAULT_PAYLOAD: &str = "An unknown critical error";
    /** Default value used in case of a missing location information. */
    pub const PANIC_DEFAULT_LOCATION: &str = "An unknown location";
    }



/** Default scene configuration values for the editor view and command-line interface. */
pub mod default {
    use glam::{
        Vec2,
        Vec3
        };

    // /** Default boolean flag indicating whether the application should show the configuration menu on start. */
    // pub const SHOW_CONFIG: bool = true;
    /** Default background colour value represented as a 3-D vector. */
    pub const BACKGROUND_COLOUR: Vec3 = Vec3::new(0.09803922, 0.09803922, 0.4392157);
    /** Default model colour value represented as a 3-D vector. */
    pub const MODEL_COLOUR: Vec3 = Vec3::new(0.6627451, 0.6627451, 0.6627451); // alt = 0.8, 0.2, 0.2
    /** Default direction vector from which the light is cast. */
    pub const LIGHT_DIRECTION: Vec2 = Vec2::new(90.0, 45.0);
    // /** Default rotation speed multiplier for the model viewing. */
    // pub const ROTATION_SPEED: f32 = 0.0;
    /** Default spatial position of the camera in the scene. */
    pub const CAMERA_POSITION: Vec3 = Vec3::new(0.0, 0.0, 2.0);
    /** Default rotation angles of the camera. */
    pub const CAMERA_ROTATION: Vec2 = Vec2::new(-90.0, 0.0);
    /** Default movement speed multiplier for the camera. */
    pub const CAMERA_SPEED: f32 = 1.0;
    /** Default mouse sensitivity multiplier for camera rotation. */
    pub const CAMERA_SENSITIVITY: f32 = 0.125;
    /** Default field of view angle in degrees. */
    pub const FOV: f32 = 90.0;
    /** Default distance to the near clipping plane. */
    pub const Z_NEAR: f32 = 0.00390625; // 1 / (2 ** 8)
    /** Default distance to the far clipping plane. */
    pub const Z_FAR: f32 = 256.0; // 2 ** 8
    
    /** Default command-line interface background colour string representation. */
    pub const CLI_BACKGROUND_COLOUR: &str = "midnightblue"; // alt = cornflowerblue
    /** Default command-line interface model colour string representation. */
    pub const CLI_MODEL_COLOUR: &str = "darkgray";
    /** Default command-line interface light direction string representation. */
    pub const CLI_LIGHT_DIRECTION: &str = "90,45";
    /** Default command-line interface camera position string representation. */
    pub const CLI_CAMERA_POSITION: &str = "0,0,2";
    /** Default command-line interface camera rotation string representation. */
    pub const CLI_CAMERA_ROTATION: &str = "-90,0";
    }



pub mod ranges {
    use {
        vulkano_util::window::WindowResizeConstraints,
        core::ops::RangeInclusive
        };

    /** Minimum and maximum physical size constraints for the application window. */
    pub const SIZE_CONSTRAINTS: WindowResizeConstraints = WindowResizeConstraints {
        min_width: 480.0,
        min_height: 360.0,
        max_width: f32::INFINITY,
        max_height: f32::INFINITY
        };

    /** Permissible range for the camera movement speed multiplier. */
    pub const CAMERA_SPEED_RANGE_LIMIT: RangeInclusive<f32> = 0.0 ..= 16.0;
    /** Permissible range for the camera look sensitivity. */
    pub const CAMERA_SENSITIVITY_RANGE_LIMIT: RangeInclusive<f32> = 0.0 ..= 1.0;
    /** Permissible range for clipping plane distances. */
    pub const Z_PLANE_RANGE_LIMIT: RangeInclusive<f32> = f32::MIN_POSITIVE ..= 1024.0;
    /** Permissible range for the camera field of view in degrees. */
    pub const FOV_RANGE_LIMIT: RangeInclusive<f32> = 10.0 ..= 150.0;
    // /** Permissible range for the camera yaw angle in degrees. */
    // pub const YAW_RANGE_LIMIT: RangeInclusive<f32> = -180.0 ..= 180.0;
    /** Permissible range for the camera pitch angle in degrees to prevent gimbal lock. */
    pub const PITCH_RANGE_LIMIT: RangeInclusive<f32> = (-90.0_f32).next_up() ..= (90.0_f32).next_down();
    // /** Permissible range for the camera roll angle in degrees. */
    // pub const ROLL_RANGE_LIMIT: RangeInclusive<f32> = -90.0 ..= 90.0;
    }