use uuid::Uuid;

use crate::{line::Line, train::Train};

#[derive(Debug)]
/// A train system.
pub struct System {
    id: Uuid,
    name: String,
    /// All registered trains in the System.
    trains: Vec<Train>,
    /// All registered lines in the System.
    lines: Vec<Line>,
    is_active: bool,
}

impl System {
    pub fn new(name: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            trains: Vec::new(),
            lines: Vec::new(),
            is_active: true,
        }
    }

    /// Returns the count of active trains in the system.
    pub fn active_trains(&self) -> usize {
        todo!();
    }

    /// Returns the count of inactive trains in the system.
    pub fn inactive_trains(&self) -> usize {
        todo!()
    }

    /// Returns the count of active lines in the system.
    pub fn active_lines(&self) -> usize {
        todo!();
    }

    /// Returns the count of inactive lines in the system.
    pub fn inactive_lines(&self) -> usize {
        todo!()
    }
}
