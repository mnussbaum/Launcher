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
    InputChanged(String),
    // TODO: Eventually all these methods should take a query argument so they only operate if their
    // query is still the one being displayed
    EndQuery(()),
    QueryProgress(QueryProgress),
    SubmitInput,
}
