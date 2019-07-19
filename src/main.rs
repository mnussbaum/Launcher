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
        query_entry.connect_changed( move |entry| {
            let mut state = state.borrow_mut();
            state.query = Some(entry.get_text().unwrap().to_string());
            gui.update_from(&state);
        });
    }

    gui.start();
    gtk::main();
}

