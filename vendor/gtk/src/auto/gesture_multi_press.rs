// This file was generated by gir (d933f9a) from gir-files (469db10)
// DO NOT EDIT

use EventController;
use Gesture;
use GestureSingle;
#[cfg(any(feature = "v3_14", feature = "dox"))]
use Widget;
use ffi;
#[cfg(any(feature = "v3_14", feature = "dox"))]
use gdk;
use glib;
use glib::object::Downcast;
use glib::object::IsA;
#[cfg(any(feature = "v3_14", feature = "dox"))]
use glib::signal::SignalHandlerId;
#[cfg(any(feature = "v3_14", feature = "dox"))]
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
#[cfg(any(feature = "v3_14", feature = "dox"))]
use libc;
#[cfg(any(feature = "v3_14", feature = "dox"))]
use std::boxed::Box as Box_;
use std::mem;
#[cfg(any(feature = "v3_14", feature = "dox"))]
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct GestureMultiPress(Object<ffi::GtkGestureMultiPress, ffi::GtkGestureMultiPressClass>): GestureSingle, Gesture, EventController;

    match fn {
        get_type => || ffi::gtk_gesture_multi_press_get_type(),
    }
}

impl GestureMultiPress {
    #[cfg(any(feature = "v3_14", feature = "dox"))]
    pub fn new<P: IsA<Widget>>(widget: &P) -> GestureMultiPress {
        skip_assert_initialized!();
        unsafe {
            Gesture::from_glib_full(ffi::gtk_gesture_multi_press_new(widget.to_glib_none().0)).downcast_unchecked()
        }
    }
}

pub trait GestureMultiPressExt {
    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn get_area(&self) -> Option<gdk::Rectangle>;

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn set_area<'a, P: Into<Option<&'a gdk::Rectangle>>>(&self, rect: P);

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn connect_pressed<F: Fn(&Self, i32, f64, f64) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn connect_released<F: Fn(&Self, i32, f64, f64) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn connect_stopped<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<GestureMultiPress> + IsA<glib::object::Object>> GestureMultiPressExt for O {
    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn get_area(&self) -> Option<gdk::Rectangle> {
        unsafe {
            let mut rect = gdk::Rectangle::uninitialized();
            let ret = from_glib(ffi::gtk_gesture_multi_press_get_area(self.to_glib_none().0, rect.to_glib_none_mut().0));
            if ret { Some(rect) } else { None }
        }
    }

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn set_area<'a, P: Into<Option<&'a gdk::Rectangle>>>(&self, rect: P) {
        let rect = rect.into();
        let rect = rect.to_glib_none();
        unsafe {
            ffi::gtk_gesture_multi_press_set_area(self.to_glib_none().0, rect.0);
        }
    }

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn connect_pressed<F: Fn(&Self, i32, f64, f64) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self, i32, f64, f64) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "pressed",
                transmute(pressed_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn connect_released<F: Fn(&Self, i32, f64, f64) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self, i32, f64, f64) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "released",
                transmute(released_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn connect_stopped<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "stopped",
                transmute(stopped_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

#[cfg(any(feature = "v3_14", feature = "dox"))]
unsafe extern "C" fn pressed_trampoline<P>(this: *mut ffi::GtkGestureMultiPress, n_press: libc::c_int, x: libc::c_double, y: libc::c_double, f: glib_ffi::gpointer)
where P: IsA<GestureMultiPress> {
    callback_guard!();
    let f: &&(Fn(&P, i32, f64, f64) + 'static) = transmute(f);
    f(&GestureMultiPress::from_glib_borrow(this).downcast_unchecked(), n_press, x, y)
}

#[cfg(any(feature = "v3_14", feature = "dox"))]
unsafe extern "C" fn released_trampoline<P>(this: *mut ffi::GtkGestureMultiPress, n_press: libc::c_int, x: libc::c_double, y: libc::c_double, f: glib_ffi::gpointer)
where P: IsA<GestureMultiPress> {
    callback_guard!();
    let f: &&(Fn(&P, i32, f64, f64) + 'static) = transmute(f);
    f(&GestureMultiPress::from_glib_borrow(this).downcast_unchecked(), n_press, x, y)
}

#[cfg(any(feature = "v3_14", feature = "dox"))]
unsafe extern "C" fn stopped_trampoline<P>(this: *mut ffi::GtkGestureMultiPress, f: glib_ffi::gpointer)
where P: IsA<GestureMultiPress> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&GestureMultiPress::from_glib_borrow(this).downcast_unchecked())
}