#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use {
    clap::Parser,
    winit::event_loop::{
        ControlFlow,
        EventLoop
        },
    std::process::{
        ExitCode,
        Termination
        },
    core::error::Error,
    mishou_3d::{
        App,
        Arguments,
        alert_error,
        init_logger
        }
    };

/* Non-debug mode imports */
#[cfg(not(debug_assertions))]
use {
    std::panic::set_hook,
    mishou_3d::panic_hook
    };

/* Windows, and non-debug mode imports */
#[cfg(all(windows, not(debug_assertions)))]
use windows_sys::Win32::System::Console::{
    AttachConsole,
    ATTACH_PARENT_PROCESS
    };


/**
Executes the main application logic and event loop.

# Errors

Returns an error if the event loop fails to initialise, the application fails to build, or a runtime error occurs during execution.
*/
fn run() -> Result<(), Box<dyn Error>> {
    let args = Arguments::parse();
    args.validate();

    let event_loop = EventLoop::new()?;

    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = App::new(&event_loop, args.into())?;

    event_loop.run_app(&mut app)?;

    Result::from(app)?;

    Ok(())
    }

/**
The main entry point of the application.

Initialises the logger, sets up a custom panic hook for release builds,  
attaches to the parent console on Windows, and runs the application.  
Alerts the user if a critical error propagates to the top level.
*/
fn main() -> ExitCode {
    /* Set up custom panic hook */
    #[cfg(not(debug_assertions))]
    set_hook(Box::new(panic_hook));

    /*
    SAFETY:
    Crossing of FFI, Rust can't deduce whether this is safe
    There is nothing inherently unsafe about this operation
    Worst case scenario – the console won't work
    */
    #[cfg(all(windows, not(debug_assertions)))]
    unsafe {
        /* Attach window console for use in terminal */
        _ = AttachConsole(ATTACH_PARENT_PROCESS)
        };

    /* Set up logging */
    init_logger();

    /* Run the application */
    let result = run();

    /* Alert possible errors */
    if let Err(ref error) = result {
        alert_error(&error.to_string())
        }
        
    /* Return exit code based on result */
    result.report()
    }