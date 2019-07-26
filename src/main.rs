extern crate glib;
extern crate gtk;
extern crate nom;

use gtk::prelude::*;
use std;
use std::rc::Rc;

mod main_window;
mod querier;

use main_window::MainWindow;
use querier::Querier;

use nom::IResult;
use nom::bytes::complete::tag;
use nom::sequence::{separated_pair, terminated};
use nom::character::complete::alphanumeric1;
use nom::character::complete::char;
use nom::combinator::iterator;
use std::iter::Iterator;
use std::collections::HashMap;
use nom::bytes::complete::escaped;
use nom::character::complete::anychar;
use nom::character::complete::one_of;
use nom::sequence::delimited;


fn main() {
    gtk::init().expect("Unable to start GTK3. Error");
    let main_window = Rc::new(MainWindow::new());

    {
        let query_entry = main_window.query_entry();
        let main_window = Rc::clone(&main_window);
        query_entry.connect_changed(move |entry| {
            main_window.update_from(Querier::new().query(
                entry.get_text().unwrap().to_string()
            ));
        });
    }


    // expirimenting with query parsing
    let data = "key1:'value1' key2:'value2 woo' key3:value3 ;";

    let mut nom_it = iterator(data, terminated(separated_pair(
	alphanumeric1,
	tag(":"),
	delimited(tag("'"), alphanumeric1, tag("'")), 
    ), tag(" ")));
    // replace the alphanumeric1 middle of delimited with something that handles spaces
    // maybe allow for escaping
    // handle double quotes
    // handle non-quoted single words

    let res = nom_it.map(|(k, v)| (k.to_uppercase(), v)).collect::<HashMap<_, _>>();

    let parser_result: IResult<_, _> = nom_it.finish();
    let (remaining_input, ()) = parser_result.unwrap();

    println!("iterator returned {:?}, remaining input is '{}'", res, remaining_input);

    main_window.start();
    gtk::main();
}
