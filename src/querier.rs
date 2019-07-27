// use std::io::Write;
use std::process::{Command, Stdio};

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

        let child = Command::new("/bin/ls")
            .arg(query.text)
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
            .trim()
            .split("\n")
            .map(String::from)
            .collect();
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

        let mut nom_it = iterator(
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

        let parsed_keywords = nom_it.map(|(k, v)| (k.to_string(), v.to_string())).collect::<HashMap<_, _>>();
        let parser_result: IResult<_, _> = nom_it.finish();
        let (remainder_text, ()) = parser_result.unwrap();

        return Query {
            keywords: parsed_keywords,
            raw_text: query_text,
            text: remainder_text.trim().to_string(),
        }
    }
}
