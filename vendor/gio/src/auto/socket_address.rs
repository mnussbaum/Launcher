// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use SocketConnectable;
use SocketFamily;
use ffi;
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
    pub struct SocketAddress(Object<ffi::GSocketAddress, ffi::GSocketAddressClass, SocketAddressClass>) @implements SocketConnectable;

    match fn {
        get_type => || ffi::g_socket_address_get_type(),
    }
}

impl SocketAddress {
    //pub fn new_from_native(native: /*Unimplemented*/Fundamental: Pointer, len: usize) -> SocketAddress {
    //    unsafe { TODO: call ffi::g_socket_address_new_from_native() }
    //}
}

unsafe impl Send for SocketAddress {}
unsafe impl Sync for SocketAddress {}

pub const NONE_SOCKET_ADDRESS: Option<&SocketAddress> = None;

pub trait SocketAddressExt: 'static {
    fn get_family(&self) -> SocketFamily;

    fn get_native_size(&self) -> isize;

    //fn to_native(&self, dest: /*Unimplemented*/Option<Fundamental: Pointer>, destlen: usize) -> Result<(), Error>;

    fn connect_property_family_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<SocketAddress>> SocketAddressExt for O {
    fn get_family(&self) -> SocketFamily {
        unsafe {
            from_glib(ffi::g_socket_address_get_family(self.as_ref().to_glib_none().0))
        }
    }

    fn get_native_size(&self) -> isize {
        unsafe {
            ffi::g_socket_address_get_native_size(self.as_ref().to_glib_none().0)
        }
    }

    //fn to_native(&self, dest: /*Unimplemented*/Option<Fundamental: Pointer>, destlen: usize) -> Result<(), Error> {
    //    unsafe { TODO: call ffi::g_socket_address_to_native() }
    //}

    fn connect_property_family_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::family\0".as_ptr() as *const _,
                Some(transmute(notify_family_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }
}

unsafe extern "C" fn notify_family_trampoline<P, F: Fn(&P) + Send + Sync + 'static>(this: *mut ffi::GSocketAddress, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<SocketAddress> {
    let f: &F = transmute(f);
    f(&SocketAddress::from_glib_borrow(this).unsafe_cast())
}

impl fmt::Display for SocketAddress {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SocketAddress")
    }
}
