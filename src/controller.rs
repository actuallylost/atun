use uuid::Uuid;

use crate::system::System;

/// The system controller.
#[derive(Debug)]
pub struct Controller {
    id: Uuid,
    system: System,
}
