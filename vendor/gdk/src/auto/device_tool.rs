// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use AxisFlags;
use DeviceToolType;
use ffi;
use glib::StaticType;
use glib::Value;
use glib::object::ObjectType;
use glib::translate::*;
use gobject_ffi;
use std::fmt;

glib_wrapper! {
    pub struct DeviceTool(Object<ffi::GdkDeviceTool, DeviceToolClass>);

    match fn {
        get_type => || ffi::gdk_device_tool_get_type(),
    }
}

impl DeviceTool {
    #[cfg(any(feature = "v3_22", feature = "dox"))]
    pub fn get_hardware_id(&self) -> u64 {
        unsafe {
            ffi::gdk_device_tool_get_hardware_id(self.to_glib_none().0)
        }
    }

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    pub fn get_serial(&self) -> u64 {
        unsafe {
            ffi::gdk_device_tool_get_serial(self.to_glib_none().0)
        }
    }

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    pub fn get_tool_type(&self) -> DeviceToolType {
        unsafe {
            from_glib(ffi::gdk_device_tool_get_tool_type(self.to_glib_none().0))
        }
    }

    pub fn get_property_axes(&self) -> AxisFlags {
        unsafe {
            let mut value = Value::from_type(<AxisFlags as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.as_ptr() as *mut gobject_ffi::GObject, b"axes\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    pub fn get_property_hardware_id(&self) -> u64 {
        unsafe {
            let mut value = Value::from_type(<u64 as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.as_ptr() as *mut gobject_ffi::GObject, b"hardware-id\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    pub fn get_property_serial(&self) -> u64 {
        unsafe {
            let mut value = Value::from_type(<u64 as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.as_ptr() as *mut gobject_ffi::GObject, b"serial\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    pub fn get_property_tool_type(&self) -> DeviceToolType {
        unsafe {
            let mut value = Value::from_type(<DeviceToolType as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.as_ptr() as *mut gobject_ffi::GObject, b"tool-type\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }
}

impl fmt::Display for DeviceTool {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DeviceTool")
    }
}