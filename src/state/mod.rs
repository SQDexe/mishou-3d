/*! Global scene configuration and runtime state tracking. */

/** Timekeeper responsible for calculating frame deltas and tracking frames per second. */
mod clock;
/** Frame rate limiting mode configuration. */
mod fps_mode;
/** Structure holding the global configuration data for the 3D scene. */
mod scene_config;
/** Manager responsible for application window state, error handling, and exit signalling. */
mod window_manager;



pub use clock::Clock;
pub use fps_mode::FpsMode;
pub use scene_config::SceneConfig;
pub use window_manager::WindowManager;