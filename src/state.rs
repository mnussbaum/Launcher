#[derive(Debug, PartialEq)]
pub struct State {
    pub error: Option<String>,
    pub query: Option<String>,
    pub query_results: Option<Vec<String>>,
}

impl State {
    pub fn new() -> Self {
        return Self {
            error: None,
            query: None,
            query_results: None,
        };
    }
}
