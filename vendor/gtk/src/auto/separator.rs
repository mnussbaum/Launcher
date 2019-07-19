// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Buildable;
use Orientable;
use Orientation;
use Widget;
use ffi;
use glib::object::Cast;
use glib::translate::*;
use std::fmt;

glib_wrapper! {
    pub struct Separator(Object<ffi::GtkSeparator, ffi::GtkSeparatorClass, SeparatorClass>) @extends Widget, @implements Buildable, Orientable;

    match fn {
        get_type => || ffi::gtk_separator_get_type(),
    }
}

impl Separator {
    pub fn new(orientation: Orientation) -> Separator {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_separator_new(orientation.to_glib())).unsafe_cast()
        }
    }
}

pub const NONE_SEPARATOR: Option<&Separator> = None;

impl fmt::Display for Separator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Separator")
    }
}