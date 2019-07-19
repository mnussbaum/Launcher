// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Bin;
use Buildable;
use Container;
use Popover;
use Widget;
use ffi;
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
    pub struct PopoverMenu(Object<ffi::GtkPopoverMenu, ffi::GtkPopoverMenuClass, PopoverMenuClass>) @extends Popover, Bin, Container, Widget, @implements Buildable;

    match fn {
        get_type => || ffi::gtk_popover_menu_get_type(),
    }
}

impl PopoverMenu {
    #[cfg(any(feature = "v3_16", feature = "dox"))]
    pub fn new() -> PopoverMenu {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_popover_menu_new()).unsafe_cast()
        }
    }
}

#[cfg(any(feature = "v3_16", feature = "dox"))]
impl Default for PopoverMenu {
    fn default() -> Self {
        Self::new()
    }
}

pub const NONE_POPOVER_MENU: Option<&PopoverMenu> = None;

pub trait PopoverMenuExt: 'static {
    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn open_submenu(&self, name: &str);

    fn get_property_visible_submenu(&self) -> Option<GString>;

    fn set_property_visible_submenu<'a, P: Into<Option<&'a str>>>(&self, visible_submenu: P);

    fn get_child_position<T: IsA<Widget>>(&self, item: &T) -> i32;

    fn set_child_position<T: IsA<Widget>>(&self, item: &T, position: i32);

    fn get_child_submenu<T: IsA<Widget>>(&self, item: &T) -> Option<GString>;

    fn set_child_submenu<'a, P: Into<Option<&'a str>>, T: IsA<Widget>>(&self, item: &T, submenu: P);

    fn connect_property_visible_submenu_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<PopoverMenu>> PopoverMenuExt for O {
    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn open_submenu(&self, name: &str) {
        unsafe {
            ffi::gtk_popover_menu_open_submenu(self.as_ref().to_glib_none().0, name.to_glib_none().0);
        }
    }

    fn get_property_visible_submenu(&self) -> Option<GString> {
        unsafe {
            let mut value = Value::from_type(<GString as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0 as *mut gobject_ffi::GObject, b"visible-submenu\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get()
        }
    }

    fn set_property_visible_submenu<'a, P: Into<Option<&'a str>>>(&self, visible_submenu: P) {
        let visible_submenu = visible_submenu.into();
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0 as *mut gobject_ffi::GObject, b"visible-submenu\0".as_ptr() as *const _, Value::from(visible_submenu).to_glib_none().0);
        }
    }

    fn get_child_position<T: IsA<Widget>>(&self, item: &T) -> i32 {
        unsafe {
            let mut value = Value::from_type(<i32 as StaticType>::static_type());
            ffi::gtk_container_child_get_property(self.to_glib_none().0 as *mut ffi::GtkContainer, item.to_glib_none().0 as *mut _, b"position\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn set_child_position<T: IsA<Widget>>(&self, item: &T, position: i32) {
        unsafe {
            ffi::gtk_container_child_set_property(self.to_glib_none().0 as *mut ffi::GtkContainer, item.to_glib_none().0 as *mut _, b"position\0".as_ptr() as *const _, Value::from(&position).to_glib_none().0);
        }
    }

    fn get_child_submenu<T: IsA<Widget>>(&self, item: &T) -> Option<GString> {
        unsafe {
            let mut value = Value::from_type(<GString as StaticType>::static_type());
            ffi::gtk_container_child_get_property(self.to_glib_none().0 as *mut ffi::GtkContainer, item.to_glib_none().0 as *mut _, b"submenu\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get()
        }
    }

    fn set_child_submenu<'a, P: Into<Option<&'a str>>, T: IsA<Widget>>(&self, item: &T, submenu: P) {
        let submenu = submenu.into();
        unsafe {
            ffi::gtk_container_child_set_property(self.to_glib_none().0 as *mut ffi::GtkContainer, item.to_glib_none().0 as *mut _, b"submenu\0".as_ptr() as *const _, Value::from(submenu).to_glib_none().0);
        }
    }

    fn connect_property_visible_submenu_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::visible-submenu\0".as_ptr() as *const _,
                Some(transmute(notify_visible_submenu_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }
}

unsafe extern "C" fn notify_visible_submenu_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GtkPopoverMenu, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<PopoverMenu> {
    let f: &F = transmute(f);
    f(&PopoverMenu::from_glib_borrow(this).unsafe_cast())
}

impl fmt::Display for PopoverMenu {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "PopoverMenu")
    }
}