// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Device;
use DeviceType;
use Display;
use ffi;
use glib::object::ObjectType;
use glib::signal::SignalHandlerId;
use glib::signal::connect_raw;
use glib::translate::*;
use glib_ffi;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib_wrapper! {
    pub struct DeviceManager(Object<ffi::GdkDeviceManager, DeviceManagerClass>);

    match fn {
        get_type => || ffi::gdk_device_manager_get_type(),
    }
}

impl DeviceManager {
    #[cfg_attr(feature = "v3_20", deprecated)]
    pub fn get_client_pointer(&self) -> Option<Device> {
        unsafe {
            from_glib_none(ffi::gdk_device_manager_get_client_pointer(self.to_glib_none().0))
        }
    }

    pub fn get_display(&self) -> Option<Display> {
        unsafe {
            from_glib_none(ffi::gdk_device_manager_get_display(self.to_glib_none().0))
        }
    }

    #[cfg_attr(feature = "v3_20", deprecated)]
    pub fn list_devices(&self, type_: DeviceType) -> Vec<Device> {
        unsafe {
            FromGlibPtrContainer::from_glib_container(ffi::gdk_device_manager_list_devices(self.to_glib_none().0, type_.to_glib()))
        }
    }

    pub fn connect_device_added<F: Fn(&DeviceManager, &Device) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"device-added\0".as_ptr() as *const _,
                Some(transmute(device_added_trampoline::<F> as usize)), Box_::into_raw(f))
        }
    }

    pub fn connect_device_changed<F: Fn(&DeviceManager, &Device) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"device-changed\0".as_ptr() as *const _,
                Some(transmute(device_changed_trampoline::<F> as usize)), Box_::into_raw(f))
        }
    }

    pub fn connect_device_removed<F: Fn(&DeviceManager, &Device) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"device-removed\0".as_ptr() as *const _,
                Some(transmute(device_removed_trampoline::<F> as usize)), Box_::into_raw(f))
        }
    }
}

unsafe extern "C" fn device_added_trampoline<F: Fn(&DeviceManager, &Device) + 'static>(this: *mut ffi::GdkDeviceManager, device: *mut ffi::GdkDevice, f: glib_ffi::gpointer) {
    let f: &F = transmute(f);
    f(&from_glib_borrow(this), &from_glib_borrow(device))
}

unsafe extern "C" fn device_changed_trampoline<F: Fn(&DeviceManager, &Device) + 'static>(this: *mut ffi::GdkDeviceManager, device: *mut ffi::GdkDevice, f: glib_ffi::gpointer) {
    let f: &F = transmute(f);
    f(&from_glib_borrow(this), &from_glib_borrow(device))
}

unsafe extern "C" fn device_removed_trampoline<F: Fn(&DeviceManager, &Device) + 'static>(this: *mut ffi::GdkDeviceManager, device: *mut ffi::GdkDevice, f: glib_ffi::gpointer) {
    let f: &F = transmute(f);
    f(&from_glib_borrow(this), &from_glib_borrow(device))
}

impl fmt::Display for DeviceManager {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DeviceManager")
    }
}