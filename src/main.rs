extern crate glib;
extern crate gtk;
extern crate nom;
extern crate tokio;

use gtk::prelude::*;
use std;
use std::rc::Rc;

mod main_window;
mod querier;

use main_window::MainWindow;
use querier::Querier;

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

    main_window.start();
    gtk::main();
}
