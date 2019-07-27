use std::process::{Command, Stdio};
use std::time::Duration;

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

use tokio::prelude::*;
use tokio_process::CommandExt;

#[derive(Debug)]
pub struct Query {
    keywords: HashMap<String, String>,
    raw_text: String,
    text: String,
}

pub struct Querier {}

impl Querier {
    pub fn new() -> Self {
        return Self {};
    }

    pub fn query(&self, query_text: String) -> Vec<String> {
        let query = self.parse_query_text(query_text);

        let mut runtime = tokio::runtime::Runtime::new().expect("Could not create tokio runtime!");
        let mut plugin_process = Command::new("/usr/bin/ls")
            .arg(query.text)
            .stdin(Stdio::piped())
            .stderr(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn_async()
            .expect("Failed to start player 1!");
        // let mut player1_input = player1_process.stdin().take().unwrap();
        // let fut = tokio_io::io::write_all(player1_input, "uci\n".as_bytes());
        // player1_input = runtime.block_on(fut).expect("Couldn't write").0;
        // let fut = tokio_io::io::write_all(player1_input, "isready\n".as_bytes());
        // player1_input = runtime.block_on(fut).expect("Couldn't write").0;
        let plugin_output = plugin_process.stdout().take().unwrap();
        let lines_codec = tokio::codec::LinesCodec::new();
        let line_fut = tokio::codec::FramedRead::new(plugin_output, lines_codec)
            .into_future()
            .timeout(Duration::from_millis(3000));
        let result = runtime.block_on(line_fut);
        match result {
            Ok(s) => match s.0 {
                Some(str) => {
                    return str.trim()
                        .split("\n")
                        .map(String::from)
                        .collect();
                }
                None => {
                    return Vec::new();
                }
            },
            Err(e) => {
                    return Vec::new();
            },
        }
    }

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
    pub fn parse_query_text(&self, query_text: String) -> Query {
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
}
