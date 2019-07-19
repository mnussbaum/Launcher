// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use AssistantPageType;
use Bin;
use Buildable;
use Container;
use Widget;
use Window;
use ffi;
use glib;
use glib::GString;
use glib::StaticType;
use glib::Value;
use glib::object::Cast;
use glib::object::IsA;
use glib::object::ObjectExt;
use glib::signal::SignalHandlerId;
use glib::signal::connect_raw;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib_wrapper! {
    pub struct Assistant(Object<ffi::GtkAssistant, ffi::GtkAssistantClass, AssistantClass>) @extends Window, Bin, Container, Widget, @implements Buildable;

    match fn {
        get_type => || ffi::gtk_assistant_get_type(),
    }
}

impl Assistant {
    pub fn new() -> Assistant {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_assistant_new()).unsafe_cast()
        }
    }
}

impl Default for Assistant {
    fn default() -> Self {
        Self::new()
    }
}

pub const NONE_ASSISTANT: Option<&Assistant> = None;

pub trait AssistantExt: 'static {
    fn add_action_widget<P: IsA<Widget>>(&self, child: &P);

    fn append_page<P: IsA<Widget>>(&self, page: &P) -> i32;

    fn commit(&self);

    fn get_current_page(&self) -> i32;

    fn get_n_pages(&self) -> i32;

    fn get_nth_page(&self, page_num: i32) -> Option<Widget>;

    fn get_page_complete<P: IsA<Widget>>(&self, page: &P) -> bool;

    #[cfg(any(feature = "v3_18", feature = "dox"))]
    fn get_page_has_padding<P: IsA<Widget>>(&self, page: &P) -> bool;

    fn get_page_title<P: IsA<Widget>>(&self, page: &P) -> Option<GString>;

    fn get_page_type<P: IsA<Widget>>(&self, page: &P) -> AssistantPageType;

    fn insert_page<P: IsA<Widget>>(&self, page: &P, position: i32) -> i32;

    fn next_page(&self);

    fn prepend_page<P: IsA<Widget>>(&self, page: &P) -> i32;

    fn previous_page(&self);

    fn remove_action_widget<P: IsA<Widget>>(&self, child: &P);

    fn remove_page(&self, page_num: i32);

    fn set_current_page(&self, page_num: i32);

    fn set_forward_page_func(&self, page_func: Option<Box<dyn Fn(i32) -> i32 + 'static>>);

    fn set_page_complete<P: IsA<Widget>>(&self, page: &P, complete: bool);

    #[cfg(any(feature = "v3_18", feature = "dox"))]
    fn set_page_has_padding<P: IsA<Widget>>(&self, page: &P, has_padding: bool);

    fn set_page_title<P: IsA<Widget>>(&self, page: &P, title: &str);

    fn set_page_type<P: IsA<Widget>>(&self, page: &P, type_: AssistantPageType);

    fn update_buttons_state(&self);

    fn get_property_use_header_bar(&self) -> i32;

    fn get_child_complete<T: IsA<Widget>>(&self, item: &T) -> bool;

    fn set_child_complete<T: IsA<Widget>>(&self, item: &T, complete: bool);

    fn get_child_has_padding<T: IsA<Widget>>(&self, item: &T) -> bool;

    fn set_child_has_padding<T: IsA<Widget>>(&self, item: &T, has_padding: bool);

    fn get_child_page_type<T: IsA<Widget>>(&self, item: &T) -> AssistantPageType;

    fn set_child_page_type<T: IsA<Widget>>(&self, item: &T, page_type: AssistantPageType);

    fn get_child_title<T: IsA<Widget>>(&self, item: &T) -> Option<GString>;

    fn set_child_title<'a, P: Into<Option<&'a str>>, T: IsA<Widget>>(&self, item: &T, title: P);

    fn connect_apply<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_cancel<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_close<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_escape<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn emit_escape(&self);

    fn connect_prepare<F: Fn(&Self, &Widget) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Assistant>> AssistantExt for O {
    fn add_action_widget<P: IsA<Widget>>(&self, child: &P) {
        unsafe {
            ffi::gtk_assistant_add_action_widget(self.as_ref().to_glib_none().0, child.as_ref().to_glib_none().0);
        }
    }

    fn append_page<P: IsA<Widget>>(&self, page: &P) -> i32 {
        unsafe {
            ffi::gtk_assistant_append_page(self.as_ref().to_glib_none().0, page.as_ref().to_glib_none().0)
        }
    }

    fn commit(&self) {
        unsafe {
            ffi::gtk_assistant_commit(self.as_ref().to_glib_none().0);
        }
    }

    fn get_current_page(&self) -> i32 {
        unsafe {
            ffi::gtk_assistant_get_current_page(self.as_ref().to_glib_none().0)
        }
    }

    fn get_n_pages(&self) -> i32 {
        unsafe {
            ffi::gtk_assistant_get_n_pages(self.as_ref().to_glib_none().0)
        }
    }

    fn get_nth_page(&self, page_num: i32) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_assistant_get_nth_page(self.as_ref().to_glib_none().0, page_num))
        }
    }

    fn get_page_complete<P: IsA<Widget>>(&self, page: &P) -> bool {
        unsafe {
            from_glib(ffi::gtk_assistant_get_page_complete(self.as_ref().to_glib_none().0, page.as_ref().to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v3_18", feature = "dox"))]
    fn get_page_has_padding<P: IsA<Widget>>(&self, page: &P) -> bool {
        unsafe {
            from_glib(ffi::gtk_assistant_get_page_has_padding(self.as_ref().to_glib_none().0, page.as_ref().to_glib_none().0))
        }
    }

    fn get_page_title<P: IsA<Widget>>(&self, page: &P) -> Option<GString> {
        unsafe {
            from_glib_none(ffi::gtk_assistant_get_page_title(self.as_ref().to_glib_none().0, page.as_ref().to_glib_none().0))
        }
    }

    fn get_page_type<P: IsA<Widget>>(&self, page: &P) -> AssistantPageType {
        unsafe {
            from_glib(ffi::gtk_assistant_get_page_type(self.as_ref().to_glib_none().0, page.as_ref().to_glib_none().0))
        }
    }

    fn insert_page<P: IsA<Widget>>(&self, page: &P, position: i32) -> i32 {
        unsafe {
            ffi::gtk_assistant_insert_page(self.as_ref().to_glib_none().0, page.as_ref().to_glib_none().0, position)
        }
    }

    fn next_page(&self) {
        unsafe {
            ffi::gtk_assistant_next_page(self.as_ref().to_glib_none().0);
        }
    }

    fn prepend_page<P: IsA<Widget>>(&self, page: &P) -> i32 {
        unsafe {
            ffi::gtk_assistant_prepend_page(self.as_ref().to_glib_none().0, page.as_ref().to_glib_none().0)
        }
    }

    fn previous_page(&self) {
        unsafe {
            ffi::gtk_assistant_previous_page(self.as_ref().to_glib_none().0);
        }
    }

    fn remove_action_widget<P: IsA<Widget>>(&self, child: &P) {
        unsafe {
            ffi::gtk_assistant_remove_action_widget(self.as_ref().to_glib_none().0, child.as_ref().to_glib_none().0);
        }
    }

    fn remove_page(&self, page_num: i32) {
        unsafe {
            ffi::gtk_assistant_remove_page(self.as_ref().to_glib_none().0, page_num);
        }
    }

    fn set_current_page(&self, page_num: i32) {
        unsafe {
            ffi::gtk_assistant_set_current_page(self.as_ref().to_glib_none().0, page_num);
        }
    }

    fn set_forward_page_func(&self, page_func: Option<Box<dyn Fn(i32) -> i32 + 'static>>) {
        let page_func_data: Box_<Option<Box<dyn Fn(i32) -> i32 + 'static>>> = Box::new(page_func);
        unsafe extern "C" fn page_func_func(current_page: libc::c_int, data: glib_ffi::gpointer) -> libc::c_int {
            let callback: &Option<Box<dyn Fn(i32) -> i32 + 'static>> = &*(data as *mut _);
            let res = if let Some(ref callback) = *callback {
                callback(current_page)
            } else {
                panic!("cannot get closure...")
            };
            res
        }
        let page_func = if page_func_data.is_some() { Some(page_func_func as _) } else { None };
        unsafe extern "C" fn destroy_func(data: glib_ffi::gpointer) {
            let _callback: Box_<Option<Box<dyn Fn(i32) -> i32 + 'static>>> = Box_::from_raw(data as *mut _);
        }
        let destroy_call3 = Some(destroy_func as _);
        let super_callback0: Box_<Option<Box<dyn Fn(i32) -> i32 + 'static>>> = page_func_data;
        unsafe {
            ffi::gtk_assistant_set_forward_page_func(self.as_ref().to_glib_none().0, page_func, Box::into_raw(super_callback0) as *mut _, destroy_call3);
        }
    }

    fn set_page_complete<P: IsA<Widget>>(&self, page: &P, complete: bool) {
        unsafe {
            ffi::gtk_assistant_set_page_complete(self.as_ref().to_glib_none().0, page.as_ref().to_glib_none().0, complete.to_glib());
        }
    }

    #[cfg(any(feature = "v3_18", feature = "dox"))]
    fn set_page_has_padding<P: IsA<Widget>>(&self, page: &P, has_padding: bool) {
        unsafe {
            ffi::gtk_assistant_set_page_has_padding(self.as_ref().to_glib_none().0, page.as_ref().to_glib_none().0, has_padding.to_glib());
        }
    }

    fn set_page_title<P: IsA<Widget>>(&self, page: &P, title: &str) {
        unsafe {
            ffi::gtk_assistant_set_page_title(self.as_ref().to_glib_none().0, page.as_ref().to_glib_none().0, title.to_glib_none().0);
        }
    }

    fn set_page_type<P: IsA<Widget>>(&self, page: &P, type_: AssistantPageType) {
        unsafe {
            ffi::gtk_assistant_set_page_type(self.as_ref().to_glib_none().0, page.as_ref().to_glib_none().0, type_.to_glib());
        }
    }

    fn update_buttons_state(&self) {
        unsafe {
            ffi::gtk_assistant_update_buttons_state(self.as_ref().to_glib_none().0);
        }
    }

    fn get_property_use_header_bar(&self) -> i32 {
        unsafe {
            let mut value = Value::from_type(<i32 as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0 as *mut gobject_ffi::GObject, b"use-header-bar\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn get_child_complete<T: IsA<Widget>>(&self, item: &T) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            ffi::gtk_container_child_get_property(self.to_glib_none().0 as *mut ffi::GtkContainer, item.to_glib_none().0 as *mut _, b"complete\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn set_child_complete<T: IsA<Widget>>(&self, item: &T, complete: bool) {
        unsafe {
            ffi::gtk_container_child_set_property(self.to_glib_none().0 as *mut ffi::GtkContainer, item.to_glib_none().0 as *mut _, b"complete\0".as_ptr() as *const _, Value::from(&complete).to_glib_none().0);
        }
    }

    fn get_child_has_padding<T: IsA<Widget>>(&self, item: &T) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            ffi::gtk_container_child_get_property(self.to_glib_none().0 as *mut ffi::GtkContainer, item.to_glib_none().0 as *mut _, b"has-padding\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn set_child_has_padding<T: IsA<Widget>>(&self, item: &T, has_padding: bool) {
        unsafe {
            ffi::gtk_container_child_set_property(self.to_glib_none().0 as *mut ffi::GtkContainer, item.to_glib_none().0 as *mut _, b"has-padding\0".as_ptr() as *const _, Value::from(&has_padding).to_glib_none().0);
        }
    }

    fn get_child_page_type<T: IsA<Widget>>(&self, item: &T) -> AssistantPageType {
        unsafe {
            let mut value = Value::from_type(<AssistantPageType as StaticType>::static_type());
            ffi::gtk_container_child_get_property(self.to_glib_none().0 as *mut ffi::GtkContainer, item.to_glib_none().0 as *mut _, b"page-type\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn set_child_page_type<T: IsA<Widget>>(&self, item: &T, page_type: AssistantPageType) {
        unsafe {
            ffi::gtk_container_child_set_property(self.to_glib_none().0 as *mut ffi::GtkContainer, item.to_glib_none().0 as *mut _, b"page-type\0".as_ptr() as *const _, Value::from(&page_type).to_glib_none().0);
        }
    }

    fn get_child_title<T: IsA<Widget>>(&self, item: &T) -> Option<GString> {
        unsafe {
            let mut value = Value::from_type(<GString as StaticType>::static_type());
            ffi::gtk_container_child_get_property(self.to_glib_none().0 as *mut ffi::GtkContainer, item.to_glib_none().0 as *mut _, b"title\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get()
        }
    }

    fn set_child_title<'a, P: Into<Option<&'a str>>, T: IsA<Widget>>(&self, item: &T, title: P) {
        let title = title.into();
        unsafe {
            ffi::gtk_container_child_set_property(self.to_glib_none().0 as *mut ffi::GtkContainer, item.to_glib_none().0 as *mut _, b"title\0".as_ptr() as *const _, Value::from(title).to_glib_none().0);
        }
    }

    fn connect_apply<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"apply\0".as_ptr() as *const _,
                Some(transmute(apply_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_cancel<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"cancel\0".as_ptr() as *const _,
                Some(transmute(cancel_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_close<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"close\0".as_ptr() as *const _,
                Some(transmute(close_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_escape<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"escape\0".as_ptr() as *const _,
                Some(transmute(escape_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn emit_escape(&self) {
        let _ = unsafe { glib::Object::from_glib_borrow(self.to_glib_none().0 as *mut gobject_ffi::GObject).emit("escape", &[]).unwrap() };
    }

    fn connect_prepare<F: Fn(&Self, &Widget) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"prepare\0".as_ptr() as *const _,
                Some(transmute(prepare_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }
}

unsafe extern "C" fn apply_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GtkAssistant, f: glib_ffi::gpointer)
where P: IsA<Assistant> {
    let f: &F = transmute(f);
    f(&Assistant::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn cancel_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GtkAssistant, f: glib_ffi::gpointer)
where P: IsA<Assistant> {
    let f: &F = transmute(f);
    f(&Assistant::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn close_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GtkAssistant, f: glib_ffi::gpointer)
where P: IsA<Assistant> {
    let f: &F = transmute(f);
    f(&Assistant::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn escape_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GtkAssistant, f: glib_ffi::gpointer)
where P: IsA<Assistant> {
    let f: &F = transmute(f);
    f(&Assistant::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn prepare_trampoline<P, F: Fn(&P, &Widget) + 'static>(this: *mut ffi::GtkAssistant, page: *mut ffi::GtkWidget, f: glib_ffi::gpointer)
where P: IsA<Assistant> {
    let f: &F = transmute(f);
    f(&Assistant::from_glib_borrow(this).unsafe_cast(), &from_glib_borrow(page))
}

impl fmt::Display for Assistant {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Assistant")
    }
}