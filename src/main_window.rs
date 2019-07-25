use gio::prelude::*;
use gtk::prelude::*;

use crate::state::State;
use crate::row_data::RowData;

pub struct MainWindow {
    window: gtk::Window,
    results_list: gtk::ListBox,
    query_entry: gtk::Entry,
}

// make moving clones into closures more convenient
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

        let store = gio::ListStore::new(RowData::static_type());
        if let Some(ref query_results) = state.query_results {
            for query_result in query_results.iter() {
                store.append(&RowData::new(&query_result));
            }
        }

        let window_weak = self.window.downgrade();
        self.results_list.bind_model(
            Some(&store),
            clone!(window_weak => move |item| {
                let item = item.downcast_ref::<RowData>().expect("Row data is of wrong type");
                let label = gtk::Label::new(None);
                item.bind_property("result_text", &label, "label")
                    .flags(glib::BindingFlags::DEFAULT | glib::BindingFlags::SYNC_CREATE)
                    .build();

                let box_ = gtk::ListBoxRow::new();
                let hbox = gtk::Box::new(gtk::Orientation::Horizontal, 5);
                hbox.pack_start(&label, true, true, 0);
                box_.add(&hbox);

                box_.connect_activate(clone!(item => move |_| {
                    let maybe_result_text: Option<String> = item
                        .get_property("result_text").unwrap().get();
                    println!("{:?}", maybe_result_text);
                }));

                box_.show_all();

                box_.upcast::<gtk::Widget>()
            }),
        );

        self.results_list.set_activate_on_single_click(true);
        // self.results_list.connect_row_selected(move |wee, woo| {
        //     println!("SELECTED {:?}, {:?}", wee, woo);
        // });
        // self.results_list.connect_row_activated(move |_, activated_row| {
        //     println!("{:?}", activated_row.user_data);
        // });
    }

    pub fn query_entry(&self) -> &gtk::Entry {
        &self.query_entry
    }
}
