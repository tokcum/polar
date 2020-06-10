#[derive(Debug)]
pub enum Rev {
    Rev(String)
}

impl Rev {
    pub fn new() -> Self {
        Rev::Rev("1-1".to_string())
    }
}
