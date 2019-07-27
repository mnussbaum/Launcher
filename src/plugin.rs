use std::process::{Child, Command, Stdio};

use crate::querier::Query;

pub struct Plugin<'plugin> {
    command: &'plugin str,
    // keywords: Vec<String>,
    persistent: bool,
    process: Option<Child>,
    version: semver::Version,
}

impl<'plugin> Plugin<'plugin> {
    pub fn new(command: &'plugin str, persistent: bool, version: semver::Version) -> Self {
        return Plugin {
            command: command,
            persistent: persistent,
            process: None,
            version: version,
        }
    }

    pub fn run_query_if_applicable(&mut self, query: &Query) -> Result<(), String> {
        // Wipe out old processes here first

        self.process = Some(Command::new(self.command)
            .arg(query.text)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .expect("failed to execute child"));
        //         let mut stdin = child.stdin.as_mut().expect("Failed to open stdin");
        //         stdin.write_all("Hello, world!".as_bytes()).expect(
        //             "Failed to write to
        // stdin",
        //         );

        return Ok(())
    }

    pub fn yield_query_output(&mut self) -> Result<Vec<String>, String> {
        if let Some(process) = self.process.take() {
            // TODO: Get output without consuming the process
            let output = process.wait_with_output().expect("Failed to read stdout");

            return Ok(String::from_utf8_lossy(&output.stdout)
                      .to_string()
                      .trim()
                      .split("\n")
                      .map(String::from)
                      .collect())
        }

        Ok(Vec::new())
    }

    pub fn stop_query(&self, query: &Query) -> Result<(), String> {
        return Ok(())
    }
}
