use {
    std::time::{
        Duration,
        Instant
        },
    crate::consts::config::REFRESH_TIME
    };



/** Timekeeper responsible for calculating frame deltas and tracking frames per second. */
#[derive(Debug, Clone, Copy)]
pub struct Clock {
    /** Timestamp of the last clock update. */
    last_update: Instant,
    /** Duration elapsed between the two most recent clock updates. */
    time_delta: Duration,
    /** Accumulator for the elapsed time since the last frame rate calculation. */
    fps_timer: f32,
    /** Number of frames rendered since the last frame rate calculation. */
    frame_count: u32,
    /** Most recently calculated frames per second value. */
    current_fps: u32
    }

impl Clock {
    /**
    Updates the internal timers and calculates the current frame rate.
    
    This method should be called exactly once per render loop iteration.
    */
    pub fn update(&mut self) {
        let now = Instant::now();

        self.time_delta = now.duration_since(self.last_update);
        self.last_update = now;

        let delta_seconds = self.time_delta.as_secs_f32();

        self.frame_count += 1;
        self.fps_timer += delta_seconds;

        if REFRESH_TIME <= self.fps_timer {
            self.current_fps = (self.frame_count as f32 / self.fps_timer).round() as u32;
            self.frame_count = 0;
            self.fps_timer = 0.0;
            }
        }
    
    /** Retrieves the duration elapsed between the last two updates. */
    pub fn time_delta(&self) -> Duration {
        self.time_delta
        }

    /** Retrieves the duration elapsed since the last update was called. */
    pub fn time_elapsed(&self) -> Duration {
        self.last_update.elapsed() 
        }
    
    /** Retrieves the most recently calculated frames per second value. */
    pub fn fps(&self) -> u32 {
        self.current_fps
        } 
    }

impl Default for Clock {
    fn default() -> Self {
        Self {
            last_update: Instant::now(),
            time_delta: Duration::ZERO,
            fps_timer: 0.0,
            frame_count: 0,
            current_fps: 0
            }
        }
    }