use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Created {
    pub id: i32,
}

impl Created {
    pub fn new(id: i32) -> Self {
        Self { id }
    }
}
