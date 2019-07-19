// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Cancellable;
use Error;
use OutputStream;
use ffi;
use glib::object::IsA;
use glib::translate::*;
use std::fmt;
use std::ptr;

glib_wrapper! {
    pub struct PollableOutputStream(Interface<ffi::GPollableOutputStream>) @requires OutputStream;

    match fn {
        get_type => || ffi::g_pollable_output_stream_get_type(),
    }
}

pub const NONE_POLLABLE_OUTPUT_STREAM: Option<&PollableOutputStream> = None;

pub trait PollableOutputStreamExt: 'static {
    fn can_poll(&self) -> bool;

    fn is_writable(&self) -> bool;

    fn write_nonblocking<'a, P: IsA<Cancellable> + 'a, Q: Into<Option<&'a P>>>(&self, buffer: &[u8], cancellable: Q) -> Result<isize, Error>;
}

impl<O: IsA<PollableOutputStream>> PollableOutputStreamExt for O {
    fn can_poll(&self) -> bool {
        unsafe {
            from_glib(ffi::g_pollable_output_stream_can_poll(self.as_ref().to_glib_none().0))
        }
    }

    fn is_writable(&self) -> bool {
        unsafe {
            from_glib(ffi::g_pollable_output_stream_is_writable(self.as_ref().to_glib_none().0))
        }
    }

    fn write_nonblocking<'a, P: IsA<Cancellable> + 'a, Q: Into<Option<&'a P>>>(&self, buffer: &[u8], cancellable: Q) -> Result<isize, Error> {
        let cancellable = cancellable.into();
        let count = buffer.len() as usize;
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_pollable_output_stream_write_nonblocking(self.as_ref().to_glib_none().0, buffer.to_glib_none().0, count, cancellable.map(|p| p.as_ref()).to_glib_none().0, &mut error);
            if error.is_null() { Ok(ret) } else { Err(from_glib_full(error)) }
        }
    }
}

impl fmt::Display for PollableOutputStream {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "PollableOutputStream")
    }
}