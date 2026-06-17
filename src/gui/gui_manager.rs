use {
    egui_file_dialog::FileDialog,
    egui_winit_vulkano::Gui,
    crate::utils::create_gui_filedialog_config
    };



/** Manager responsible for the graphical user interface state and its component windows. */
pub struct GuiManager {
    /** Core graphical user interface rendering and integration state. */
    pub gui: Gui,
    /** State and configuration for the file selection dialogue widget. */
    pub file_dialog: FileDialog,
    /** Flag indicating whether the configuration panel is currently visible. */
    pub show_config: bool,
    /** Flag indicating whether the frame rate counter is currently visible. */
    pub show_fps: bool,
    /** Flag indicating whether the 'About' window is currently visible. */
    pub show_about: bool
    }

impl GuiManager {
    /**
    Initialises a new graphical user interface manager with the provided core state.
    
    This sets up the default file dialogue configuration and establishes the initial visibility of interface components.
    */
    pub fn new(gui: Gui) -> Self {
        let config = create_gui_filedialog_config();
        let file_dialog = FileDialog::with_config(config);

        Self {
            gui,
            file_dialog,
            show_config: true,
            show_fps: true,
            show_about: false
            }
        }
    }