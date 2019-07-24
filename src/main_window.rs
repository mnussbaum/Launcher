use crate::state::State;
use gio::prelude::*;
use gtk::prelude::*;

use row_data::RowData;

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
                let box_ = gtk::ListBoxRow::new();
                let item = item.downcast_ref::<RowData>().expect("Row data is of wrong type");
                box_.show_all();

                box_.upcast::<gtk::Widget>()
            }),
        );

        // self.result.set_text(&format!("{}", state.value));
    }

    pub fn query_entry(&self) -> &gtk::Entry {
        &self.query_entry
    }
}

mod row_data {
    use super::*;

    use glib::subclass;
    use glib::subclass::prelude::*;
    use glib::translate::*;

    // Implementation sub-module of the GObject
    mod imp {
        use super::*;
        use std::cell::RefCell;

        // The actual data structure that stores our values. This is not accessible
        // directly from the outside.
        pub struct RowData {
            hmm: RefCell<Option<String>>,
        }

        // GObject property definitions for our two values
        static PROPERTIES: [subclass::Property; 2] = [subclass::Property("hmm", |hmm| {
            glib::ParamSpec::string(
                hmm,
                "hmm",
                "hmm",
                None, // Default value
                glib::ParamFlags::READWRITE,
            )
        })];

        // Basic declaration of our type for the GObject type system
        impl ObjectSubclass for RowData {
            const NAME: &'static str = "RowData";
            type ParentType = glib::Object;
            type Instance = subclass::simple::InstanceStruct<Self>;
            type Class = subclass::simple::ClassStruct<Self>;

            glib_object_subclass!();

            // Called exactly once before the first instantiation of an instance. This
            // sets up any type-specific things, in this specific case it installs the
            // properties so that GObject knows about their existence and they can be
            // used on instances of our type
            fn class_init(klass: &mut Self::Class) {
                klass.install_properties(&PROPERTIES);
            }

            // Called once at the very beginning of instantiation of each instance and
            // creates the data structure that contains all our state
            fn new() -> Self {
                Self {
                    hmm: RefCell::new(None),
                }
            }
        }

        // The ObjectImpl trait provides the setters/getters for GObject properties.
        // Here we need to provide the values that are internally stored back to the
        // caller, or store whatever new value the caller is providing.
        //
        // This maps between the GObject properties and our internal storage of the
        // corresponding values of the properties.
        impl ObjectImpl for RowData {
            glib_object_impl!();

            fn set_property(&self, _obj: &glib::Object, id: usize, value: &glib::Value) {
                let prop = &PROPERTIES[id];

                match *prop {
                    subclass::Property("hmm", ..) => {
                        let hmm = value.get();
                        self.hmm.replace(hmm);
                    }
                    _ => unimplemented!(),
                }
            }

            fn get_property(&self, _obj: &glib::Object, id: usize) -> Result<glib::Value, ()> {
                let prop = &PROPERTIES[id];

                match *prop {
                    subclass::Property("hmm", ..) => Ok(self.hmm.borrow().to_value()),
                    _ => unimplemented!(),
                }
            }
        }
    }

    // Public part of the RowData type. This behaves like a normal gtk-rs-style GObject
    // binding
    glib_wrapper! {
        pub struct RowData(Object<subclass::simple::InstanceStruct<imp::RowData>, subclass::simple::ClassStruct<imp::RowData>, RowDataClass>);

        match fn {
            get_type => || imp::RowData::get_type().to_glib(),
        }
    }

    // Constructor for new instances. This simply calls glib::Object::new() with
    // initial values for our two properties and then returns the new instance
    impl RowData {
        pub fn new(hmm: &str) -> RowData {
            glib::Object::new(Self::static_type(), &[("hmm", &hmm)])
                .expect("Failed to create row data")
                .downcast()
                .expect("Created row data is of wrong type")
        }
    }
}
