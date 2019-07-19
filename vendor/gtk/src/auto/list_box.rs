// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Adjustment;
use Buildable;
use Container;
use ListBoxRow;
use MovementStep;
use SelectionMode;
use Widget;
use ffi;
#[cfg(any(feature = "v3_16", feature = "dox"))]
use gio;
use glib;
use glib::object::Cast;
use glib::object::IsA;
use glib::object::ObjectExt;
use glib::signal::SignalHandlerId;
use glib::signal::connect_raw;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use libc;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib_wrapper! {
    pub struct ListBox(Object<ffi::GtkListBox, ffi::GtkListBoxClass, ListBoxClass>) @extends Container, Widget, @implements Buildable;

    match fn {
        get_type => || ffi::gtk_list_box_get_type(),
    }
}

impl ListBox {
    pub fn new() -> ListBox {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_list_box_new()).unsafe_cast()
        }
    }
}

impl Default for ListBox {
    fn default() -> Self {
        Self::new()
    }
}

pub const NONE_LIST_BOX: Option<&ListBox> = None;

pub trait ListBoxExt: 'static {
    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn bind_model<'a, P: IsA<gio::ListModel> + 'a, Q: Into<Option<&'a P>>, R: Fn(&glib::Object) -> Widget + 'static>(&self, model: Q, create_widget_func: R);

    fn drag_highlight_row<P: IsA<ListBoxRow>>(&self, row: &P);

    fn drag_unhighlight_row(&self);

    fn get_activate_on_single_click(&self) -> bool;

    fn get_adjustment(&self) -> Option<Adjustment>;

    fn get_row_at_index(&self, index_: i32) -> Option<ListBoxRow>;

    fn get_row_at_y(&self, y: i32) -> Option<ListBoxRow>;

    fn get_selected_row(&self) -> Option<ListBoxRow>;

    fn get_selected_rows(&self) -> Vec<ListBoxRow>;

    fn get_selection_mode(&self) -> SelectionMode;

    fn insert<P: IsA<Widget>>(&self, child: &P, position: i32);

    fn invalidate_filter(&self);

    fn invalidate_headers(&self);

    fn invalidate_sort(&self);

    fn prepend<P: IsA<Widget>>(&self, child: &P);

    fn select_all(&self);

    fn select_row<'a, P: IsA<ListBoxRow> + 'a, Q: Into<Option<&'a P>>>(&self, row: Q);

    fn selected_foreach<P: FnMut(&ListBox, &ListBoxRow)>(&self, func: P);

    fn set_activate_on_single_click(&self, single: bool);

    fn set_adjustment<'a, P: IsA<Adjustment> + 'a, Q: Into<Option<&'a P>>>(&self, adjustment: Q);

    fn set_filter_func(&self, filter_func: Option<Box<dyn Fn(&ListBoxRow) -> bool + 'static>>);

    fn set_header_func(&self, update_header: Option<Box<dyn Fn(&ListBoxRow, &ListBoxRow) + 'static>>);

    fn set_placeholder<'a, P: IsA<Widget> + 'a, Q: Into<Option<&'a P>>>(&self, placeholder: Q);

    fn set_selection_mode(&self, mode: SelectionMode);

    fn set_sort_func(&self, sort_func: Option<Box<dyn Fn(&ListBoxRow, &ListBoxRow) -> i32 + 'static>>);

    fn unselect_all(&self);

    fn unselect_row<P: IsA<ListBoxRow>>(&self, row: &P);

    fn connect_activate_cursor_row<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn emit_activate_cursor_row(&self);

    fn connect_move_cursor<F: Fn(&Self, MovementStep, i32) + 'static>(&self, f: F) -> SignalHandlerId;

    fn emit_move_cursor(&self, object: MovementStep, p0: i32);

    fn connect_row_activated<F: Fn(&Self, &ListBoxRow) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_row_selected<F: Fn(&Self, &Option<ListBoxRow>) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_select_all<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn emit_select_all(&self);

    fn connect_selected_rows_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_toggle_cursor_row<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn emit_toggle_cursor_row(&self);

    fn connect_unselect_all<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn emit_unselect_all(&self);

    fn connect_property_activate_on_single_click_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_selection_mode_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<ListBox>> ListBoxExt for O {
    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn bind_model<'a, P: IsA<gio::ListModel> + 'a, Q: Into<Option<&'a P>>, R: Fn(&glib::Object) -> Widget + 'static>(&self, model: Q, create_widget_func: R) {
        let model = model.into();
        let create_widget_func_data: Box_<R> = Box::new(create_widget_func);
        unsafe extern "C" fn create_widget_func_func<'a, P: IsA<gio::ListModel> + 'a, Q: Into<Option<&'a P>>, R: Fn(&glib::Object) -> Widget + 'static>(item: *mut gobject_ffi::GObject, user_data: glib_ffi::gpointer) -> *mut ffi::GtkWidget {
            let item = from_glib_borrow(item);
            let callback: &R = &*(user_data as *mut _);
            let res = (*callback)(&item);
            res.to_glib_full()
        }
        let create_widget_func = Some(create_widget_func_func::<'a, P, Q, R> as _);
        unsafe extern "C" fn user_data_free_func_func<'a, P: IsA<gio::ListModel> + 'a, Q: Into<Option<&'a P>>, R: Fn(&glib::Object) -> Widget + 'static>(data: glib_ffi::gpointer) {
            let _callback: Box_<R> = Box_::from_raw(data as *mut _);
        }
        let destroy_call4 = Some(user_data_free_func_func::<'a, P, Q, R> as _);
        let super_callback0: Box_<R> = create_widget_func_data;
        unsafe {
            ffi::gtk_list_box_bind_model(self.as_ref().to_glib_none().0, model.map(|p| p.as_ref()).to_glib_none().0, create_widget_func, Box::into_raw(super_callback0) as *mut _, destroy_call4);
        }
    }

    fn drag_highlight_row<P: IsA<ListBoxRow>>(&self, row: &P) {
        unsafe {
            ffi::gtk_list_box_drag_highlight_row(self.as_ref().to_glib_none().0, row.as_ref().to_glib_none().0);
        }
    }

    fn drag_unhighlight_row(&self) {
        unsafe {
            ffi::gtk_list_box_drag_unhighlight_row(self.as_ref().to_glib_none().0);
        }
    }

    fn get_activate_on_single_click(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_list_box_get_activate_on_single_click(self.as_ref().to_glib_none().0))
        }
    }

    fn get_adjustment(&self) -> Option<Adjustment> {
        unsafe {
            from_glib_none(ffi::gtk_list_box_get_adjustment(self.as_ref().to_glib_none().0))
        }
    }

    fn get_row_at_index(&self, index_: i32) -> Option<ListBoxRow> {
        unsafe {
            from_glib_none(ffi::gtk_list_box_get_row_at_index(self.as_ref().to_glib_none().0, index_))
        }
    }

    fn get_row_at_y(&self, y: i32) -> Option<ListBoxRow> {
        unsafe {
            from_glib_none(ffi::gtk_list_box_get_row_at_y(self.as_ref().to_glib_none().0, y))
        }
    }

    fn get_selected_row(&self) -> Option<ListBoxRow> {
        unsafe {
            from_glib_none(ffi::gtk_list_box_get_selected_row(self.as_ref().to_glib_none().0))
        }
    }

    fn get_selected_rows(&self) -> Vec<ListBoxRow> {
        unsafe {
            FromGlibPtrContainer::from_glib_container(ffi::gtk_list_box_get_selected_rows(self.as_ref().to_glib_none().0))
        }
    }

    fn get_selection_mode(&self) -> SelectionMode {
        unsafe {
            from_glib(ffi::gtk_list_box_get_selection_mode(self.as_ref().to_glib_none().0))
        }
    }

    fn insert<P: IsA<Widget>>(&self, child: &P, position: i32) {
        unsafe {
            ffi::gtk_list_box_insert(self.as_ref().to_glib_none().0, child.as_ref().to_glib_none().0, position);
        }
    }

    fn invalidate_filter(&self) {
        unsafe {
            ffi::gtk_list_box_invalidate_filter(self.as_ref().to_glib_none().0);
        }
    }

    fn invalidate_headers(&self) {
        unsafe {
            ffi::gtk_list_box_invalidate_headers(self.as_ref().to_glib_none().0);
        }
    }

    fn invalidate_sort(&self) {
        unsafe {
            ffi::gtk_list_box_invalidate_sort(self.as_ref().to_glib_none().0);
        }
    }

    fn prepend<P: IsA<Widget>>(&self, child: &P) {
        unsafe {
            ffi::gtk_list_box_prepend(self.as_ref().to_glib_none().0, child.as_ref().to_glib_none().0);
        }
    }

    fn select_all(&self) {
        unsafe {
            ffi::gtk_list_box_select_all(self.as_ref().to_glib_none().0);
        }
    }

    fn select_row<'a, P: IsA<ListBoxRow> + 'a, Q: Into<Option<&'a P>>>(&self, row: Q) {
        let row = row.into();
        unsafe {
            ffi::gtk_list_box_select_row(self.as_ref().to_glib_none().0, row.map(|p| p.as_ref()).to_glib_none().0);
        }
    }

    fn selected_foreach<P: FnMut(&ListBox, &ListBoxRow)>(&self, func: P) {
        let func_data: P = func;
        unsafe extern "C" fn func_func<P: FnMut(&ListBox, &ListBoxRow)>(box_: *mut ffi::GtkListBox, row: *mut ffi::GtkListBoxRow, user_data: glib_ffi::gpointer) {
            let box_ = from_glib_borrow(box_);
            let row = from_glib_borrow(row);
            let callback: *mut P = user_data as *const _ as usize as *mut P;
            (*callback)(&box_, &row);
        }
        let func = Some(func_func::<P> as _);
        let super_callback0: &P = &func_data;
        unsafe {
            ffi::gtk_list_box_selected_foreach(self.as_ref().to_glib_none().0, func, super_callback0 as *const _ as usize as *mut _);
        }
    }

    fn set_activate_on_single_click(&self, single: bool) {
        unsafe {
            ffi::gtk_list_box_set_activate_on_single_click(self.as_ref().to_glib_none().0, single.to_glib());
        }
    }

    fn set_adjustment<'a, P: IsA<Adjustment> + 'a, Q: Into<Option<&'a P>>>(&self, adjustment: Q) {
        let adjustment = adjustment.into();
        unsafe {
            ffi::gtk_list_box_set_adjustment(self.as_ref().to_glib_none().0, adjustment.map(|p| p.as_ref()).to_glib_none().0);
        }
    }

    fn set_filter_func(&self, filter_func: Option<Box<dyn Fn(&ListBoxRow) -> bool + 'static>>) {
        let filter_func_data: Box_<Option<Box<dyn Fn(&ListBoxRow) -> bool + 'static>>> = Box::new(filter_func);
        unsafe extern "C" fn filter_func_func(row: *mut ffi::GtkListBoxRow, user_data: glib_ffi::gpointer) -> glib_ffi::gboolean {
            let row = from_glib_borrow(row);
            let callback: &Option<Box<dyn Fn(&ListBoxRow) -> bool + 'static>> = &*(user_data as *mut _);
            let res = if let Some(ref callback) = *callback {
                callback(&row)
            } else {
                panic!("cannot get closure...")
            };
            res.to_glib()
        }
        let filter_func = if filter_func_data.is_some() { Some(filter_func_func as _) } else { None };
        unsafe extern "C" fn destroy_func(data: glib_ffi::gpointer) {
            let _callback: Box_<Option<Box<dyn Fn(&ListBoxRow) -> bool + 'static>>> = Box_::from_raw(data as *mut _);
        }
        let destroy_call3 = Some(destroy_func as _);
        let super_callback0: Box_<Option<Box<dyn Fn(&ListBoxRow) -> bool + 'static>>> = filter_func_data;
        unsafe {
            ffi::gtk_list_box_set_filter_func(self.as_ref().to_glib_none().0, filter_func, Box::into_raw(super_callback0) as *mut _, destroy_call3);
        }
    }

    fn set_header_func(&self, update_header: Option<Box<dyn Fn(&ListBoxRow, &ListBoxRow) + 'static>>) {
        let update_header_data: Box_<Option<Box<dyn Fn(&ListBoxRow, &ListBoxRow) + 'static>>> = Box::new(update_header);
        unsafe extern "C" fn update_header_func(row: *mut ffi::GtkListBoxRow, before: *mut ffi::GtkListBoxRow, user_data: glib_ffi::gpointer) {
            let row = from_glib_borrow(row);
            let before = from_glib_borrow(before);
            let callback: &Option<Box<dyn Fn(&ListBoxRow, &ListBoxRow) + 'static>> = &*(user_data as *mut _);
            if let Some(ref callback) = *callback {
                callback(&row, &before)
            } else {
                panic!("cannot get closure...")
            };
        }
        let update_header = if update_header_data.is_some() { Some(update_header_func as _) } else { None };
        unsafe extern "C" fn destroy_func(data: glib_ffi::gpointer) {
            let _callback: Box_<Option<Box<dyn Fn(&ListBoxRow, &ListBoxRow) + 'static>>> = Box_::from_raw(data as *mut _);
        }
        let destroy_call3 = Some(destroy_func as _);
        let super_callback0: Box_<Option<Box<dyn Fn(&ListBoxRow, &ListBoxRow) + 'static>>> = update_header_data;
        unsafe {
            ffi::gtk_list_box_set_header_func(self.as_ref().to_glib_none().0, update_header, Box::into_raw(super_callback0) as *mut _, destroy_call3);
        }
    }

    fn set_placeholder<'a, P: IsA<Widget> + 'a, Q: Into<Option<&'a P>>>(&self, placeholder: Q) {
        let placeholder = placeholder.into();
        unsafe {
            ffi::gtk_list_box_set_placeholder(self.as_ref().to_glib_none().0, placeholder.map(|p| p.as_ref()).to_glib_none().0);
        }
    }

    fn set_selection_mode(&self, mode: SelectionMode) {
        unsafe {
            ffi::gtk_list_box_set_selection_mode(self.as_ref().to_glib_none().0, mode.to_glib());
        }
    }

    fn set_sort_func(&self, sort_func: Option<Box<dyn Fn(&ListBoxRow, &ListBoxRow) -> i32 + 'static>>) {
        let sort_func_data: Box_<Option<Box<dyn Fn(&ListBoxRow, &ListBoxRow) -> i32 + 'static>>> = Box::new(sort_func);
        unsafe extern "C" fn sort_func_func(row1: *mut ffi::GtkListBoxRow, row2: *mut ffi::GtkListBoxRow, user_data: glib_ffi::gpointer) -> libc::c_int {
            let row1 = from_glib_borrow(row1);
            let row2 = from_glib_borrow(row2);
            let callback: &Option<Box<dyn Fn(&ListBoxRow, &ListBoxRow) -> i32 + 'static>> = &*(user_data as *mut _);
            let res = if let Some(ref callback) = *callback {
                callback(&row1, &row2)
            } else {
                panic!("cannot get closure...")
            };
            res
        }
        let sort_func = if sort_func_data.is_some() { Some(sort_func_func as _) } else { None };
        unsafe extern "C" fn destroy_func(data: glib_ffi::gpointer) {
            let _callback: Box_<Option<Box<dyn Fn(&ListBoxRow, &ListBoxRow) -> i32 + 'static>>> = Box_::from_raw(data as *mut _);
        }
        let destroy_call3 = Some(destroy_func as _);
        let super_callback0: Box_<Option<Box<dyn Fn(&ListBoxRow, &ListBoxRow) -> i32 + 'static>>> = sort_func_data;
        unsafe {
            ffi::gtk_list_box_set_sort_func(self.as_ref().to_glib_none().0, sort_func, Box::into_raw(super_callback0) as *mut _, destroy_call3);
        }
    }

    fn unselect_all(&self) {
        unsafe {
            ffi::gtk_list_box_unselect_all(self.as_ref().to_glib_none().0);
        }
    }

    fn unselect_row<P: IsA<ListBoxRow>>(&self, row: &P) {
        unsafe {
            ffi::gtk_list_box_unselect_row(self.as_ref().to_glib_none().0, row.as_ref().to_glib_none().0);
        }
    }

    fn connect_activate_cursor_row<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"activate-cursor-row\0".as_ptr() as *const _,
                Some(transmute(activate_cursor_row_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn emit_activate_cursor_row(&self) {
        let _ = unsafe { glib::Object::from_glib_borrow(self.to_glib_none().0 as *mut gobject_ffi::GObject).emit("activate-cursor-row", &[]).unwrap() };
    }

    fn connect_move_cursor<F: Fn(&Self, MovementStep, i32) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"move-cursor\0".as_ptr() as *const _,
                Some(transmute(move_cursor_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn emit_move_cursor(&self, object: MovementStep, p0: i32) {
        let _ = unsafe { glib::Object::from_glib_borrow(self.to_glib_none().0 as *mut gobject_ffi::GObject).emit("move-cursor", &[&object, &p0]).unwrap() };
    }

    fn connect_row_activated<F: Fn(&Self, &ListBoxRow) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"row-activated\0".as_ptr() as *const _,
                Some(transmute(row_activated_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_row_selected<F: Fn(&Self, &Option<ListBoxRow>) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"row-selected\0".as_ptr() as *const _,
                Some(transmute(row_selected_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_select_all<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"select-all\0".as_ptr() as *const _,
                Some(transmute(select_all_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn emit_select_all(&self) {
        let _ = unsafe { glib::Object::from_glib_borrow(self.to_glib_none().0 as *mut gobject_ffi::GObject).emit("select-all", &[]).unwrap() };
    }

    fn connect_selected_rows_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"selected-rows-changed\0".as_ptr() as *const _,
                Some(transmute(selected_rows_changed_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_toggle_cursor_row<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"toggle-cursor-row\0".as_ptr() as *const _,
                Some(transmute(toggle_cursor_row_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn emit_toggle_cursor_row(&self) {
        let _ = unsafe { glib::Object::from_glib_borrow(self.to_glib_none().0 as *mut gobject_ffi::GObject).emit("toggle-cursor-row", &[]).unwrap() };
    }

    fn connect_unselect_all<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"unselect-all\0".as_ptr() as *const _,
                Some(transmute(unselect_all_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn emit_unselect_all(&self) {
        let _ = unsafe { glib::Object::from_glib_borrow(self.to_glib_none().0 as *mut gobject_ffi::GObject).emit("unselect-all", &[]).unwrap() };
    }

    fn connect_property_activate_on_single_click_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::activate-on-single-click\0".as_ptr() as *const _,
                Some(transmute(notify_activate_on_single_click_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_selection_mode_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::selection-mode\0".as_ptr() as *const _,
                Some(transmute(notify_selection_mode_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }
}

unsafe extern "C" fn activate_cursor_row_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GtkListBox, f: glib_ffi::gpointer)
where P: IsA<ListBox> {
    let f: &F = transmute(f);
    f(&ListBox::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn move_cursor_trampoline<P, F: Fn(&P, MovementStep, i32) + 'static>(this: *mut ffi::GtkListBox, object: ffi::GtkMovementStep, p0: libc::c_int, f: glib_ffi::gpointer)
where P: IsA<ListBox> {
    let f: &F = transmute(f);
    f(&ListBox::from_glib_borrow(this).unsafe_cast(), from_glib(object), p0)
}

unsafe extern "C" fn row_activated_trampoline<P, F: Fn(&P, &ListBoxRow) + 'static>(this: *mut ffi::GtkListBox, row: *mut ffi::GtkListBoxRow, f: glib_ffi::gpointer)
where P: IsA<ListBox> {
    let f: &F = transmute(f);
    f(&ListBox::from_glib_borrow(this).unsafe_cast(), &from_glib_borrow(row))
}

unsafe extern "C" fn row_selected_trampoline<P, F: Fn(&P, &Option<ListBoxRow>) + 'static>(this: *mut ffi::GtkListBox, row: *mut ffi::GtkListBoxRow, f: glib_ffi::gpointer)
where P: IsA<ListBox> {
    let f: &F = transmute(f);
    f(&ListBox::from_glib_borrow(this).unsafe_cast(), &from_glib_borrow(row))
}

unsafe extern "C" fn select_all_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GtkListBox, f: glib_ffi::gpointer)
where P: IsA<ListBox> {
    let f: &F = transmute(f);
    f(&ListBox::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn selected_rows_changed_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GtkListBox, f: glib_ffi::gpointer)
where P: IsA<ListBox> {
    let f: &F = transmute(f);
    f(&ListBox::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn toggle_cursor_row_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GtkListBox, f: glib_ffi::gpointer)
where P: IsA<ListBox> {
    let f: &F = transmute(f);
    f(&ListBox::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn unselect_all_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GtkListBox, f: glib_ffi::gpointer)
where P: IsA<ListBox> {
    let f: &F = transmute(f);
    f(&ListBox::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_activate_on_single_click_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GtkListBox, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<ListBox> {
    let f: &F = transmute(f);
    f(&ListBox::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_selection_mode_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GtkListBox, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<ListBox> {
    let f: &F = transmute(f);
    f(&ListBox::from_glib_borrow(this).unsafe_cast())
}

impl fmt::Display for ListBox {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ListBox")
    }
}