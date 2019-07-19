// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Cancellable;
use Drive;
use Error;
use File;
use Icon;
use Mount;
use MountMountFlags;
use MountOperation;
use MountUnmountFlags;
use ffi;
#[cfg(feature = "futures")]
use futures_core;
use glib::GString;
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
use std::ptr;

glib_wrapper! {
    pub struct Volume(Interface<ffi::GVolume>);

    match fn {
        get_type => || ffi::g_volume_get_type(),
    }
}

pub const NONE_VOLUME: Option<&Volume> = None;

pub trait VolumeExt: 'static {
    fn can_eject(&self) -> bool;

    fn can_mount(&self) -> bool;

    fn eject_with_operation<'a, 'b, P: IsA<MountOperation> + 'a, Q: Into<Option<&'a P>>, R: IsA<Cancellable> + 'b, S: Into<Option<&'b R>>, T: FnOnce(Result<(), Error>) + Send + 'static>(&self, flags: MountUnmountFlags, mount_operation: Q, cancellable: S, callback: T);

    #[cfg(feature = "futures")]
    fn eject_with_operation_future<'a, P: IsA<MountOperation> + Clone + 'static, Q: Into<Option<&'a P>>>(&self, flags: MountUnmountFlags, mount_operation: Q) -> Box_<futures_core::Future<Item = (Self, ()), Error = (Self, Error)>> where Self: Sized + Clone;

    fn enumerate_identifiers(&self) -> Vec<GString>;

    fn get_activation_root(&self) -> Option<File>;

    fn get_drive(&self) -> Option<Drive>;

    fn get_icon(&self) -> Option<Icon>;

    fn get_identifier(&self, kind: &str) -> Option<GString>;

    fn get_mount(&self) -> Option<Mount>;

    fn get_name(&self) -> Option<GString>;

    fn get_sort_key(&self) -> Option<GString>;

    fn get_symbolic_icon(&self) -> Option<Icon>;

    fn get_uuid(&self) -> Option<GString>;

    fn mount<'a, 'b, P: IsA<MountOperation> + 'a, Q: Into<Option<&'a P>>, R: IsA<Cancellable> + 'b, S: Into<Option<&'b R>>, T: FnOnce(Result<(), Error>) + Send + 'static>(&self, flags: MountMountFlags, mount_operation: Q, cancellable: S, callback: T);

    #[cfg(feature = "futures")]
    fn mount_future<'a, P: IsA<MountOperation> + Clone + 'static, Q: Into<Option<&'a P>>>(&self, flags: MountMountFlags, mount_operation: Q) -> Box_<futures_core::Future<Item = (Self, ()), Error = (Self, Error)>> where Self: Sized + Clone;

    fn should_automount(&self) -> bool;

    fn connect_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_removed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Volume>> VolumeExt for O {
    fn can_eject(&self) -> bool {
        unsafe {
            from_glib(ffi::g_volume_can_eject(self.as_ref().to_glib_none().0))
        }
    }

    fn can_mount(&self) -> bool {
        unsafe {
            from_glib(ffi::g_volume_can_mount(self.as_ref().to_glib_none().0))
        }
    }

    fn eject_with_operation<'a, 'b, P: IsA<MountOperation> + 'a, Q: Into<Option<&'a P>>, R: IsA<Cancellable> + 'b, S: Into<Option<&'b R>>, T: FnOnce(Result<(), Error>) + Send + 'static>(&self, flags: MountUnmountFlags, mount_operation: Q, cancellable: S, callback: T) {
        let mount_operation = mount_operation.into();
        let cancellable = cancellable.into();
        let user_data: Box<T> = Box::new(callback);
        unsafe extern "C" fn eject_with_operation_trampoline<T: FnOnce(Result<(), Error>) + Send + 'static>(_source_object: *mut gobject_ffi::GObject, res: *mut ffi::GAsyncResult, user_data: glib_ffi::gpointer) {
            let mut error = ptr::null_mut();
            let _ = ffi::g_volume_eject_with_operation_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) };
            let callback: Box<T> = Box::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = eject_with_operation_trampoline::<T>;
        unsafe {
            ffi::g_volume_eject_with_operation(self.as_ref().to_glib_none().0, flags.to_glib(), mount_operation.map(|p| p.as_ref()).to_glib_none().0, cancellable.map(|p| p.as_ref()).to_glib_none().0, Some(callback), Box::into_raw(user_data) as *mut _);
        }
    }

    #[cfg(feature = "futures")]
    fn eject_with_operation_future<'a, P: IsA<MountOperation> + Clone + 'static, Q: Into<Option<&'a P>>>(&self, flags: MountUnmountFlags, mount_operation: Q) -> Box_<futures_core::Future<Item = (Self, ()), Error = (Self, Error)>> where Self: Sized + Clone {
        use GioFuture;
        use fragile::Fragile;

        let mount_operation = mount_operation.into();
        let mount_operation = mount_operation.map(ToOwned::to_owned);
        GioFuture::new(self, move |obj, send| {
            let cancellable = Cancellable::new();
            let send = Fragile::new(send);
            let obj_clone = Fragile::new(obj.clone());
            obj.eject_with_operation(
                 flags,
                 mount_operation.as_ref().map(::std::borrow::Borrow::borrow),
                 Some(&cancellable),
                 move |res| {
                     let obj = obj_clone.into_inner();
                     let res = res.map(|v| (obj.clone(), v)).map_err(|v| (obj.clone(), v));
                     let _ = send.into_inner().send(res);
                 },
            );

            cancellable
        })
    }

    fn enumerate_identifiers(&self) -> Vec<GString> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::g_volume_enumerate_identifiers(self.as_ref().to_glib_none().0))
        }
    }

    fn get_activation_root(&self) -> Option<File> {
        unsafe {
            from_glib_full(ffi::g_volume_get_activation_root(self.as_ref().to_glib_none().0))
        }
    }

    fn get_drive(&self) -> Option<Drive> {
        unsafe {
            from_glib_full(ffi::g_volume_get_drive(self.as_ref().to_glib_none().0))
        }
    }

    fn get_icon(&self) -> Option<Icon> {
        unsafe {
            from_glib_full(ffi::g_volume_get_icon(self.as_ref().to_glib_none().0))
        }
    }

    fn get_identifier(&self, kind: &str) -> Option<GString> {
        unsafe {
            from_glib_full(ffi::g_volume_get_identifier(self.as_ref().to_glib_none().0, kind.to_glib_none().0))
        }
    }

    fn get_mount(&self) -> Option<Mount> {
        unsafe {
            from_glib_full(ffi::g_volume_get_mount(self.as_ref().to_glib_none().0))
        }
    }

    fn get_name(&self) -> Option<GString> {
        unsafe {
            from_glib_full(ffi::g_volume_get_name(self.as_ref().to_glib_none().0))
        }
    }

    fn get_sort_key(&self) -> Option<GString> {
        unsafe {
            from_glib_none(ffi::g_volume_get_sort_key(self.as_ref().to_glib_none().0))
        }
    }

    fn get_symbolic_icon(&self) -> Option<Icon> {
        unsafe {
            from_glib_full(ffi::g_volume_get_symbolic_icon(self.as_ref().to_glib_none().0))
        }
    }

    fn get_uuid(&self) -> Option<GString> {
        unsafe {
            from_glib_full(ffi::g_volume_get_uuid(self.as_ref().to_glib_none().0))
        }
    }

    fn mount<'a, 'b, P: IsA<MountOperation> + 'a, Q: Into<Option<&'a P>>, R: IsA<Cancellable> + 'b, S: Into<Option<&'b R>>, T: FnOnce(Result<(), Error>) + Send + 'static>(&self, flags: MountMountFlags, mount_operation: Q, cancellable: S, callback: T) {
        let mount_operation = mount_operation.into();
        let cancellable = cancellable.into();
        let user_data: Box<T> = Box::new(callback);
        unsafe extern "C" fn mount_trampoline<T: FnOnce(Result<(), Error>) + Send + 'static>(_source_object: *mut gobject_ffi::GObject, res: *mut ffi::GAsyncResult, user_data: glib_ffi::gpointer) {
            let mut error = ptr::null_mut();
            let _ = ffi::g_volume_mount_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) };
            let callback: Box<T> = Box::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = mount_trampoline::<T>;
        unsafe {
            ffi::g_volume_mount(self.as_ref().to_glib_none().0, flags.to_glib(), mount_operation.map(|p| p.as_ref()).to_glib_none().0, cancellable.map(|p| p.as_ref()).to_glib_none().0, Some(callback), Box::into_raw(user_data) as *mut _);
        }
    }

    #[cfg(feature = "futures")]
    fn mount_future<'a, P: IsA<MountOperation> + Clone + 'static, Q: Into<Option<&'a P>>>(&self, flags: MountMountFlags, mount_operation: Q) -> Box_<futures_core::Future<Item = (Self, ()), Error = (Self, Error)>> where Self: Sized + Clone {
        use GioFuture;
        use fragile::Fragile;

        let mount_operation = mount_operation.into();
        let mount_operation = mount_operation.map(ToOwned::to_owned);
        GioFuture::new(self, move |obj, send| {
            let cancellable = Cancellable::new();
            let send = Fragile::new(send);
            let obj_clone = Fragile::new(obj.clone());
            obj.mount(
                 flags,
                 mount_operation.as_ref().map(::std::borrow::Borrow::borrow),
                 Some(&cancellable),
                 move |res| {
                     let obj = obj_clone.into_inner();
                     let res = res.map(|v| (obj.clone(), v)).map_err(|v| (obj.clone(), v));
                     let _ = send.into_inner().send(res);
                 },
            );

            cancellable
        })
    }

    fn should_automount(&self) -> bool {
        unsafe {
            from_glib(ffi::g_volume_should_automount(self.as_ref().to_glib_none().0))
        }
    }

    fn connect_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"changed\0".as_ptr() as *const _,
                Some(transmute(changed_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_removed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"removed\0".as_ptr() as *const _,
                Some(transmute(removed_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }
}

unsafe extern "C" fn changed_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GVolume, f: glib_ffi::gpointer)
where P: IsA<Volume> {
    let f: &F = transmute(f);
    f(&Volume::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn removed_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GVolume, f: glib_ffi::gpointer)
where P: IsA<Volume> {
    let f: &F = transmute(f);
    f(&Volume::from_glib_borrow(this).unsafe_cast())
}

impl fmt::Display for Volume {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Volume")
    }
}