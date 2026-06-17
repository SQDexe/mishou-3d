use {
    vulkano::swapchain::PresentMode,
    std::time::Duration,
    core::num::NonZeroU16,
    sqds_tools::get_match
    };



/** Frame rate limiting mode configuration. */
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum FpsMode {
    /** Synchronise the frame rate with the monitor's refresh rate. */
    #[default]
    VSync,
    /** Limit the frame rate to a specific target value. */
    WaitFor {
        /** Target frames per second value. */
        fps_label: NonZeroU16,
        /** Calculated duration of a single frame. */
        duration: Duration
        },
    /** Do not impose any limits on the frame rate. */
    Unlimited
    }

impl FpsMode {
    /** Creates a constrained frame rate mode alongside its required frame duration based on the provided target value. */
    pub fn from_fps(fps: NonZeroU16) -> Self {
        Self::WaitFor {
            fps_label: fps,
            duration: Duration::from_secs_f64(1.0 / fps.get() as f64)
            }
        }
    }

impl PartialEq<NonZeroU16> for FpsMode {
    fn eq(&self, other: &NonZeroU16) -> bool {
        matches!(self, FpsMode::WaitFor { fps_label, .. } if fps_label == other)
        }
    }

impl From<FpsMode> for PresentMode {
    fn from(value: FpsMode) -> Self {
        get_match!(value, FpsMode::VSync => PresentMode::Fifo)
            .unwrap_or(PresentMode::Immediate)
        }
    }