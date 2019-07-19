// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Icon;
use ffi;
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
    pub struct ThemedIcon(Object<ffi::GThemedIcon, ffi::GThemedIconClass, ThemedIconClass>) @implements Icon;

    match fn {
        get_type => || ffi::g_themed_icon_get_type(),
    }
}

impl ThemedIcon {
    pub fn new(iconname: &str) -> ThemedIcon {
        unsafe {
            from_glib_full(ffi::g_themed_icon_new(iconname.to_glib_none().0))
        }
    }

    pub fn new_from_names(iconnames: &[&str]) -> ThemedIcon {
        let len = iconnames.len() as i32;
        unsafe {
            from_glib_full(ffi::g_themed_icon_new_from_names(iconnames.to_glib_none().0, len))
        }
    }

    pub fn new_with_default_fallbacks(iconname: &str) -> ThemedIcon {
        unsafe {
            from_glib_full(ffi::g_themed_icon_new_with_default_fallbacks(iconname.to_glib_none().0))
        }
    }
}

pub const NONE_THEMED_ICON: Option<&ThemedIcon> = None;

pub trait ThemedIconExt: 'static {
    fn append_name(&self, iconname: &str);

    fn get_names(&self) -> Vec<GString>;

    fn prepend_name(&self, iconname: &str);

    fn get_property_use_default_fallbacks(&self) -> bool;

    fn connect_property_names_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<ThemedIcon>> ThemedIconExt for O {
    fn append_name(&self, iconname: &str) {
        unsafe {
            ffi::g_themed_icon_append_name(self.as_ref().to_glib_none().0, iconname.to_glib_none().0);
        }
    }

    fn get_names(&self) -> Vec<GString> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(ffi::g_themed_icon_get_names(self.as_ref().to_glib_none().0))
        }
    }

    fn prepend_name(&self, iconname: &str) {
        unsafe {
            ffi::g_themed_icon_prepend_name(self.as_ref().to_glib_none().0, iconname.to_glib_none().0);
        }
    }

    fn get_property_use_default_fallbacks(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0 as *mut gobject_ffi::GObject, b"use-default-fallbacks\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn connect_property_names_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::names\0".as_ptr() as *const _,
                Some(transmute(notify_names_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }
}

unsafe extern "C" fn notify_names_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GThemedIcon, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<ThemedIcon> {
    let f: &F = transmute(f);
    f(&ThemedIcon::from_glib_borrow(this).unsafe_cast())
}

impl fmt::Display for ThemedIcon {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ThemedIcon")
    }
}