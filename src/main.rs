extern crate gtk;
extern crate glib;

use std;
use std::rc::Rc;
use std::cell::RefCell;
use gtk::prelude::*;

mod state;
use state::State;
mod main_window;
use main_window::MainWindow;

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
        query_entry.connect_activate(move |_| {
        // query_entry.connect_activate(move |entry| {
            // let spec = entry.get_text().unwrap().to_string();

            let state = state.borrow_mut();
            gui.update_from(&state);
        });
    }

    gui.start();
    gtk::main();
}

