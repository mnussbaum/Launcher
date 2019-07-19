#[derive(Debug, PartialEq)]
pub struct State {
    pub error: Option<String>,
    pub query: Option<String>,
}

impl State {
    pub fn new() -> Self {
        return Self {
            error: None,
            query: None
        }
    }
}
