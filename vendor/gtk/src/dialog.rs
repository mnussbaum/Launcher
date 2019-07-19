// Copyright 2016, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use ffi;
use glib::object::Cast;
use glib::translate::*;
use std::ptr;
use Dialog;
use DialogFlags;
use IsA;
use Widget;
use Window;
use auto::DialogExt;
use ResponseType;

impl Dialog {
    pub fn new_with_buttons<'a, I: Into<Option<&'a str>>, T: IsA<Window>>(
        title: I,
        parent: Option<&T>,
        flags: DialogFlags,
        buttons: &[(&str, ResponseType)],
    ) -> Dialog {
        assert_initialized_main_thread!();
        let title = title.into();
        let ret: Dialog = unsafe {
            Widget::from_glib_none(
                ffi::gtk_dialog_new_with_buttons(title.to_glib_none().0, parent.map(|p| p.as_ref()).to_glib_none().0,
                    flags.to_glib(), ptr::null_mut()))
                .unsafe_cast()
        };

        ret.add_buttons(buttons);
        ret
    }
}

pub trait DialogExtManual: 'static {
    fn add_buttons(&self, buttons: &[(&str, ResponseType)]);
}

impl<O: DialogExt> DialogExtManual for O {
    fn add_buttons(&self, buttons: &[(&str, ResponseType)]) {
        for &(text, id) in buttons {
            //FIXME: self.add_button don't work on 1.8
            O::add_button(self, text, id);
        }
    }
}
