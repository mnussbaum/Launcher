use crate::state::State;
use gtk::prelude::*;

pub struct MainWindow {
    window: gtk::Window,
    results_list: gtk::ListBox,
    query_entry: gtk::Entry,
}

impl MainWindow {
    pub fn new() -> MainWindow {
        // Initialize the UI from the Glade XML.
        let glade_src = include_str!("mainwindow.glade");
        let builder = gtk::Builder::new_from_string(glade_src);

        // Get handles for the various controls we need to use.
        let window: gtk::Window = builder.get_object("mainWindow").unwrap();
        let results_list: gtk::ListBox = builder.get_object("resultsList").unwrap();
        let query_entry: gtk::Entry = builder.get_object("queryEntry").unwrap();

        MainWindow {
            window,
            results_list,
            query_entry,
        }
    }

    // Set up naming for the window and show it to the user.
    pub fn start(&self) {
        glib::set_application_name("launcher");
        self.window.set_wmclass("Launcher", "Launcher");
        self.window.connect_delete_event(|_, _| {
            gtk::main_quit();
            Inhibit(false)
        });
        self.window.show_all();
    }

    pub fn update_from(&self, state: &State) {
        println!("{:?}", state);
        if let Some(ref err) = state.error {
        } else {
        }
        let col_types: [gtk::Type; 1] = [gtk::Type::String];
        let col_indices: [u32; 1] = [0];
        let store = gtk::ListStore::new(&col_types);
        if let Some(ref query_results) = state.query_results {
            for query_result in query_results.iter() {
                let values: [&dyn ToValue; 1] = [&query_result];
                store.set(&store.append(), &col_indices, &values);
            }
        }
        self.results_list.bind_model(Some(&store), clone!(window_weak => move |item| {
        let box_ = gtk::ListBoxRow::new();
        }

        // self.result.set_text(&format!("{}", state.value));
    }

    pub fn query_entry(&self) -> &gtk::Entry {
        &self.query_entry
    }
}
