use std::time::{Duration, SystemTime};

use crate::frame_util::FrameManager;

/// This resemble the state of the timer, meaning that the behavior
/// of the shown timer, e.g number of cycles, cycle length, starting time,
/// the displayed time at the start, etc
#[derive(Debug, Clone, Copy)]
pub struct TimerState {
    /// the duration for each pomodoro, in seconds unit
    pub duration: Duration,
    /// from where should the pomodor start counting, this is usefull
    /// to know how many pomodoros we've finished
    pub start: SystemTime,
    /// displayed time
    pub displayed: Duration,
    /// needed util to manage frame update and refresh
    pub util: FrameManager,
    /// Number of Cycles
    pub cycles: u16,
    /// maximum number of cycles
    pub max_cycles: u16,

    pub prev_diff: u16,
}

impl Default for TimerState {
    fn default() -> Self {
        TimerState {
            duration: Duration::from_secs(1500),
            start: SystemTime::now(),
            displayed: Duration::from_secs(1500),
            cycles: 0,
            prev_diff: 0,
            max_cycles: 5,
            util: FrameManager::default(),
        }
    }
}
