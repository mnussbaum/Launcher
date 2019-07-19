// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Display;
use Rectangle;
#[cfg(any(feature = "v3_22", feature = "dox"))]
use SubpixelLayout;
use ffi;
#[cfg(any(feature = "v3_22", feature = "dox"))]
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
    pub struct Monitor(Object<ffi::GdkMonitor, ffi::GdkMonitorClass, MonitorClass>);

    match fn {
        get_type => || ffi::gdk_monitor_get_type(),
    }
}

pub const NONE_MONITOR: Option<&Monitor> = None;

pub trait MonitorExt: 'static {
    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn get_display(&self) -> Option<Display>;

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn get_geometry(&self) -> Rectangle;

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn get_height_mm(&self) -> i32;

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn get_manufacturer(&self) -> Option<GString>;

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn get_model(&self) -> Option<GString>;

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn get_refresh_rate(&self) -> i32;

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn get_scale_factor(&self) -> i32;

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn get_subpixel_layout(&self) -> SubpixelLayout;

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn get_width_mm(&self) -> i32;

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn get_workarea(&self) -> Rectangle;

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn is_primary(&self) -> bool;

    fn get_property_display(&self) -> Option<Display>;

    fn get_property_geometry(&self) -> Option<Rectangle>;

    fn get_property_height_mm(&self) -> i32;

    fn get_property_refresh_rate(&self) -> i32;

    fn get_property_scale_factor(&self) -> i32;

    fn get_property_width_mm(&self) -> i32;

    fn get_property_workarea(&self) -> Option<Rectangle>;

    fn connect_invalidate<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_geometry_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_height_mm_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn connect_property_manufacturer_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn connect_property_model_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_refresh_rate_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_scale_factor_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn connect_property_subpixel_layout_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_width_mm_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_workarea_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Monitor>> MonitorExt for O {
    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn get_display(&self) -> Option<Display> {
        unsafe {
            from_glib_none(ffi::gdk_monitor_get_display(self.as_ref().to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn get_geometry(&self) -> Rectangle {
        unsafe {
            let mut geometry = Rectangle::uninitialized();
            ffi::gdk_monitor_get_geometry(self.as_ref().to_glib_none().0, geometry.to_glib_none_mut().0);
            geometry
        }
    }

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn get_height_mm(&self) -> i32 {
        unsafe {
            ffi::gdk_monitor_get_height_mm(self.as_ref().to_glib_none().0)
        }
    }

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn get_manufacturer(&self) -> Option<GString> {
        unsafe {
            from_glib_none(ffi::gdk_monitor_get_manufacturer(self.as_ref().to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn get_model(&self) -> Option<GString> {
        unsafe {
            from_glib_none(ffi::gdk_monitor_get_model(self.as_ref().to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn get_refresh_rate(&self) -> i32 {
        unsafe {
            ffi::gdk_monitor_get_refresh_rate(self.as_ref().to_glib_none().0)
        }
    }

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn get_scale_factor(&self) -> i32 {
        unsafe {
            ffi::gdk_monitor_get_scale_factor(self.as_ref().to_glib_none().0)
        }
    }

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn get_subpixel_layout(&self) -> SubpixelLayout {
        unsafe {
            from_glib(ffi::gdk_monitor_get_subpixel_layout(self.as_ref().to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn get_width_mm(&self) -> i32 {
        unsafe {
            ffi::gdk_monitor_get_width_mm(self.as_ref().to_glib_none().0)
        }
    }

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn get_workarea(&self) -> Rectangle {
        unsafe {
            let mut workarea = Rectangle::uninitialized();
            ffi::gdk_monitor_get_workarea(self.as_ref().to_glib_none().0, workarea.to_glib_none_mut().0);
            workarea
        }
    }

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn is_primary(&self) -> bool {
        unsafe {
            from_glib(ffi::gdk_monitor_is_primary(self.as_ref().to_glib_none().0))
        }
    }

    fn get_property_display(&self) -> Option<Display> {
        unsafe {
            let mut value = Value::from_type(<Display as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0 as *mut gobject_ffi::GObject, b"display\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get()
        }
    }

    fn get_property_geometry(&self) -> Option<Rectangle> {
        unsafe {
            let mut value = Value::from_type(<Rectangle as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0 as *mut gobject_ffi::GObject, b"geometry\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get()
        }
    }

    fn get_property_height_mm(&self) -> i32 {
        unsafe {
            let mut value = Value::from_type(<i32 as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0 as *mut gobject_ffi::GObject, b"height-mm\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn get_property_refresh_rate(&self) -> i32 {
        unsafe {
            let mut value = Value::from_type(<i32 as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0 as *mut gobject_ffi::GObject, b"refresh-rate\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn get_property_scale_factor(&self) -> i32 {
        unsafe {
            let mut value = Value::from_type(<i32 as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0 as *mut gobject_ffi::GObject, b"scale-factor\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn get_property_width_mm(&self) -> i32 {
        unsafe {
            let mut value = Value::from_type(<i32 as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0 as *mut gobject_ffi::GObject, b"width-mm\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn get_property_workarea(&self) -> Option<Rectangle> {
        unsafe {
            let mut value = Value::from_type(<Rectangle as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0 as *mut gobject_ffi::GObject, b"workarea\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get()
        }
    }

    fn connect_invalidate<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"invalidate\0".as_ptr() as *const _,
                Some(transmute(invalidate_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_geometry_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::geometry\0".as_ptr() as *const _,
                Some(transmute(notify_geometry_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_height_mm_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::height-mm\0".as_ptr() as *const _,
                Some(transmute(notify_height_mm_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn connect_property_manufacturer_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::manufacturer\0".as_ptr() as *const _,
                Some(transmute(notify_manufacturer_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn connect_property_model_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::model\0".as_ptr() as *const _,
                Some(transmute(notify_model_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_refresh_rate_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::refresh-rate\0".as_ptr() as *const _,
                Some(transmute(notify_refresh_rate_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_scale_factor_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::scale-factor\0".as_ptr() as *const _,
                Some(transmute(notify_scale_factor_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn connect_property_subpixel_layout_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::subpixel-layout\0".as_ptr() as *const _,
                Some(transmute(notify_subpixel_layout_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_width_mm_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::width-mm\0".as_ptr() as *const _,
                Some(transmute(notify_width_mm_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_workarea_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::workarea\0".as_ptr() as *const _,
                Some(transmute(notify_workarea_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }
}

unsafe extern "C" fn invalidate_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GdkMonitor, f: glib_ffi::gpointer)
where P: IsA<Monitor> {
    let f: &F = transmute(f);
    f(&Monitor::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_geometry_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GdkMonitor, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Monitor> {
    let f: &F = transmute(f);
    f(&Monitor::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_height_mm_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GdkMonitor, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Monitor> {
    let f: &F = transmute(f);
    f(&Monitor::from_glib_borrow(this).unsafe_cast())
}

#[cfg(any(feature = "v3_22", feature = "dox"))]
unsafe extern "C" fn notify_manufacturer_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GdkMonitor, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Monitor> {
    let f: &F = transmute(f);
    f(&Monitor::from_glib_borrow(this).unsafe_cast())
}

#[cfg(any(feature = "v3_22", feature = "dox"))]
unsafe extern "C" fn notify_model_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GdkMonitor, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Monitor> {
    let f: &F = transmute(f);
    f(&Monitor::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_refresh_rate_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GdkMonitor, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Monitor> {
    let f: &F = transmute(f);
    f(&Monitor::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_scale_factor_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GdkMonitor, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Monitor> {
    let f: &F = transmute(f);
    f(&Monitor::from_glib_borrow(this).unsafe_cast())
}

#[cfg(any(feature = "v3_22", feature = "dox"))]
unsafe extern "C" fn notify_subpixel_layout_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GdkMonitor, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Monitor> {
    let f: &F = transmute(f);
    f(&Monitor::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_width_mm_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GdkMonitor, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Monitor> {
    let f: &F = transmute(f);
    f(&Monitor::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_workarea_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GdkMonitor, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Monitor> {
    let f: &F = transmute(f);
    f(&Monitor::from_glib_borrow(this).unsafe_cast())
}

impl fmt::Display for Monitor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Monitor")
    }
}