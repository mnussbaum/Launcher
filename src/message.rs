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
pub enum LauncherMessage {
    FocusTextInput,
    InputChanged(String),
    QueryProgress(QueryProgress),
    SubmitInput,
}
