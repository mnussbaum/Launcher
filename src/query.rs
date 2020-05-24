use std::hash::Hash;
use std::process::Stdio;

use nom::IResult;
use nom::bytes::complete::tag;
use nom::sequence::{separated_pair, terminated};
use nom::character::complete::alphanumeric1;
use nom::character::complete::char;
use nom::branch::alt;
use nom::combinator::iterator;
use std::iter::Iterator;
use std::collections::HashMap;
use nom::bytes::complete::escaped;
use nom::bytes::complete::is_not;
use nom::character::complete::line_ending;
use nom::character::complete::anychar;
use nom::sequence::delimited;

use tokio::process::{Child, Command, ChildStdout};
use tokio::io::{BufReader, AsyncBufReadExt, Lines};
use iced_futures::futures;

use crate::QueryProgress;

#[derive(Debug)]
pub struct Query {
    keywords: HashMap<String, String>,
    raw_text: String,
    text: String,
}

impl Query {
    // Parse text into keys, values and remainder text. Values can be single
    // word or multiple words enclosed in double quotes. Quoted strings use \
    // for escaping. Example:
    //
    // key1:"value1" key2:"value2 \" woo" key3:value3 bye
    //
    // Yields:
    //
    // {"key1": "value1", "key2": "value2 \" woo", "key3": "value3"}
    // Remainder text: "bye"
    pub fn new(query_text: String) -> Self {
        // Add line ending just to match against it to determine end of string
        let query_with_line_ending = format!("{}\n", query_text);

        let mut query_iterator = iterator(
            query_with_line_ending.as_str(),
            terminated(separated_pair(
                    alphanumeric1,
                    tag(":"),
                    alt((
                            delimited(
                                char('"'),
                                escaped(is_not(r#"\""#), '\\', anychar),
                                char('"'),
                            ),
                            alphanumeric1,
                    )),
            ), alt((tag(" "), line_ending)))
        );

        let parsed_keywords = query_iterator.map( |(k, v)| {
            (k.to_string(), v.to_string())
        }).collect::<HashMap<_, _>>();
        let parser_result: IResult<_, _> = query_iterator.finish();
        let (remainder_text, ()) = parser_result.unwrap();

        return Query {
            keywords: parsed_keywords,
            raw_text: query_text,
            text: remainder_text.trim().to_string(),
        }
    }

    pub fn get_output(self) -> iced::Subscription<QueryProgress> {
        return iced::Subscription::from_recipe(self)
    }
}

// TODO: Stop processing query
// TODO: Invoke real plugin commands
// TODO: Multiple plugin processes per query
impl<H, I> iced_native::subscription::Recipe<H, I> for Query
where
    H: std::hash::Hasher,
{
    type Output = QueryProgress;

    fn hash(&self, state: &mut H) {
        self.raw_text.hash(state);
    }

    fn stream(
        self: Box<Self>,
        _input: futures::stream::BoxStream<'static, I>,
    ) -> futures::stream::BoxStream<'static, Self::Output> {
        Box::pin(futures::stream::unfold(
            State::Starting(self.text),
            |state: State| async move {
                match state {
                    State::Starting(text) => {
                        let mut plugin_process = Command::new("/usr/bin/ls")
                            .arg(text)
                            .stdout(Stdio::piped())
                            .spawn().expect("whyyy");
                        let stdout = plugin_process.stdout.take().expect("child did not have a handle to stdout");
                        let plugin_output = BufReader::new(stdout).lines();
                        Some((
                                QueryProgress::Started,
                                State::Running {
                                    plugin_process,
                                    plugin_output,
                                },
                        ))
                    },
                    State::Running {
                        plugin_process,
                        mut plugin_output,
                    } => {
                        if let Some(line) =  plugin_output.next_line().await.expect("boom") {
                            Some((
                                    QueryProgress::NewLine(line),
                                    State::Running {
                                        plugin_process,
                                        plugin_output,
                                    },
                            ))
                        } else {
                            let exit_status = plugin_process
                                .await
                                .expect("is this a process failing?");

                            Some((
                                    QueryProgress::Finished {
                                        exit_status,
                                    },
                                    State::Finished,
                            ))
                        }
                    },
                    State::Finished => None,
                }
            },
            ))
    }
}

pub enum State {
    Starting(String),
    Running {
        plugin_process: Child,
        plugin_output: Lines<BufReader<ChildStdout>>,
    },
    Finished,
}
