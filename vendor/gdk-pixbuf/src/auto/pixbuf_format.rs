// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use gdk_pixbuf_sys;
use glib::translate::*;
use glib::GString;

glib_wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct PixbufFormat(Boxed<gdk_pixbuf_sys::GdkPixbufFormat>);

    match fn {
        copy => |ptr| gdk_pixbuf_sys::gdk_pixbuf_format_copy(mut_override(ptr)),
        free => |ptr| gdk_pixbuf_sys::gdk_pixbuf_format_free(ptr),
        get_type => || gdk_pixbuf_sys::gdk_pixbuf_format_get_type(),
    }
}

impl PixbufFormat {
    pub fn get_description(&self) -> Option<GString> {
        unsafe {
            from_glib_full(gdk_pixbuf_sys::gdk_pixbuf_format_get_description(
                mut_override(self.to_glib_none().0),
            ))
        }
    }

    pub fn get_extensions(&self) -> Vec<GString> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(gdk_pixbuf_sys::gdk_pixbuf_format_get_extensions(
                mut_override(self.to_glib_none().0),
            ))
        }
    }

    pub fn get_license(&self) -> Option<GString> {
        unsafe {
            from_glib_full(gdk_pixbuf_sys::gdk_pixbuf_format_get_license(mut_override(
                self.to_glib_none().0,
            )))
        }
    }

    pub fn get_mime_types(&self) -> Vec<GString> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(gdk_pixbuf_sys::gdk_pixbuf_format_get_mime_types(
                mut_override(self.to_glib_none().0),
            ))
        }
    }

    pub fn get_name(&self) -> Option<GString> {
        unsafe {
            from_glib_full(gdk_pixbuf_sys::gdk_pixbuf_format_get_name(mut_override(
                self.to_glib_none().0,
            )))
        }
    }

    pub fn is_disabled(&self) -> bool {
        unsafe {
            from_glib(gdk_pixbuf_sys::gdk_pixbuf_format_is_disabled(mut_override(
                self.to_glib_none().0,
            )))
        }
    }

    #[cfg(any(feature = "v2_36", feature = "dox"))]
    pub fn is_save_option_supported(&self, option_key: &str) -> bool {
        unsafe {
            from_glib(gdk_pixbuf_sys::gdk_pixbuf_format_is_save_option_supported(
                mut_override(self.to_glib_none().0),
                option_key.to_glib_none().0,
            ))
        }
    }

    pub fn is_scalable(&self) -> bool {
        unsafe {
            from_glib(gdk_pixbuf_sys::gdk_pixbuf_format_is_scalable(mut_override(
                self.to_glib_none().0,
            )))
        }
    }

    pub fn is_writable(&self) -> bool {
        unsafe {
            from_glib(gdk_pixbuf_sys::gdk_pixbuf_format_is_writable(mut_override(
                self.to_glib_none().0,
            )))
        }
    }

    pub fn set_disabled(&mut self, disabled: bool) {
        unsafe {
            gdk_pixbuf_sys::gdk_pixbuf_format_set_disabled(
                self.to_glib_none_mut().0,
                disabled.to_glib(),
            );
        }
    }
}
