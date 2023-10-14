use std::{time::{Duration, SystemTime}, fs::File, io::Write};

use crate::frame_util::FrameManager;

/// This resemble the state of the timer, meaning that the behavior 
/// of the shown timer, e.g number of cycles, cycle length, starting time,
/// the displayed time at the start, etc
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

    /// Cycles setter: number of approved cycles
    pub fn cycle<'a>(&'a mut self, cycles: u16) -> TimerState{
        self.cycles = cycles;
        *self
    }

    /// increment Cycles by num
    pub fn inc_cycle<'a>(&'a mut self, num: u16) -> TimerState{
        self.cycles+=num;
        *self
    }

    /// decrement Cycles by num
    pub fn dic_cycle<'a>(&'a mut self, num: u16) -> TimerState{
        if self.cycles > 0 {
            self.cycles-=num;  
        }
        *self
    }

    /// get the current cycles number
    pub fn get_cycle<'a>(&'a mut self)->u16{self.cycles}

    /// Max cycles setter: the maximum number of allowed cycles
    pub fn max_cycles<'a>(&'a mut self, max: u16) -> TimerState{
        self.max_cycles = max;
        *self
    }

    /// increment the maximum number of cycles by num amount
    pub fn inc_max_cycle<'a>(&'a mut self, num: u16) -> TimerState{
        self.max_cycles+=num;
        *self
    }

    /// decrement the maximum number of cycles by num amount
    pub fn dic_max_cycle<'a>(&'a mut self, num: u16) -> TimerState{
        self.max_cycles-=num;
        *self
    }

    /// get the maximum number of cycles
    pub fn get_max_cycle<'a>(&'a mut self) -> u16{
        self.max_cycles
    }

    pub fn prev_diff<'a>(&'a mut self, prev_diff: u16) -> TimerState{
        self.prev_diff = prev_diff;
        *self
    }

    pub fn manage_state(&mut self){

        // difference between the current time and the started time as a second

        let mut raw_diff = SystemTime::now()
            .duration_since(self.start)
            .expect("unable to manage time")
            .as_secs();

        let mut diff = raw_diff;

        // meaning that time has advanced since the beginning of the counter
        // need to do a modulo opperation to get the number of cycles

        if diff > 0 {

            if self.util.prev_diff.as_secs() < diff {

                diff %= self.duration.as_secs();

                self.util.prev_diff(Duration::from_secs(diff));

                self.displayed(
                    self.duration - Duration::from_secs(diff) 
                );
                
            }

            // thus a full cycle is completed
            if diff == self.duration.as_secs() - 1 && self.prev_diff < raw_diff as u16 {

                self.prev_diff = raw_diff as u16;

                // another check, to know wether or not the 
                // previous call is different or not from the current

                self.inc_cycle(1);

                let mut f = File::create("thing.txt")
                    .expect("Couldn't create File");
                f.write(self.get_cycle().to_string().as_bytes())
                    .expect("Couldn't write to file");

                
            }


        }

    }

}
