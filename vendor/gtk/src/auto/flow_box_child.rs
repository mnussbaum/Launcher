// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Bin;
use Buildable;
use Container;
use Widget;
use ffi;
use glib;
use glib::object::Cast;
use glib::object::IsA;
use glib::object::ObjectExt;
use glib::signal::SignalHandlerId;
use glib::signal::connect_raw;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib_wrapper! {
    pub struct FlowBoxChild(Object<ffi::GtkFlowBoxChild, ffi::GtkFlowBoxChildClass, FlowBoxChildClass>) @extends Bin, Container, Widget, @implements Buildable;

    match fn {
        get_type => || ffi::gtk_flow_box_child_get_type(),
    }
}

impl FlowBoxChild {
    pub fn new() -> FlowBoxChild {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_flow_box_child_new()).unsafe_cast()
        }
    }
}

impl Default for FlowBoxChild {
    fn default() -> Self {
        Self::new()
    }
}

pub const NONE_FLOW_BOX_CHILD: Option<&FlowBoxChild> = None;

pub trait FlowBoxChildExt: 'static {
    fn changed(&self);

    fn get_index(&self) -> i32;

    fn is_selected(&self) -> bool;

    fn connect_activate<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn emit_activate(&self);
}

impl<O: IsA<FlowBoxChild>> FlowBoxChildExt for O {
    fn changed(&self) {
        unsafe {
            ffi::gtk_flow_box_child_changed(self.as_ref().to_glib_none().0);
        }
    }

    fn get_index(&self) -> i32 {
        unsafe {
            ffi::gtk_flow_box_child_get_index(self.as_ref().to_glib_none().0)
        }
    }

    fn is_selected(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_flow_box_child_is_selected(self.as_ref().to_glib_none().0))
        }
    }

    fn connect_activate<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"activate\0".as_ptr() as *const _,
                Some(transmute(activate_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn emit_activate(&self) {
        let _ = unsafe { glib::Object::from_glib_borrow(self.to_glib_none().0 as *mut gobject_ffi::GObject).emit("activate", &[]).unwrap() };
    }
}

unsafe extern "C" fn activate_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GtkFlowBoxChild, f: glib_ffi::gpointer)
where P: IsA<FlowBoxChild> {
    let f: &F = transmute(f);
    f(&FlowBoxChild::from_glib_borrow(this).unsafe_cast())
}

impl fmt::Display for FlowBoxChild {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "FlowBoxChild")
    }
}
