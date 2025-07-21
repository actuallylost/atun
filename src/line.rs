use uuid::Uuid;

use crate::stop::Stop;

#[derive(Debug)]
pub struct Line {
    id: Uuid,
    name: String,
    stops: Vec<Stop>,
}

impl Line {
    pub fn new(id: Uuid, name: String, stops: Vec<Stop>) -> Self {
        Self { id, name, stops }
    }
}
