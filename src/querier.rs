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

use crate::plugin::Plugin;

#[derive(Debug)]
pub struct Query<'query> {
    pub keywords: HashMap<&'query str, &'query str>,
    raw_text: &'query str,
    pub text: &'query str,
}

pub struct Querier<'querier> {
    plugins: Vec<Plugin<'querier>>,
}

impl<'querier> Querier<'querier> {
    pub fn new() -> Self {
        return Self {
            plugins: vec![
                Plugin::new("/bin/ls", false, semver::Version::parse("1.0.0").unwrap()),
            ],
        };
    }

    pub fn query(&mut self, query_text: &str) -> Vec<String> {
        let query_with_line_ending = format!("{}\n", query_text);
        let query = self.parse_query_text(query_with_line_ending.as_str());
        for plugin in self.plugins.iter_mut() {
            plugin.run_query_if_applicable(&query).unwrap();
        }

        let mut query_results: Vec<String> = Vec::new();
        for ref mut plugin in self.plugins.iter_mut() {
            query_results.extend(plugin.yield_query_output().unwrap());
        }

        return query_results
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
    pub fn parse_query_text<'query_text>(&self, query_text: &'query_text str) -> Query<'query_text> {
        // Add line ending just to match against it to determine end of string

        let mut query_iterator = iterator(
            query_text.as_ref(),
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

        let parsed_keywords = query_iterator.map( |(k, v): (&str, &str)| {
            (k, v)
        }).collect::<HashMap<_, _>>();
        let parser_result: IResult<_, _> = query_iterator.finish();
        let (remainder_text, ()) = parser_result.unwrap();

        return Query {
            keywords: parsed_keywords,
            raw_text: query_text,
            text: remainder_text.trim(),
        }
    }
}
