use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
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

    pub fn title(&self) -> &String {
        &self.title
    }

    pub fn id(&self) -> &Uuid {
        &self.uuid
    }
}

impl fmt::Display for Event {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let last_four = last_four_chars(self.uuid.to_string());
        write!(
            f,
            "Event ( uuid: {}, title: {}, occurances: WIP )",
            last_four, self.title
        )
    }
}

// Misc

fn last_four_chars(s: String) -> String {
    let char_count = s.chars().count();
    if char_count <= 4 {
        return s; // Return entire string if <= 4 chars
    }
    let start = s.char_indices().nth(char_count - 4).unwrap().0;
    s[start..].to_string()
}
