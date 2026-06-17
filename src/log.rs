/*! Logging infrastructure and custom panic hooks. */

use {
    env_logger::builder as logger_builder,
    log::{
        warn,
        error,
        STATIC_MAX_LEVEL
        },
    rfd::{
        MessageDialog,
        MessageLevel
        },
    std::{
        io::Write,
        panic::PanicHookInfo
        },
    crate::consts::log::{
        DIALOG_WARNING_LEVEL,
        DIALOG_WARNING_TITLE,
        DIALOG_ERROR_LEVEL,
        DIALOG_ERROR_TITLE,
        DIALOG_CRITICAL_TITLE,
        DIALOG_BUTTONS_LAYOUT,
        PANIC_DEFAULT_PAYLOAD,
        PANIC_DEFAULT_LOCATION
        }
    };



/** Alert levels for adequate response. */
#[derive(Debug, Clone, Copy)]
enum AlertLevel {
    /** Non-critical errors. */
    Warning,
    /** Serious, but controlled errors. */
    Error,
    /** For uncontroled panics. */
    Critical
    }

impl AlertLevel {
    /** Logs the message with specified alert level. */
    fn log(&self, msg: &str) {
        match self {
            Self::Warning =>
                warn!("{msg}"),
            Self::Error | Self::Critical =>
                error!("{msg}")
            }
        }
    }

impl From<AlertLevel> for (MessageLevel, &'static str) {
    fn from(value: AlertLevel) -> Self {
        match value {
            AlertLevel::Warning =>
                (DIALOG_WARNING_LEVEL, DIALOG_WARNING_TITLE),
            AlertLevel::Error =>
                (DIALOG_ERROR_LEVEL, DIALOG_ERROR_TITLE),
            AlertLevel::Critical =>
                (DIALOG_ERROR_LEVEL, DIALOG_CRITICAL_TITLE)
            }
        }
    }

/**
Prepare, build, and show alert box,  
with a provided message.

Additionally logs the message.
*/
fn show_alert(level: AlertLevel, msg: &str) {
    /* Log the message first */
    level.log(msg);

    /* Map alert level to corresponding values */
    let (dialog_box_level, title) = level.into();

    /* Build, and show the alert box */
    MessageDialog::new()
        .set_level(dialog_box_level)
        .set_title(title)
        .set_description(msg)
        .set_buttons(DIALOG_BUTTONS_LAYOUT)
        .show();
    }

/** Show alerts for warning level. */
pub fn alert_warning(msg: &str) {
    show_alert(AlertLevel::Warning, msg);
    }

/** Show alerts for error level. */
pub fn alert_error(msg: &str) {
    show_alert(AlertLevel::Error, msg);
    }

/** Custom panic hook, for better user experience in case of critical application errors. */
pub fn panic_hook(panic_info: &PanicHookInfo) {
    /* Extract important informations */
    let payload = panic_info.payload_as_str()
        .unwrap_or(PANIC_DEFAULT_PAYLOAD);
    let location = panic_info.location()
        .map(|location| location.to_string())
        .unwrap_or_else(|| PANIC_DEFAULT_LOCATION.to_owned());

    /* Format informations into a message */
    let msg = format!("{payload}\nAt: {location}");

    /* Build, and show message alert box */
    show_alert(AlertLevel::Critical, &msg);
    }

/** Logger initialisation. */
pub fn init_logger() {
    logger_builder()
        .format(|buf, record| {
            let level = record.level();
            let style = buf.default_level_style(level);
            writeln!(buf, "[{style}{level}{style:#}]: {}", record.args())
            })
        .filter_level(STATIC_MAX_LEVEL)
        .init();
    }