// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Actionable;
use Bin;
use Buildable;
use Container;
use ToolButton;
use ToolItem;
use Widget;
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
    pub struct MenuToolButton(Object<ffi::GtkMenuToolButton, ffi::GtkMenuToolButtonClass, MenuToolButtonClass>) @extends ToolButton, ToolItem, Bin, Container, Widget, @implements Buildable, Actionable;

    match fn {
        get_type => || ffi::gtk_menu_tool_button_get_type(),
    }
}

impl MenuToolButton {
    pub fn new<'a, 'b, P: IsA<Widget> + 'a, Q: Into<Option<&'a P>>, R: Into<Option<&'b str>>>(icon_widget: Q, label: R) -> MenuToolButton {
        assert_initialized_main_thread!();
        let icon_widget = icon_widget.into();
        let label = label.into();
        unsafe {
            ToolItem::from_glib_none(ffi::gtk_menu_tool_button_new(icon_widget.map(|p| p.as_ref()).to_glib_none().0, label.to_glib_none().0)).unsafe_cast()
        }
    }
}

pub const NONE_MENU_TOOL_BUTTON: Option<&MenuToolButton> = None;

pub trait MenuToolButtonExt: 'static {
    fn get_menu(&self) -> Option<Widget>;

    fn set_arrow_tooltip_markup(&self, markup: &str);

    fn set_arrow_tooltip_text(&self, text: &str);

    fn set_menu<P: IsA<Widget>>(&self, menu: &P);

    fn connect_show_menu<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_menu_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<MenuToolButton>> MenuToolButtonExt for O {
    fn get_menu(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_menu_tool_button_get_menu(self.as_ref().to_glib_none().0))
        }
    }

    fn set_arrow_tooltip_markup(&self, markup: &str) {
        unsafe {
            ffi::gtk_menu_tool_button_set_arrow_tooltip_markup(self.as_ref().to_glib_none().0, markup.to_glib_none().0);
        }
    }

    fn set_arrow_tooltip_text(&self, text: &str) {
        unsafe {
            ffi::gtk_menu_tool_button_set_arrow_tooltip_text(self.as_ref().to_glib_none().0, text.to_glib_none().0);
        }
    }

    fn set_menu<P: IsA<Widget>>(&self, menu: &P) {
        unsafe {
            ffi::gtk_menu_tool_button_set_menu(self.as_ref().to_glib_none().0, menu.as_ref().to_glib_none().0);
        }
    }

    fn connect_show_menu<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"show-menu\0".as_ptr() as *const _,
                Some(transmute(show_menu_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_menu_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::menu\0".as_ptr() as *const _,
                Some(transmute(notify_menu_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }
}

unsafe extern "C" fn show_menu_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GtkMenuToolButton, f: glib_ffi::gpointer)
where P: IsA<MenuToolButton> {
    let f: &F = transmute(f);
    f(&MenuToolButton::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_menu_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GtkMenuToolButton, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<MenuToolButton> {
    let f: &F = transmute(f);
    f(&MenuToolButton::from_glib_borrow(this).unsafe_cast())
}

impl fmt::Display for MenuToolButton {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MenuToolButton")
    }
}