use serde_derive::{Deserialize, Serialize};

use crate::entry::Entry;

#[derive(Serialize, Deserialize)]
pub struct State {
    pub entry: Entry,
}

impl State {
    pub fn reset(&mut self) {
        self.entry = Entry::new("");
    }
}
