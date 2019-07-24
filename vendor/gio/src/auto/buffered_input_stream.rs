// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

#[cfg(feature = "futures")]
use futures::future;
use gio_sys;
use glib;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
use glib_sys;
use gobject_sys;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem;
use std::mem::transmute;
use std::ptr;
use Cancellable;
use Error;
use FilterInputStream;
use InputStream;
use Seekable;

glib_wrapper! {
    pub struct BufferedInputStream(Object<gio_sys::GBufferedInputStream, gio_sys::GBufferedInputStreamClass, BufferedInputStreamClass>) @extends FilterInputStream, InputStream, @implements Seekable;

    match fn {
        get_type => || gio_sys::g_buffered_input_stream_get_type(),
    }
}

impl BufferedInputStream {
    pub fn new<P: IsA<InputStream>>(base_stream: &P) -> BufferedInputStream {
        unsafe {
            InputStream::from_glib_full(gio_sys::g_buffered_input_stream_new(
                base_stream.as_ref().to_glib_none().0,
            ))
            .unsafe_cast()
        }
    }

    pub fn new_sized<P: IsA<InputStream>>(base_stream: &P, size: usize) -> BufferedInputStream {
        unsafe {
            InputStream::from_glib_full(gio_sys::g_buffered_input_stream_new_sized(
                base_stream.as_ref().to_glib_none().0,
                size,
            ))
            .unsafe_cast()
        }
    }
}

pub struct BufferedInputStreamBuilder {
    buffer_size: Option<u32>,
    base_stream: Option<InputStream>,
    close_base_stream: Option<bool>,
}

impl BufferedInputStreamBuilder {
    pub fn new() -> Self {
        Self {
            buffer_size: None,
            base_stream: None,
            close_base_stream: None,
        }
    }

    pub fn build(self) -> BufferedInputStream {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref buffer_size) = self.buffer_size {
            properties.push(("buffer-size", buffer_size));
        }
        if let Some(ref base_stream) = self.base_stream {
            properties.push(("base-stream", base_stream));
        }
        if let Some(ref close_base_stream) = self.close_base_stream {
            properties.push(("close-base-stream", close_base_stream));
        }
        glib::Object::new(BufferedInputStream::static_type(), &properties)
            .expect("object new")
            .downcast()
            .expect("downcast")
    }

    pub fn buffer_size(mut self, buffer_size: u32) -> Self {
        self.buffer_size = Some(buffer_size);
        self
    }

    pub fn base_stream(mut self, base_stream: &InputStream) -> Self {
        self.base_stream = Some(base_stream.clone());
        self
    }

    pub fn close_base_stream(mut self, close_base_stream: bool) -> Self {
        self.close_base_stream = Some(close_base_stream);
        self
    }
}

pub const NONE_BUFFERED_INPUT_STREAM: Option<&BufferedInputStream> = None;

pub trait BufferedInputStreamExt: 'static {
    fn fill<P: IsA<Cancellable>>(
        &self,
        count: isize,
        cancellable: Option<&P>,
    ) -> Result<isize, Error>;

    fn fill_async<P: IsA<Cancellable>, Q: FnOnce(Result<isize, Error>) + Send + 'static>(
        &self,
        count: isize,
        io_priority: glib::Priority,
        cancellable: Option<&P>,
        callback: Q,
    );

    #[cfg(feature = "futures")]
    fn fill_async_future(
        &self,
        count: isize,
        io_priority: glib::Priority,
    ) -> Box_<dyn future::Future<Output = Result<isize, Error>> + std::marker::Unpin>;

    fn get_available(&self) -> usize;

    fn get_buffer_size(&self) -> usize;

    fn peek_buffer(&self) -> Vec<u8>;

    fn read_byte<P: IsA<Cancellable>>(&self, cancellable: Option<&P>) -> Result<i32, Error>;

    fn set_buffer_size(&self, size: usize);

    fn connect_property_buffer_size_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<BufferedInputStream>> BufferedInputStreamExt for O {
    fn fill<P: IsA<Cancellable>>(
        &self,
        count: isize,
        cancellable: Option<&P>,
    ) -> Result<isize, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = gio_sys::g_buffered_input_stream_fill(
                self.as_ref().to_glib_none().0,
                count,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(ret)
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn fill_async<P: IsA<Cancellable>, Q: FnOnce(Result<isize, Error>) + Send + 'static>(
        &self,
        count: isize,
        io_priority: glib::Priority,
        cancellable: Option<&P>,
        callback: Q,
    ) {
        let user_data: Box<Q> = Box::new(callback);
        unsafe extern "C" fn fill_async_trampoline<
            Q: FnOnce(Result<isize, Error>) + Send + 'static,
        >(
            _source_object: *mut gobject_sys::GObject,
            res: *mut gio_sys::GAsyncResult,
            user_data: glib_sys::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let ret = gio_sys::g_buffered_input_stream_fill_finish(
                _source_object as *mut _,
                res,
                &mut error,
            );
            let result = if error.is_null() {
                Ok(ret)
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box<Q> = Box::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = fill_async_trampoline::<Q>;
        unsafe {
            gio_sys::g_buffered_input_stream_fill_async(
                self.as_ref().to_glib_none().0,
                count,
                io_priority.to_glib(),
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box::into_raw(user_data) as *mut _,
            );
        }
    }

    #[cfg(feature = "futures")]
    fn fill_async_future(
        &self,
        count: isize,
        io_priority: glib::Priority,
    ) -> Box_<dyn future::Future<Output = Result<isize, Error>> + std::marker::Unpin> {
        use fragile::Fragile;
        use GioFuture;

        GioFuture::new(self, move |obj, send| {
            let cancellable = Cancellable::new();
            let send = Fragile::new(send);
            obj.fill_async(count, io_priority, Some(&cancellable), move |res| {
                let _ = send.into_inner().send(res);
            });

            cancellable
        })
    }

    fn get_available(&self) -> usize {
        unsafe { gio_sys::g_buffered_input_stream_get_available(self.as_ref().to_glib_none().0) }
    }

    fn get_buffer_size(&self) -> usize {
        unsafe { gio_sys::g_buffered_input_stream_get_buffer_size(self.as_ref().to_glib_none().0) }
    }

    fn peek_buffer(&self) -> Vec<u8> {
        unsafe {
            let mut count = mem::uninitialized();
            let ret = FromGlibContainer::from_glib_none_num(
                gio_sys::g_buffered_input_stream_peek_buffer(
                    self.as_ref().to_glib_none().0,
                    &mut count,
                ),
                count as usize,
            );
            ret
        }
    }

    fn read_byte<P: IsA<Cancellable>>(&self, cancellable: Option<&P>) -> Result<i32, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = gio_sys::g_buffered_input_stream_read_byte(
                self.as_ref().to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(ret)
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn set_buffer_size(&self, size: usize) {
        unsafe {
            gio_sys::g_buffered_input_stream_set_buffer_size(self.as_ref().to_glib_none().0, size);
        }
    }

    fn connect_property_buffer_size_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_buffer_size_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gio_sys::GBufferedInputStream,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<BufferedInputStream>,
        {
            let f: &F = &*(f as *const F);
            f(&BufferedInputStream::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::buffer-size\0".as_ptr() as *const _,
                Some(transmute(notify_buffer_size_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for BufferedInputStream {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "BufferedInputStream")
    }
}
