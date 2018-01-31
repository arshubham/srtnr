#[cfg(feature = "use_glib")]
use glib::translate::*;
#[cfg(feature = "use_glib")]
use glib_ffi;
#[cfg(feature = "use_glib")]
use std::ptr;
#[cfg(feature = "use_glib")]
use std::mem;
use std::cmp::PartialEq;
use ffi;

use ffi::enums::{
    Antialias,
    SubpixelOrder,
    HintStyle,
    HintMetrics,
};

#[cfg(feature = "use_glib")]
glib_wrapper! {
    pub struct FontOptions(Boxed<ffi::cairo_font_options_t>);

    match fn {
        copy => |ptr| {
            let ptr = ffi::cairo_font_options_copy(ptr);
            let status = ffi::cairo_font_options_status(ptr);
            status.ensure_valid();
            ptr
        },
        free => |ptr| ffi::cairo_font_options_destroy(ptr),
    }
}

#[cfg(not(feature = "use_glib"))]
pub struct FontOptions(*mut ffi::cairo_font_options_t);

impl FontOptions {
    pub fn new() -> FontOptions {
        let font_options: FontOptions = unsafe {
            FontOptions::from_raw_full(ffi::cairo_font_options_create())
        };
        font_options.ensure_status();
        font_options
    }

    #[cfg(feature = "use_glib")]
    #[doc(hidden)]
    pub unsafe fn from_raw_full(ptr: *mut ffi::cairo_font_options_t) -> FontOptions {
        from_glib_full(ptr)
    }

    #[cfg(not(feature = "use_glib"))]
    #[doc(hidden)]
    pub unsafe fn from_raw_full(ptr: *mut ffi::cairo_font_options_t) -> FontOptions {
        assert!(!ptr.is_null());
        FontOptions(ptr)
    }

    #[cfg(feature = "use_glib")]
    #[doc(hidden)]
    pub fn to_raw_none(&self) -> *mut ffi::cairo_font_options_t {
        mut_override(self.to_glib_none().0)
    }

    #[cfg(not(feature = "use_glib"))]
    #[doc(hidden)]
    pub fn to_raw_none(&self) -> *mut ffi::cairo_font_options_t {
        self.0
    }

    pub fn ensure_status(&self) {
        let status = unsafe {
            ffi::cairo_font_options_status(self.to_raw_none())
        };
        status.ensure_valid()
    }

    pub fn merge(&mut self, other: &FontOptions) {
        unsafe {
            ffi::cairo_font_options_merge(self.to_raw_none(), other.to_raw_none())
        }
    }

    pub fn hash(&self) -> u64{
        unsafe {
            ffi::cairo_font_options_hash(self.to_raw_none()) as u64
        }
    }

    pub fn set_antialias(&mut self, antialias: Antialias) {
        unsafe {
            ffi::cairo_font_options_set_antialias(self.to_raw_none(), antialias)
        }
    }

    pub fn get_antialias(&self) -> Antialias {
        unsafe {
            ffi::cairo_font_options_get_antialias(self.to_raw_none())
        }
    }

    pub fn set_subpixel_order(&mut self, order: SubpixelOrder) {
        unsafe {
            ffi::cairo_font_options_set_subpixel_order(self.to_raw_none(), order)
        }
    }

    pub fn get_subpixel_order(&self) -> SubpixelOrder {
        unsafe {
            ffi::cairo_font_options_get_subpixel_order(self.to_raw_none())
        }
    }

    pub fn set_hint_style(&mut self, hint_style: HintStyle) {
        unsafe {
            ffi::cairo_font_options_set_hint_style(self.to_raw_none(), hint_style)
        }
    }

    pub fn get_hint_style(&self) -> HintStyle {
        unsafe {
            ffi::cairo_font_options_get_hint_style(self.to_raw_none())
        }
    }

    pub fn set_hint_metrics(&mut self, hint_metrics: HintMetrics) {
        unsafe {
            ffi::cairo_font_options_set_hint_metrics(self.to_raw_none(), hint_metrics)
        }
    }

    pub fn get_hint_metrics(&self) -> HintMetrics {
        unsafe {
            ffi::cairo_font_options_get_hint_metrics(self.to_raw_none())
        }
    }
}

impl PartialEq for FontOptions {
    fn eq(&self, other: &FontOptions) -> bool {
        unsafe {
            ffi::cairo_font_options_equal(self.to_raw_none(), other.to_raw_none()).as_bool()
        }
    }
}
