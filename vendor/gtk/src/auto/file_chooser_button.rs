// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Box;
use Buildable;
use Container;
use Dialog;
use FileChooser;
use FileChooserAction;
use Orientable;
use Widget;
use ffi;
use glib::GString;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect_raw;
use glib::translate::*;
use glib_ffi;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib_wrapper! {
    pub struct FileChooserButton(Object<ffi::GtkFileChooserButton, ffi::GtkFileChooserButtonClass, FileChooserButtonClass>) @extends Box, Container, Widget, @implements Buildable, Orientable, FileChooser;

    match fn {
        get_type => || ffi::gtk_file_chooser_button_get_type(),
    }
}

impl FileChooserButton {
    pub fn new(title: &str, action: FileChooserAction) -> FileChooserButton {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_file_chooser_button_new(title.to_glib_none().0, action.to_glib())).unsafe_cast()
        }
    }

    pub fn new_with_dialog<P: IsA<Dialog>>(dialog: &P) -> FileChooserButton {
        skip_assert_initialized!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_file_chooser_button_new_with_dialog(dialog.as_ref().to_glib_none().0)).unsafe_cast()
        }
    }
}

pub const NONE_FILE_CHOOSER_BUTTON: Option<&FileChooserButton> = None;

pub trait FileChooserButtonExt: 'static {
    #[cfg_attr(feature = "v3_20", deprecated)]
    #[cfg(any(not(feature = "v3_20"), feature = "dox"))]
    fn get_focus_on_click(&self) -> bool;

    fn get_title(&self) -> Option<GString>;

    fn get_width_chars(&self) -> i32;

    #[cfg_attr(feature = "v3_20", deprecated)]
    #[cfg(any(not(feature = "v3_20"), feature = "dox"))]
    fn set_focus_on_click(&self, focus_on_click: bool);

    fn set_title(&self, title: &str);

    fn set_width_chars(&self, n_chars: i32);

    fn connect_file_set<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_title_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_width_chars_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<FileChooserButton>> FileChooserButtonExt for O {
    #[cfg(any(not(feature = "v3_20"), feature = "dox"))]
    fn get_focus_on_click(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_file_chooser_button_get_focus_on_click(self.as_ref().to_glib_none().0))
        }
    }

    fn get_title(&self) -> Option<GString> {
        unsafe {
            from_glib_none(ffi::gtk_file_chooser_button_get_title(self.as_ref().to_glib_none().0))
        }
    }

    fn get_width_chars(&self) -> i32 {
        unsafe {
            ffi::gtk_file_chooser_button_get_width_chars(self.as_ref().to_glib_none().0)
        }
    }

    #[cfg(any(not(feature = "v3_20"), feature = "dox"))]
    fn set_focus_on_click(&self, focus_on_click: bool) {
        unsafe {
            ffi::gtk_file_chooser_button_set_focus_on_click(self.as_ref().to_glib_none().0, focus_on_click.to_glib());
        }
    }

    fn set_title(&self, title: &str) {
        unsafe {
            ffi::gtk_file_chooser_button_set_title(self.as_ref().to_glib_none().0, title.to_glib_none().0);
        }
    }

    fn set_width_chars(&self, n_chars: i32) {
        unsafe {
            ffi::gtk_file_chooser_button_set_width_chars(self.as_ref().to_glib_none().0, n_chars);
        }
    }

    fn connect_file_set<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"file-set\0".as_ptr() as *const _,
                Some(transmute(file_set_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_title_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::title\0".as_ptr() as *const _,
                Some(transmute(notify_title_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_width_chars_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::width-chars\0".as_ptr() as *const _,
                Some(transmute(notify_width_chars_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }
}

unsafe extern "C" fn file_set_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GtkFileChooserButton, f: glib_ffi::gpointer)
where P: IsA<FileChooserButton> {
    let f: &F = transmute(f);
    f(&FileChooserButton::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_title_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GtkFileChooserButton, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<FileChooserButton> {
    let f: &F = transmute(f);
    f(&FileChooserButton::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_width_chars_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GtkFileChooserButton, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<FileChooserButton> {
    let f: &F = transmute(f);
    f(&FileChooserButton::from_glib_borrow(this).unsafe_cast())
}

impl fmt::Display for FileChooserButton {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "FileChooserButton")
    }
}