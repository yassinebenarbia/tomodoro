use tui::widgets::ListState;

#[derive(Debug)]
pub struct Event {
    pub items: Vec<String>,
    pub state: ListState
}
impl Event {

    pub fn new(items: Vec<String>) -> Event {
        Event{
            items,
            state: ListState::default()
        }
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));

    }
    
}
