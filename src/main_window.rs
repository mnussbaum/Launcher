use gtk::prelude::*;

pub struct MainWindow {
    window: gtk::Window,
    results_store: gtk::ListStore,
    query_entry: gtk::Entry,
}

macro_rules! clone {
    (@param _) => ( _ );
    (@param $x:ident) => ( $x );
    ($($n:ident),+ => move || $body:expr) => (
        {
            $( let $n = $n.clone(); )+
            move || $body
        }
    );
    ($($n:ident),+ => move |$($p:tt),+| $body:expr) => (
        {
            $( let $n = $n.clone(); )+
            move |$(clone!(@param $p),)+| $body
        }
    );
}

impl MainWindow {
    pub fn new() -> MainWindow {
        let glade_src = include_str!("mainwindow.glade");
        let builder = gtk::Builder::new_from_string(glade_src);

        let window: gtk::Window = builder.get_object("mainWindow").unwrap();
        let query_entry: gtk::Entry = builder.get_object("queryEntry").unwrap();

        let results_view: gtk::TreeView = builder.get_object("resultsList").unwrap();;
        results_view.append_column(&MainWindow::build_results_column());

        let results_store = gtk::ListStore::new(&[gtk::Type::String]);
        results_view.set_model(Some(&results_store));

        results_view.connect_row_activated(clone!(results_view => move |_, _, _| {
            let selection = results_view.get_selection();
            selection.get_selected().map(|(model, iter)| {
                let hmm: Option<String> = model.get_value(&iter, 0).get();
                println!("{:?}", hmm.unwrap());
            });
        }));

        MainWindow {
            window,
            results_store,
            query_entry,
        }
    }

    pub fn build_results_column() -> gtk::TreeViewColumn {
        let column = gtk::TreeViewColumn::new();
        let cell = gtk::CellRendererText::new();
        column.set_sort_column_id(0);
        column.set_title("results");
        column.set_visible(true);
        column.pack_start(&cell, true);
        column.add_attribute(&cell, "text", 0);

        return column
    }

    pub fn start(&self) {
        glib::set_application_name("launcher");
        self.window.connect_delete_event(|_, _| {
            gtk::main_quit();
            Inhibit(false)
        });
        self.window.show_all();
    }

    pub fn update_from(&self, query_results: Vec<String>) {
        // For some reason only the first line of the initial insert shows, and
        // then later inserts show all values. This line works around this quirk
        self.results_store.insert_with_values(None, &[0], &[&""]);

        self.results_store.clear();
        for query_result in query_results.iter() {
            self.results_store.insert_with_values(None, &[0], &[&query_result]);
        }
    }

    pub fn query_entry(&self) -> &gtk::Entry {
        &self.query_entry
    }
}
