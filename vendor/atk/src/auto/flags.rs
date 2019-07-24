// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use atk_sys;
use glib::translate::*;
use glib::value::FromValue;
use glib::value::FromValueOptional;
use glib::value::SetValue;
use glib::value::Value;
use glib::StaticType;
use glib::Type;
use gobject_sys;

bitflags! {
    pub struct HyperlinkStateFlags: u32 {
        const INLINE = 1;
    }
}

#[doc(hidden)]
impl ToGlib for HyperlinkStateFlags {
    type GlibType = atk_sys::AtkHyperlinkStateFlags;

    fn to_glib(&self) -> atk_sys::AtkHyperlinkStateFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<atk_sys::AtkHyperlinkStateFlags> for HyperlinkStateFlags {
    fn from_glib(value: atk_sys::AtkHyperlinkStateFlags) -> HyperlinkStateFlags {
        skip_assert_initialized!();
        HyperlinkStateFlags::from_bits_truncate(value)
    }
}

impl StaticType for HyperlinkStateFlags {
    fn static_type() -> Type {
        unsafe { from_glib(atk_sys::atk_hyperlink_state_flags_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for HyperlinkStateFlags {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for HyperlinkStateFlags {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_sys::g_value_get_flags(value.to_glib_none().0))
    }
}

impl SetValue for HyperlinkStateFlags {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_sys::g_value_set_flags(value.to_glib_none_mut().0, this.to_glib())
    }
}
