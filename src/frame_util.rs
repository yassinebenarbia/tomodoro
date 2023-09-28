use std::time::Duration;

#[derive(Debug, Clone, Copy)]
pub struct FrameManager{ 
    /// just a second, previousloy time difference
    pub prev_diff: Duration,
}

impl Default for FrameManager {

    fn default() -> Self {
        FrameManager {
            prev_diff: Duration::from_secs(0),
        }
    }
    
}

impl FrameManager {

    pub fn prev_diff(&mut self, prev_diff: Duration)->FrameManager{
        self.prev_diff = prev_diff;
        *self
    }
}
