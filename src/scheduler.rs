use uuid::Uuid;

use crate::{controller::Controller, system::System};

/// Handles the train traffic in the assigned systems.
#[derive(Debug)]
pub struct Scheduler {
    id: Uuid,
    systems: Vec<System>,
    controller: Controller,
}
