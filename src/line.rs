use uuid::Uuid;

use crate::{stop::Stop, train::Train};

#[derive(Debug)]
/// A train line.
pub struct Line {
    id: Uuid,
    name: String,
    /// All registered trains in the line.
    trains: Vec<Train>,
    /// All registered stops in the line.
    stops: Vec<Stop>,
    is_active: bool,
}

impl Line {
    /// Returns the count of active trains in the line.
    pub fn active_trains(&self) -> usize {
        todo!();
    }

    /// Returns the count of active stops in the line.
    pub fn active_stops(&self) -> usize {
        todo!();
    }
}
