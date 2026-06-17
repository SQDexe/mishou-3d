/*! Command-line interface argument parsing and validation. */

/** Defines the primary command-line argument structures and data transfer objects. */
mod arguments;
/** Provides validation logic and boundary checks for parsed command-line parameters. */
mod checks;
/** Contains custom parsing functions for converting string inputs into typed application configurations. */
mod parsers;



pub use arguments::Arguments;
pub use arguments::SettingsDto;