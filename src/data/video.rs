use std::{
    cmp::Ordering,
    hash::{Hash, Hasher},
};

use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

use crate::time::time_since_formatted;

#[derive(Eq, Clone, Serialize, Deserialize)]
pub struct Video {
    id: String,
    title: String,
    description: String,
    upload_date: DateTime<Local>,
    progress: Option<WatchProgress>,
}

#[derive(Serialize, Deserialize, Eq, PartialEq, Clone, Copy)]
pub struct WatchProgress {
    pub current: u32,
    pub duration: u32,
}

impl Video {
    pub fn new(
        title: impl Into<String>,
        id: impl Into<String>,
        description: impl Into<String>,
        upload_date: DateTime<Local>,
        progress: Option<WatchProgress>,
    ) -> Video {
        Video {
            title: title.into(),
            id: id.into(),
            description: description.into(),
            upload_date,
            progress,
        }
    }

    pub fn url(&self) -> String {
        format!("{}{}", "https://www.youtube.com/watch?v=", self.id)
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn date_relative_str(&self) -> String {
        time_since_formatted(self.upload_date)
    }
}

impl PartialEq for Video {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Ord for Video {
    fn cmp(&self, other: &Self) -> Ordering {
        other.upload_date.cmp(&self.upload_date)
    }
}

impl PartialOrd for Video {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.upload_date.partial_cmp(&self.upload_date)
    }
}

impl Hash for Video {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}
