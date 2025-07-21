use uuid::Uuid;

#[derive(Debug)]
/// A train stop.
pub struct Stop {
    id: Uuid,
    name: String,
    is_occupied: bool,
    is_active: bool,
}
