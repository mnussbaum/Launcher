// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use CellRenderer;
use IconSize;
use ffi;
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
    pub struct CellRendererSpinner(Object<ffi::GtkCellRendererSpinner, ffi::GtkCellRendererSpinnerClass, CellRendererSpinnerClass>) @extends CellRenderer;

    match fn {
        get_type => || ffi::gtk_cell_renderer_spinner_get_type(),
    }
}

impl CellRendererSpinner {
    pub fn new() -> CellRendererSpinner {
        assert_initialized_main_thread!();
        unsafe {
            CellRenderer::from_glib_none(ffi::gtk_cell_renderer_spinner_new()).unsafe_cast()
        }
    }
}

impl Default for CellRendererSpinner {
    fn default() -> Self {
        Self::new()
    }
}

pub const NONE_CELL_RENDERER_SPINNER: Option<&CellRendererSpinner> = None;

pub trait CellRendererSpinnerExt: 'static {
    fn get_property_active(&self) -> bool;

    fn set_property_active(&self, active: bool);

    fn get_property_pulse(&self) -> u32;

    fn set_property_pulse(&self, pulse: u32);

    fn get_property_size(&self) -> IconSize;

    fn set_property_size(&self, size: IconSize);

    fn connect_property_active_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_pulse_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_size_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<CellRendererSpinner>> CellRendererSpinnerExt for O {
    fn get_property_active(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0 as *mut gobject_ffi::GObject, b"active\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn set_property_active(&self, active: bool) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0 as *mut gobject_ffi::GObject, b"active\0".as_ptr() as *const _, Value::from(&active).to_glib_none().0);
        }
    }

    fn get_property_pulse(&self) -> u32 {
        unsafe {
            let mut value = Value::from_type(<u32 as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0 as *mut gobject_ffi::GObject, b"pulse\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn set_property_pulse(&self, pulse: u32) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0 as *mut gobject_ffi::GObject, b"pulse\0".as_ptr() as *const _, Value::from(&pulse).to_glib_none().0);
        }
    }

    fn get_property_size(&self) -> IconSize {
        unsafe {
            let mut value = Value::from_type(<IconSize as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0 as *mut gobject_ffi::GObject, b"size\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn set_property_size(&self, size: IconSize) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0 as *mut gobject_ffi::GObject, b"size\0".as_ptr() as *const _, Value::from(&size).to_glib_none().0);
        }
    }

    fn connect_property_active_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::active\0".as_ptr() as *const _,
                Some(transmute(notify_active_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_pulse_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::pulse\0".as_ptr() as *const _,
                Some(transmute(notify_pulse_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_size_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::size\0".as_ptr() as *const _,
                Some(transmute(notify_size_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }
}

unsafe extern "C" fn notify_active_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GtkCellRendererSpinner, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<CellRendererSpinner> {
    let f: &F = transmute(f);
    f(&CellRendererSpinner::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_pulse_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GtkCellRendererSpinner, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<CellRendererSpinner> {
    let f: &F = transmute(f);
    f(&CellRendererSpinner::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_size_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GtkCellRendererSpinner, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<CellRendererSpinner> {
    let f: &F = transmute(f);
    f(&CellRendererSpinner::from_glib_borrow(this).unsafe_cast())
}

impl fmt::Display for CellRendererSpinner {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CellRendererSpinner")
    }
}