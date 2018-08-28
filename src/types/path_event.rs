use std::path::{Path, PathBuf};

use types::event_type::EventType;

pub struct PathEvent {
    pub path: PathBuf,
    pub event: EventType,
}

impl PathEvent {
    pub fn new<P: AsRef<Path>>(path: P, event: EventType) -> PathEvent {
        PathEvent {
            path: path.as_ref().to_owned(),
            event,
        }
    }
}
