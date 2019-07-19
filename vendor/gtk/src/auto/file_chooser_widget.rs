// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Box;
use Buildable;
use Container;
use FileChooser;
use FileChooserAction;
use Orientable;
use Widget;
use ffi;
use glib;
use glib::GString;
use glib::StaticType;
use glib::Value;
use glib::object::Cast;
use glib::object::IsA;
use glib::object::ObjectExt;
use glib::signal::SignalHandlerId;
use glib::signal::connect_raw;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use libc;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib_wrapper! {
    pub struct FileChooserWidget(Object<ffi::GtkFileChooserWidget, ffi::GtkFileChooserWidgetClass, FileChooserWidgetClass>) @extends Box, Container, Widget, @implements Buildable, Orientable, FileChooser;

    match fn {
        get_type => || ffi::gtk_file_chooser_widget_get_type(),
    }
}

impl FileChooserWidget {
    pub fn new(action: FileChooserAction) -> FileChooserWidget {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_file_chooser_widget_new(action.to_glib())).unsafe_cast()
        }
    }
}

pub const NONE_FILE_CHOOSER_WIDGET: Option<&FileChooserWidget> = None;

pub trait FileChooserWidgetExt: 'static {
    fn get_property_search_mode(&self) -> bool;

    fn set_property_search_mode(&self, search_mode: bool);

    fn get_property_subtitle(&self) -> Option<GString>;

    fn connect_desktop_folder<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn emit_desktop_folder(&self);

    fn connect_down_folder<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn emit_down_folder(&self);

    fn connect_home_folder<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn emit_home_folder(&self);

    fn connect_location_popup<F: Fn(&Self, &str) + 'static>(&self, f: F) -> SignalHandlerId;

    fn emit_location_popup(&self, path: &str);

    fn connect_location_popup_on_paste<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn emit_location_popup_on_paste(&self);

    fn connect_location_toggle_popup<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn emit_location_toggle_popup(&self);

    fn connect_places_shortcut<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn emit_places_shortcut(&self);

    fn connect_quick_bookmark<F: Fn(&Self, i32) + 'static>(&self, f: F) -> SignalHandlerId;

    fn emit_quick_bookmark(&self, bookmark_index: i32);

    fn connect_recent_shortcut<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn emit_recent_shortcut(&self);

    fn connect_search_shortcut<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn emit_search_shortcut(&self);

    fn connect_show_hidden<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn emit_show_hidden(&self);

    fn connect_up_folder<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn emit_up_folder(&self);

    fn connect_property_search_mode_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_subtitle_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<FileChooserWidget>> FileChooserWidgetExt for O {
    fn get_property_search_mode(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0 as *mut gobject_ffi::GObject, b"search-mode\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn set_property_search_mode(&self, search_mode: bool) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0 as *mut gobject_ffi::GObject, b"search-mode\0".as_ptr() as *const _, Value::from(&search_mode).to_glib_none().0);
        }
    }

    fn get_property_subtitle(&self) -> Option<GString> {
        unsafe {
            let mut value = Value::from_type(<GString as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0 as *mut gobject_ffi::GObject, b"subtitle\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get()
        }
    }

    fn connect_desktop_folder<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"desktop-folder\0".as_ptr() as *const _,
                Some(transmute(desktop_folder_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn emit_desktop_folder(&self) {
        let _ = unsafe { glib::Object::from_glib_borrow(self.to_glib_none().0 as *mut gobject_ffi::GObject).emit("desktop-folder", &[]).unwrap() };
    }

    fn connect_down_folder<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"down-folder\0".as_ptr() as *const _,
                Some(transmute(down_folder_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn emit_down_folder(&self) {
        let _ = unsafe { glib::Object::from_glib_borrow(self.to_glib_none().0 as *mut gobject_ffi::GObject).emit("down-folder", &[]).unwrap() };
    }

    fn connect_home_folder<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"home-folder\0".as_ptr() as *const _,
                Some(transmute(home_folder_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn emit_home_folder(&self) {
        let _ = unsafe { glib::Object::from_glib_borrow(self.to_glib_none().0 as *mut gobject_ffi::GObject).emit("home-folder", &[]).unwrap() };
    }

    fn connect_location_popup<F: Fn(&Self, &str) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"location-popup\0".as_ptr() as *const _,
                Some(transmute(location_popup_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn emit_location_popup(&self, path: &str) {
        let _ = unsafe { glib::Object::from_glib_borrow(self.to_glib_none().0 as *mut gobject_ffi::GObject).emit("location-popup", &[&path]).unwrap() };
    }

    fn connect_location_popup_on_paste<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"location-popup-on-paste\0".as_ptr() as *const _,
                Some(transmute(location_popup_on_paste_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn emit_location_popup_on_paste(&self) {
        let _ = unsafe { glib::Object::from_glib_borrow(self.to_glib_none().0 as *mut gobject_ffi::GObject).emit("location-popup-on-paste", &[]).unwrap() };
    }

    fn connect_location_toggle_popup<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"location-toggle-popup\0".as_ptr() as *const _,
                Some(transmute(location_toggle_popup_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn emit_location_toggle_popup(&self) {
        let _ = unsafe { glib::Object::from_glib_borrow(self.to_glib_none().0 as *mut gobject_ffi::GObject).emit("location-toggle-popup", &[]).unwrap() };
    }

    fn connect_places_shortcut<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"places-shortcut\0".as_ptr() as *const _,
                Some(transmute(places_shortcut_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn emit_places_shortcut(&self) {
        let _ = unsafe { glib::Object::from_glib_borrow(self.to_glib_none().0 as *mut gobject_ffi::GObject).emit("places-shortcut", &[]).unwrap() };
    }

    fn connect_quick_bookmark<F: Fn(&Self, i32) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"quick-bookmark\0".as_ptr() as *const _,
                Some(transmute(quick_bookmark_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn emit_quick_bookmark(&self, bookmark_index: i32) {
        let _ = unsafe { glib::Object::from_glib_borrow(self.to_glib_none().0 as *mut gobject_ffi::GObject).emit("quick-bookmark", &[&bookmark_index]).unwrap() };
    }

    fn connect_recent_shortcut<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"recent-shortcut\0".as_ptr() as *const _,
                Some(transmute(recent_shortcut_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn emit_recent_shortcut(&self) {
        let _ = unsafe { glib::Object::from_glib_borrow(self.to_glib_none().0 as *mut gobject_ffi::GObject).emit("recent-shortcut", &[]).unwrap() };
    }

    fn connect_search_shortcut<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"search-shortcut\0".as_ptr() as *const _,
                Some(transmute(search_shortcut_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn emit_search_shortcut(&self) {
        let _ = unsafe { glib::Object::from_glib_borrow(self.to_glib_none().0 as *mut gobject_ffi::GObject).emit("search-shortcut", &[]).unwrap() };
    }

    fn connect_show_hidden<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"show-hidden\0".as_ptr() as *const _,
                Some(transmute(show_hidden_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn emit_show_hidden(&self) {
        let _ = unsafe { glib::Object::from_glib_borrow(self.to_glib_none().0 as *mut gobject_ffi::GObject).emit("show-hidden", &[]).unwrap() };
    }

    fn connect_up_folder<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"up-folder\0".as_ptr() as *const _,
                Some(transmute(up_folder_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn emit_up_folder(&self) {
        let _ = unsafe { glib::Object::from_glib_borrow(self.to_glib_none().0 as *mut gobject_ffi::GObject).emit("up-folder", &[]).unwrap() };
    }

    fn connect_property_search_mode_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::search-mode\0".as_ptr() as *const _,
                Some(transmute(notify_search_mode_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_subtitle_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::subtitle\0".as_ptr() as *const _,
                Some(transmute(notify_subtitle_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }
}

unsafe extern "C" fn desktop_folder_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GtkFileChooserWidget, f: glib_ffi::gpointer)
where P: IsA<FileChooserWidget> {
    let f: &F = transmute(f);
    f(&FileChooserWidget::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn down_folder_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GtkFileChooserWidget, f: glib_ffi::gpointer)
where P: IsA<FileChooserWidget> {
    let f: &F = transmute(f);
    f(&FileChooserWidget::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn home_folder_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GtkFileChooserWidget, f: glib_ffi::gpointer)
where P: IsA<FileChooserWidget> {
    let f: &F = transmute(f);
    f(&FileChooserWidget::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn location_popup_trampoline<P, F: Fn(&P, &str) + 'static>(this: *mut ffi::GtkFileChooserWidget, path: *mut libc::c_char, f: glib_ffi::gpointer)
where P: IsA<FileChooserWidget> {
    let f: &F = transmute(f);
    f(&FileChooserWidget::from_glib_borrow(this).unsafe_cast(), &GString::from_glib_borrow(path))
}

unsafe extern "C" fn location_popup_on_paste_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GtkFileChooserWidget, f: glib_ffi::gpointer)
where P: IsA<FileChooserWidget> {
    let f: &F = transmute(f);
    f(&FileChooserWidget::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn location_toggle_popup_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GtkFileChooserWidget, f: glib_ffi::gpointer)
where P: IsA<FileChooserWidget> {
    let f: &F = transmute(f);
    f(&FileChooserWidget::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn places_shortcut_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GtkFileChooserWidget, f: glib_ffi::gpointer)
where P: IsA<FileChooserWidget> {
    let f: &F = transmute(f);
    f(&FileChooserWidget::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn quick_bookmark_trampoline<P, F: Fn(&P, i32) + 'static>(this: *mut ffi::GtkFileChooserWidget, bookmark_index: libc::c_int, f: glib_ffi::gpointer)
where P: IsA<FileChooserWidget> {
    let f: &F = transmute(f);
    f(&FileChooserWidget::from_glib_borrow(this).unsafe_cast(), bookmark_index)
}

unsafe extern "C" fn recent_shortcut_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GtkFileChooserWidget, f: glib_ffi::gpointer)
where P: IsA<FileChooserWidget> {
    let f: &F = transmute(f);
    f(&FileChooserWidget::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn search_shortcut_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GtkFileChooserWidget, f: glib_ffi::gpointer)
where P: IsA<FileChooserWidget> {
    let f: &F = transmute(f);
    f(&FileChooserWidget::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn show_hidden_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GtkFileChooserWidget, f: glib_ffi::gpointer)
where P: IsA<FileChooserWidget> {
    let f: &F = transmute(f);
    f(&FileChooserWidget::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn up_folder_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GtkFileChooserWidget, f: glib_ffi::gpointer)
where P: IsA<FileChooserWidget> {
    let f: &F = transmute(f);
    f(&FileChooserWidget::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_search_mode_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GtkFileChooserWidget, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<FileChooserWidget> {
    let f: &F = transmute(f);
    f(&FileChooserWidget::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_subtitle_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GtkFileChooserWidget, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<FileChooserWidget> {
    let f: &F = transmute(f);
    f(&FileChooserWidget::from_glib_borrow(this).unsafe_cast())
}

impl fmt::Display for FileChooserWidget {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "FileChooserWidget")
    }
}