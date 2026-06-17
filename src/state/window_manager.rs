use {
    core::error::Error,
    crate::state::FpsMode
    };



/** Manager responsible for application window state, error handling, and exit signalling. */
#[derive(Debug, Default)]
pub struct WindowManager {
    /** Stored critical error that will trigger the application to exit. */
    error: Option<Box<dyn Error>>,
    /** Flag indicating whether a graceful exit has been requested. */
    exit_requested: bool,
    /** Flag indicating whether the window should toggle its fullscreen state. */
    fullscreen_toggle_requested: bool,
    /** Currently active frame rate limiting mode. */
    fps_mode: FpsMode,
    /** Pending request to change the frame rate limiting mode. */
    fps_mode_request: Option<FpsMode>
    }

impl WindowManager {
    /** Requests that the application exits on the next state check. */
    pub const fn request_exit(&mut self) {
        self.exit_requested = true;
        }
        
    /**
    Reports a critical error.
    
    This signals the application that it should exit on the next check.
    */
    pub fn report_error<E>(&mut self, value: E)
    where E: Error + 'static {
        self.error = Some(Box::new(value));
        }

    /** Signals whether the application should begin the exit process. */
    pub const fn should_exit(&self) -> bool {
        self.exit_requested || self.error.is_some()
        }
    
    /** Requests a toggle of the window's fullscreen state. */
    pub const fn request_fullscreen_toggle(&mut self) {
        self.fullscreen_toggle_requested = true;
        }

    /** Checks whether a fullscreen toggle has been requested. */
    pub const fn is_fullscreen_toggle_requested(&self) -> bool {
        self.fullscreen_toggle_requested
        }

    /** Clears the fullscreen toggle request flag after it has been processed. */
    pub const fn finish_fullscreen_toggle(&mut self) {
        self.fullscreen_toggle_requested = false;
        }

    /** Retrieves the currently active frame rate mode. */
    pub const fn fps_mode(&self) -> &FpsMode {
        &self.fps_mode
        }

    /** Retrieves the pending frame rate mode request, if one exists. */
    pub const fn fps_mode_request_payload(&self) -> Option<&FpsMode> {
        self.fps_mode_request.as_ref()
        }

    /** Requests a transition to a new frame rate limiting mode. */
    pub const fn request_fps_mode(&mut self, fps_mode: FpsMode) {
        self.fps_mode_request = Some(fps_mode);
        }

    /** Applies the pending frame rate mode request and clears the internal request state. */
    pub const fn finish_fps_mode_request(&mut self) {
        if let Some(fps_mode) = self.fps_mode_request.take() {
            self.fps_mode = fps_mode;
            }
        }
    }

impl From<WindowManager> for Option<Box<dyn Error>> {
    fn from(value: WindowManager) -> Self {
        value.error
        }
    }