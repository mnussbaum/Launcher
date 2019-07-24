extern crate gio;
#[macro_use]
extern crate glib;
extern crate glib_sys;
extern crate gobject_sys;
extern crate gtk;

use gtk::prelude::*;
use std;
use std::cell::RefCell;
use std::rc::Rc;

mod main_window;
mod querier;
mod row_data;
mod state;

use main_window::MainWindow;
use querier::Querier;
use state::State;

fn main() {
    // Start up the GTK3 subsystem.
    gtk::init().expect("Unable to start GTK3. Error");

    // Create the main window.
    let gui = Rc::new(MainWindow::new());

    // Set up the application state.
    let state = Rc::new(RefCell::new(State::new()));

    // {
    //     let button = gui.button("halveUpResult");
    //     let gui = Rc::clone(&gui);
    //     let state = Rc::clone(&state);
    //     button.connect_clicked(move |_| {
    //         let mut state = state.borrow_mut();
    //         let prev_value = state.value;
    //         state.value = (f64::from(prev_value) / 2.0).ceil() as i32;
    //         gui.update_from(&state);
    //     });
    // }

    {
        let query_entry = gui.query_entry();
        let gui = Rc::clone(&gui);
        let state = Rc::clone(&state);
        query_entry.connect_changed(move |entry| {
            let mut state = state.borrow_mut();
            state.query_results = Some(Querier::new().query(entry.get_text().unwrap().to_string()));
            gui.update_from(&state);
        });
    }

    gui.start();
    gtk::main();
}
