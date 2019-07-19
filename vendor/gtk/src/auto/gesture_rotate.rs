// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use EventController;
use Gesture;
use Widget;
use ffi;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect_raw;
use glib::translate::*;
use glib_ffi;
use libc;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib_wrapper! {
    pub struct GestureRotate(Object<ffi::GtkGestureRotate, ffi::GtkGestureRotateClass, GestureRotateClass>) @extends Gesture, EventController;

    match fn {
        get_type => || ffi::gtk_gesture_rotate_get_type(),
    }
}

impl GestureRotate {
    pub fn new<P: IsA<Widget>>(widget: &P) -> GestureRotate {
        skip_assert_initialized!();
        unsafe {
            Gesture::from_glib_full(ffi::gtk_gesture_rotate_new(widget.as_ref().to_glib_none().0)).unsafe_cast()
        }
    }
}

pub const NONE_GESTURE_ROTATE: Option<&GestureRotate> = None;

pub trait GestureRotateExt: 'static {
    fn get_angle_delta(&self) -> f64;

    fn connect_angle_changed<F: Fn(&Self, f64, f64) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<GestureRotate>> GestureRotateExt for O {
    fn get_angle_delta(&self) -> f64 {
        unsafe {
            ffi::gtk_gesture_rotate_get_angle_delta(self.as_ref().to_glib_none().0)
        }
    }

    fn connect_angle_changed<F: Fn(&Self, f64, f64) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"angle-changed\0".as_ptr() as *const _,
                Some(transmute(angle_changed_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }
}

unsafe extern "C" fn angle_changed_trampoline<P, F: Fn(&P, f64, f64) + 'static>(this: *mut ffi::GtkGestureRotate, angle: libc::c_double, angle_delta: libc::c_double, f: glib_ffi::gpointer)
where P: IsA<GestureRotate> {
    let f: &F = transmute(f);
    f(&GestureRotate::from_glib_borrow(this).unsafe_cast(), angle, angle_delta)
}

impl fmt::Display for GestureRotate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "GestureRotate")
    }
}