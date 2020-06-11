use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Id {
    Id(String)
}

impl Id {
    pub fn new() -> Self {
        Id::Id("1".to_string())
    }
}
