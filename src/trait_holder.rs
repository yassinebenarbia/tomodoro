use tui::widgets::StatefulWidget;

use crate::displayable::Displayable;

pub trait TraitHolder: StatefulWidget + StatefulWidget{}
