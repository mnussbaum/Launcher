// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use AppChooser;
use Bin;
use Buildable;
use Container;
use Dialog;
use DialogFlags;
use Widget;
use Window;
use ffi;
use gio;
use glib::GString;
use glib::StaticType;
use glib::Value;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect_raw;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib_wrapper! {
    pub struct AppChooserDialog(Object<ffi::GtkAppChooserDialog, ffi::GtkAppChooserDialogClass, AppChooserDialogClass>) @extends Dialog, Window, Bin, Container, Widget, @implements Buildable, AppChooser;

    match fn {
        get_type => || ffi::gtk_app_chooser_dialog_get_type(),
    }
}

impl AppChooserDialog {
    pub fn new<'a, P: IsA<Window> + 'a, Q: Into<Option<&'a P>>, R: IsA<gio::File>>(parent: Q, flags: DialogFlags, file: &R) -> AppChooserDialog {
        assert_initialized_main_thread!();
        let parent = parent.into();
        unsafe {
            Widget::from_glib_none(ffi::gtk_app_chooser_dialog_new(parent.map(|p| p.as_ref()).to_glib_none().0, flags.to_glib(), file.as_ref().to_glib_none().0)).unsafe_cast()
        }
    }

    pub fn new_for_content_type<'a, P: IsA<Window> + 'a, Q: Into<Option<&'a P>>>(parent: Q, flags: DialogFlags, content_type: &str) -> AppChooserDialog {
        assert_initialized_main_thread!();
        let parent = parent.into();
        unsafe {
            Widget::from_glib_none(ffi::gtk_app_chooser_dialog_new_for_content_type(parent.map(|p| p.as_ref()).to_glib_none().0, flags.to_glib(), content_type.to_glib_none().0)).unsafe_cast()
        }
    }
}

pub const NONE_APP_CHOOSER_DIALOG: Option<&AppChooserDialog> = None;

pub trait AppChooserDialogExt: 'static {
    fn get_heading(&self) -> Option<GString>;

    fn get_widget(&self) -> Widget;

    fn set_heading(&self, heading: &str);

    fn get_property_gfile(&self) -> Option<gio::File>;

    fn connect_property_heading_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<AppChooserDialog>> AppChooserDialogExt for O {
    fn get_heading(&self) -> Option<GString> {
        unsafe {
            from_glib_none(ffi::gtk_app_chooser_dialog_get_heading(self.as_ref().to_glib_none().0))
        }
    }

    fn get_widget(&self) -> Widget {
        unsafe {
            from_glib_none(ffi::gtk_app_chooser_dialog_get_widget(self.as_ref().to_glib_none().0))
        }
    }

    fn set_heading(&self, heading: &str) {
        unsafe {
            ffi::gtk_app_chooser_dialog_set_heading(self.as_ref().to_glib_none().0, heading.to_glib_none().0);
        }
    }

    fn get_property_gfile(&self) -> Option<gio::File> {
        unsafe {
            let mut value = Value::from_type(<gio::File as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0 as *mut gobject_ffi::GObject, b"gfile\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get()
        }
    }

    fn connect_property_heading_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::heading\0".as_ptr() as *const _,
                Some(transmute(notify_heading_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }
}

unsafe extern "C" fn notify_heading_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GtkAppChooserDialog, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<AppChooserDialog> {
    let f: &F = transmute(f);
    f(&AppChooserDialog::from_glib_borrow(this).unsafe_cast())
}

impl fmt::Display for AppChooserDialog {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "AppChooserDialog")
    }
}