use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
pub struct Occurance {
    id: usize,
    title: String,
    description: String,
    created_at: DateTime<Local>,
    custom_metadata: HashMap<String, String>,
}

impl Occurance {
    fn new(id: usize, title: String, description: String) -> Self {
        Self {
            id,
            title,
            description,
            created_at: Local::now(),
            custom_metadata: HashMap::new(),
        }
    }
}

#[derive(Debug)]
pub struct Event {
    uuid: Uuid,
    title: String,
    description: String,
    occurances: Vec<Occurance>,
}

impl Event {
    pub fn new(title: String, description: String) -> Self {
        Self {
            uuid: Uuid::now_v7(),
            title,
            description,
            occurances: Vec::new(),
        }
    }

    pub fn new_for_map(title: String, description: String) -> (Uuid, Self) {
        let new_event = Self::new(title, description);
        (new_event.uuid, new_event)
    }

    pub fn occur(&mut self, title: String, description: String) {
        let new_occu = Occurance::new(self.occurances.len(), title, description);
        self.occurances.push(new_occu);
    }
}
