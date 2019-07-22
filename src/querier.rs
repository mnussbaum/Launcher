use std::borrow::Cow;
use std::io::Write;
use std::process::{Command, Stdio};

pub struct Querier {}

impl Querier {
    pub fn new() -> Self {
        return Self {};
    }

    pub fn query(&self, query_text: String) -> Vec<String> {
        let mut child = Command::new("/bin/ls")
            .arg(query_text)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .expect("failed to execute child");
        //         let mut stdin = child.stdin.as_mut().expect("Failed to open stdin");
        //         stdin.write_all("Hello, world!".as_bytes()).expect(
        //             "Failed to write to
        // stdin",
        //         );

        let output = child.wait_with_output().expect("Failed to read stdout");
        // Kick off commands in a loop
        // Return a stream of results that joins the command outputs
        return String::from_utf8_lossy(&output.stdout)
            .to_string()
            .split("\n")
            .map(String::from)
            .collect();
    }
}
