use uuid::Uuid;

#[derive(Debug)]
pub struct Stop {
    id: Uuid,
    name: String,
}

impl Stop {
    pub fn new(id: Uuid, name: String) -> Self {
        Self { id, name }
    }
}
