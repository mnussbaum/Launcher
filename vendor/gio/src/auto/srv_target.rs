// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use gio_sys;
use glib::translate::*;
use glib::GString;

glib_wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct SrvTarget(Boxed<gio_sys::GSrvTarget>);

    match fn {
        copy => |ptr| gio_sys::g_srv_target_copy(mut_override(ptr)),
        free => |ptr| gio_sys::g_srv_target_free(ptr),
        get_type => || gio_sys::g_srv_target_get_type(),
    }
}

impl SrvTarget {
    pub fn new(hostname: &str, port: u16, priority: u16, weight: u16) -> SrvTarget {
        unsafe {
            from_glib_full(gio_sys::g_srv_target_new(
                hostname.to_glib_none().0,
                port,
                priority,
                weight,
            ))
        }
    }

    pub fn get_hostname(&mut self) -> Option<GString> {
        unsafe {
            from_glib_none(gio_sys::g_srv_target_get_hostname(
                self.to_glib_none_mut().0,
            ))
        }
    }

    pub fn get_port(&mut self) -> u16 {
        unsafe { gio_sys::g_srv_target_get_port(self.to_glib_none_mut().0) }
    }

    pub fn get_priority(&mut self) -> u16 {
        unsafe { gio_sys::g_srv_target_get_priority(self.to_glib_none_mut().0) }
    }

    pub fn get_weight(&mut self) -> u16 {
        unsafe { gio_sys::g_srv_target_get_weight(self.to_glib_none_mut().0) }
    }

    //pub fn list_sort(targets: /*Unimplemented*/&[&Fundamental: Pointer]) -> /*Unimplemented*/Vec<Fundamental: Pointer> {
    //    unsafe { TODO: call gio_sys:g_srv_target_list_sort() }
    //}
}
