use tui::{
    Terminal,
    backend::Backend
};

// Terminal of choice
// backend "crossterm or termion"
#[derive(Debug)]
pub struct TWrapper<T: Backend>{
    pub terminal: Terminal<T>
}

#[derive(Debug)]
pub struct BWrapper<T: Backend>{
    pub backend: T
}

#[derive(Debug)]
pub struct Backbone<T:Backend>{
    pub backend: BWrapper<T>,
    pub terminal: TWrapper<T>,
}
