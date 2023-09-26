use std::time::{Duration, SystemTime};

#[derive(Debug)]
pub struct TimerState{
    /// the duration for each pomodoro, in seconds unit
    pub duration: Duration,
    /// from where should the pomodor start counting, this is usefull
    /// to know how many pomodoros we've finished
    pub start: SystemTime,
}
impl Default for TimerState {

    fn default() -> Self {
        TimerState {
            duration: Duration::from_secs(1500),
            start: SystemTime::now() 
        }
    }

}
