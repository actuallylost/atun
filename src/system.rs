use uuid::Uuid;

use crate::line::Line;

#[derive(Debug)]
struct System {
    id: Uuid,
    name: String,
    lines: Vec<Line>,
    active_trains: i16,
    inactive_trains: i16,
}

impl System {
    pub fn new(
        id: Uuid,
        name: String,
        lines: Vec<Line>,
        active_trains: i16,
        inactive_trains: i16,
    ) -> Self {
        Self {
            id,
            name,
            lines,
            active_trains,
            inactive_trains,
        }
    }
}
