use uuid::Uuid;

use crate::line::Line;

#[derive(Debug)]
/// A train.
pub struct Train {
    id: Uuid,
    /// The line which the train is currently active in.
    active_line: Line,
    is_active: bool,
}
