#[derive(Debug, Clone)]
pub enum QueryProgress {
    Started,
    NewLine(String),
    Finished {
        exit_status: std::process::ExitStatus,
    },
    Errored,
}

#[derive(Debug, Clone)]
pub enum Message {
    InputChanged(String),
    SubmitInput,
    QueryProgress(QueryProgress),
}
