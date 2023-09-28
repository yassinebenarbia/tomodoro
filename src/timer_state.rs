use std::{time::{Duration, SystemTime}, fs::File, io::Write};

use crate::frame_util::FrameManager;

#[derive(Debug, Clone, Copy)]
pub struct TimerState{
    /// the duration for each pomodoro, in seconds unit
    pub duration: Duration,
    /// from where should the pomodor start counting, this is usefull
    /// to know how many pomodoros we've finished
    pub start: SystemTime,
    /// displayed time
    pub displayed: Duration,
    /// needed util to manage frame update and refresh
    pub util: FrameManager,
}

impl Default for TimerState {

    fn default() -> Self {
        TimerState {
            duration: Duration::from_secs(1500),
            start: SystemTime::now(),
            displayed: Duration::from_secs(1500),
            util: FrameManager::default()
        }
    }

}

impl TimerState {
    
    /// duration setter: duration is how long count-down cycle should take in seconds
    pub fn duration<'a>(&'a mut self, duration: Duration) -> TimerState{
        self.duration = duration;
        *self
    }

    /// start setter: from which point in time should count-down be considered
    pub fn start<'a>(&'a mut self, start: SystemTime) -> TimerState{
        self.start = start;
        *self
    }

    /// displayed time setter: the default displayed time
    pub fn displayed<'a>(&'a mut self, displayed: Duration) -> TimerState{
        self.displayed = displayed;
        *self
    }

    pub fn manage_state(&mut self){

        // difference between the current time and the started time as a second
        let mut diff = SystemTime::now().duration_since(self.start).expect("unable to manage time").as_secs();

        // meaning that time has advanced since the beginning of the counter
        // need to do a modulo opperation to get the number of cycles
        if diff > 0 {

            diff %= self.duration.as_secs();
            if self.util.prev_diff.as_secs() < diff {

                self.util.prev_diff(Duration::from_secs(diff));

                self.displayed(
                    self.duration - Duration::from_secs(diff) 
                );
                
            }


        }

    }

}
