use super::id::Id;
use super::rev::Rev;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ObjectType {
    _id: Id,
    _rev: Rev,
    name: String,
    desc: String,
    created: DateTime<Utc>,
}

impl ObjectType {
    pub fn new() -> Self {
        ObjectType {
            _id: Id::new(),
            _rev: Rev::new(),
            name: "".to_string(),
            desc: "".to_string(),
            created: Utc::now(),
        }

    }
}
