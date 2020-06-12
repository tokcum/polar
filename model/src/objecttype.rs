use crate::id::Id;
use crate::rev::Rev;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[cfg(not(target_arch = "wasm32"))]
use rusqlite::types::{FromSql, FromSqlResult, ValueRef, FromSqlError};

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

#[cfg(not(target_arch = "wasm32"))]
impl FromSql for ObjectType {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        match value {
            ValueRef::Text(s) => serde_json::from_slice(s),
            _ => return Err(FromSqlError::InvalidType),
        }
            .map_err(|err| FromSqlError::Other(Box::new(err)))
    }
}
