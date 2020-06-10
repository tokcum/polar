use super::id::Id;
use super::rev::Rev;
use chrono::{DateTime, Utc};

#[derive(Debug)]
pub struct DataType {
    _id: Id,
    _rev: Rev,
    name: String,
    desc: String,
    created: DateTime<Utc>,
}

impl DataType {
    pub fn new() -> Self {
        DataType {
            _id: Id::new(),
            _rev: Rev::new(),
            name: "".to_string(),
            desc: "".to_string(),
            created: Utc::now(),
        }

    }
}
