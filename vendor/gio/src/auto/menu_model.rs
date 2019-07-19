// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use MenuAttributeIter;
use MenuLinkIter;
use ffi;
use glib;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect_raw;
use glib::translate::*;
use glib_ffi;
use libc;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib_wrapper! {
    pub struct MenuModel(Object<ffi::GMenuModel, ffi::GMenuModelClass, MenuModelClass>);

    match fn {
        get_type => || ffi::g_menu_model_get_type(),
    }
}

pub const NONE_MENU_MODEL: Option<&MenuModel> = None;

pub trait MenuModelExt: 'static {
    //fn get_item_attribute(&self, item_index: i32, attribute: &str, format_string: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) -> bool;

    fn get_item_attribute_value<'a, P: Into<Option<&'a glib::VariantTy>>>(&self, item_index: i32, attribute: &str, expected_type: P) -> Option<glib::Variant>;

    fn get_item_link(&self, item_index: i32, link: &str) -> Option<MenuModel>;

    fn get_n_items(&self) -> i32;

    fn is_mutable(&self) -> bool;

    fn items_changed(&self, position: i32, removed: i32, added: i32);

    fn iterate_item_attributes(&self, item_index: i32) -> Option<MenuAttributeIter>;

    fn iterate_item_links(&self, item_index: i32) -> Option<MenuLinkIter>;

    fn connect_items_changed<F: Fn(&Self, i32, i32, i32) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<MenuModel>> MenuModelExt for O {
    //fn get_item_attribute(&self, item_index: i32, attribute: &str, format_string: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) -> bool {
    //    unsafe { TODO: call ffi::g_menu_model_get_item_attribute() }
    //}

    fn get_item_attribute_value<'a, P: Into<Option<&'a glib::VariantTy>>>(&self, item_index: i32, attribute: &str, expected_type: P) -> Option<glib::Variant> {
        let expected_type = expected_type.into();
        unsafe {
            from_glib_full(ffi::g_menu_model_get_item_attribute_value(self.as_ref().to_glib_none().0, item_index, attribute.to_glib_none().0, expected_type.to_glib_none().0))
        }
    }

    fn get_item_link(&self, item_index: i32, link: &str) -> Option<MenuModel> {
        unsafe {
            from_glib_full(ffi::g_menu_model_get_item_link(self.as_ref().to_glib_none().0, item_index, link.to_glib_none().0))
        }
    }

    fn get_n_items(&self) -> i32 {
        unsafe {
            ffi::g_menu_model_get_n_items(self.as_ref().to_glib_none().0)
        }
    }

    fn is_mutable(&self) -> bool {
        unsafe {
            from_glib(ffi::g_menu_model_is_mutable(self.as_ref().to_glib_none().0))
        }
    }

    fn items_changed(&self, position: i32, removed: i32, added: i32) {
        unsafe {
            ffi::g_menu_model_items_changed(self.as_ref().to_glib_none().0, position, removed, added);
        }
    }

    fn iterate_item_attributes(&self, item_index: i32) -> Option<MenuAttributeIter> {
        unsafe {
            from_glib_full(ffi::g_menu_model_iterate_item_attributes(self.as_ref().to_glib_none().0, item_index))
        }
    }

    fn iterate_item_links(&self, item_index: i32) -> Option<MenuLinkIter> {
        unsafe {
            from_glib_full(ffi::g_menu_model_iterate_item_links(self.as_ref().to_glib_none().0, item_index))
        }
    }

    fn connect_items_changed<F: Fn(&Self, i32, i32, i32) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"items-changed\0".as_ptr() as *const _,
                Some(transmute(items_changed_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }
}

unsafe extern "C" fn items_changed_trampoline<P, F: Fn(&P, i32, i32, i32) + 'static>(this: *mut ffi::GMenuModel, position: libc::c_int, removed: libc::c_int, added: libc::c_int, f: glib_ffi::gpointer)
where P: IsA<MenuModel> {
    let f: &F = transmute(f);
    f(&MenuModel::from_glib_borrow(this).unsafe_cast(), position, removed, added)
}

impl fmt::Display for MenuModel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MenuModel")
    }
}