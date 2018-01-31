// This file was generated by gir (d933f9a) from gir-files (469db10)
// DO NOT EDIT

use Actionable;
use Bin;
use Buildable;
use Button;
use Container;
use Orientable;
use ScaleButton;
use Widget;
use ffi;
use glib;
use glib::Value;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct VolumeButton(Object<ffi::GtkVolumeButton, ffi::GtkVolumeButtonClass>): ScaleButton, Button, Bin, Container, Widget, Buildable, Actionable, Orientable;

    match fn {
        get_type => || ffi::gtk_volume_button_get_type(),
    }
}

impl VolumeButton {
    pub fn new() -> VolumeButton {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_volume_button_new()).downcast_unchecked()
        }
    }
}

impl Default for VolumeButton {
    fn default() -> Self {
        Self::new()
    }
}

pub trait VolumeButtonExt {
    fn get_property_use_symbolic(&self) -> bool;

    fn set_property_use_symbolic(&self, use_symbolic: bool);

    fn connect_property_use_symbolic_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<VolumeButton> + IsA<glib::object::Object>> VolumeButtonExt for O {
    fn get_property_use_symbolic(&self) -> bool {
        let mut value = Value::from(&false);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "use-symbolic".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    fn set_property_use_symbolic(&self, use_symbolic: bool) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "use-symbolic".to_glib_none().0, Value::from(&use_symbolic).to_glib_none().0);
        }
    }

    fn connect_property_use_symbolic_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::use-symbolic",
                transmute(notify_use_symbolic_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn notify_use_symbolic_trampoline<P>(this: *mut ffi::GtkVolumeButton, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<VolumeButton> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&VolumeButton::from_glib_borrow(this).downcast_unchecked())
}
