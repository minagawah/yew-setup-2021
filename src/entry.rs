use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Entry {
    pub description: String,
    pub editing: bool,
}

impl Entry {
    pub fn new(description: &str) -> Self {
        Entry {
            description: description.into(),
            editing: false,
        }
    }
}
