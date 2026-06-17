/*! 
Mishou 3D is a high-performance rendering application built with Rust.

It leverages the raw power of the Vulkan API for graphics processing and provides a seamless,  
immediate-mode graphical user interface for interacting with the environment.  
Designed for speed and simplicity, it allows users to load, view,  
and inspect 3D models with real-time configuration tweaking.
*/

mod app;
mod args;
mod camera;
mod consts;
mod error;
mod gui;
mod log;
mod model;
mod render;
mod state;
mod utils;



pub use args::Arguments;
pub use app::App;
pub use error::AppCreateError;
pub use log::alert_warning;
pub use log::alert_error;
pub use log::panic_hook;
pub use log::init_logger;