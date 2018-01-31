// This file was generated by gir (d933f9a) from gir-files (469db10)
// DO NOT EDIT

use Actionable;
#[cfg(any(feature = "v3_6", feature = "dox"))]
use ArrowType;
use Bin;
use Buildable;
use Button;
use Container;
#[cfg(any(feature = "v3_6", feature = "dox"))]
use Menu;
#[cfg(any(feature = "v3_12", feature = "dox"))]
use Popover;
use ToggleButton;
use Widget;
use ffi;
#[cfg(any(feature = "v3_6", feature = "dox"))]
use gio;
use glib;
use glib::object::Downcast;
use glib::object::IsA;
#[cfg(any(feature = "v3_6", feature = "dox"))]
use glib::signal::SignalHandlerId;
#[cfg(any(feature = "v3_6", feature = "dox"))]
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
#[cfg(any(feature = "v3_6", feature = "dox"))]
use std::boxed::Box as Box_;
use std::mem;
#[cfg(any(feature = "v3_6", feature = "dox"))]
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct MenuButton(Object<ffi::GtkMenuButton, ffi::GtkMenuButtonClass>): ToggleButton, Button, Bin, Container, Widget, Buildable, Actionable;

    match fn {
        get_type => || ffi::gtk_menu_button_get_type(),
    }
}

impl MenuButton {
    #[cfg(any(feature = "v3_6", feature = "dox"))]
    pub fn new() -> MenuButton {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_menu_button_new()).downcast_unchecked()
        }
    }
}

#[cfg(any(feature = "v3_6", feature = "dox"))]
impl Default for MenuButton {
    fn default() -> Self {
        Self::new()
    }
}

pub trait MenuButtonExt {
    #[cfg(any(feature = "v3_6", feature = "dox"))]
    fn get_align_widget(&self) -> Option<Widget>;

    #[cfg(any(feature = "v3_6", feature = "dox"))]
    fn get_direction(&self) -> ArrowType;

    #[cfg(any(feature = "v3_6", feature = "dox"))]
    fn get_menu_model(&self) -> Option<gio::MenuModel>;

    #[cfg(any(feature = "v3_12", feature = "dox"))]
    fn get_popover(&self) -> Option<Popover>;

    #[cfg(any(feature = "v3_6", feature = "dox"))]
    fn get_popup(&self) -> Option<Menu>;

    #[cfg(any(feature = "v3_12", feature = "dox"))]
    fn get_use_popover(&self) -> bool;

    #[cfg(any(feature = "v3_6", feature = "dox"))]
    fn set_align_widget<'a, P: IsA<Widget> + 'a, Q: Into<Option<&'a P>>>(&self, align_widget: Q);

    #[cfg(any(feature = "v3_6", feature = "dox"))]
    fn set_direction(&self, direction: ArrowType);

    #[cfg(any(feature = "v3_6", feature = "dox"))]
    fn set_menu_model<'a, P: IsA<gio::MenuModel> + 'a, Q: Into<Option<&'a P>>>(&self, menu_model: Q);

    #[cfg(any(feature = "v3_12", feature = "dox"))]
    fn set_popover<'a, P: IsA<Widget> + 'a, Q: Into<Option<&'a P>>>(&self, popover: Q);

    #[cfg(any(feature = "v3_6", feature = "dox"))]
    fn set_popup<'a, P: IsA<Widget> + 'a, Q: Into<Option<&'a P>>>(&self, menu: Q);

    #[cfg(any(feature = "v3_12", feature = "dox"))]
    fn set_use_popover(&self, use_popover: bool);

    #[cfg(any(feature = "v3_6", feature = "dox"))]
    fn connect_property_align_widget_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v3_6", feature = "dox"))]
    fn connect_property_direction_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v3_6", feature = "dox"))]
    fn connect_property_menu_model_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v3_12", feature = "dox"))]
    fn connect_property_popover_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v3_6", feature = "dox"))]
    fn connect_property_popup_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v3_12", feature = "dox"))]
    fn connect_property_use_popover_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<MenuButton> + IsA<glib::object::Object>> MenuButtonExt for O {
    #[cfg(any(feature = "v3_6", feature = "dox"))]
    fn get_align_widget(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_menu_button_get_align_widget(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v3_6", feature = "dox"))]
    fn get_direction(&self) -> ArrowType {
        unsafe {
            from_glib(ffi::gtk_menu_button_get_direction(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v3_6", feature = "dox"))]
    fn get_menu_model(&self) -> Option<gio::MenuModel> {
        unsafe {
            from_glib_none(ffi::gtk_menu_button_get_menu_model(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v3_12", feature = "dox"))]
    fn get_popover(&self) -> Option<Popover> {
        unsafe {
            from_glib_none(ffi::gtk_menu_button_get_popover(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v3_6", feature = "dox"))]
    fn get_popup(&self) -> Option<Menu> {
        unsafe {
            from_glib_none(ffi::gtk_menu_button_get_popup(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v3_12", feature = "dox"))]
    fn get_use_popover(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_menu_button_get_use_popover(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v3_6", feature = "dox"))]
    fn set_align_widget<'a, P: IsA<Widget> + 'a, Q: Into<Option<&'a P>>>(&self, align_widget: Q) {
        let align_widget = align_widget.into();
        let align_widget = align_widget.to_glib_none();
        unsafe {
            ffi::gtk_menu_button_set_align_widget(self.to_glib_none().0, align_widget.0);
        }
    }

    #[cfg(any(feature = "v3_6", feature = "dox"))]
    fn set_direction(&self, direction: ArrowType) {
        unsafe {
            ffi::gtk_menu_button_set_direction(self.to_glib_none().0, direction.to_glib());
        }
    }

    #[cfg(any(feature = "v3_6", feature = "dox"))]
    fn set_menu_model<'a, P: IsA<gio::MenuModel> + 'a, Q: Into<Option<&'a P>>>(&self, menu_model: Q) {
        let menu_model = menu_model.into();
        let menu_model = menu_model.to_glib_none();
        unsafe {
            ffi::gtk_menu_button_set_menu_model(self.to_glib_none().0, menu_model.0);
        }
    }

    #[cfg(any(feature = "v3_12", feature = "dox"))]
    fn set_popover<'a, P: IsA<Widget> + 'a, Q: Into<Option<&'a P>>>(&self, popover: Q) {
        let popover = popover.into();
        let popover = popover.to_glib_none();
        unsafe {
            ffi::gtk_menu_button_set_popover(self.to_glib_none().0, popover.0);
        }
    }

    #[cfg(any(feature = "v3_6", feature = "dox"))]
    fn set_popup<'a, P: IsA<Widget> + 'a, Q: Into<Option<&'a P>>>(&self, menu: Q) {
        let menu = menu.into();
        let menu = menu.to_glib_none();
        unsafe {
            ffi::gtk_menu_button_set_popup(self.to_glib_none().0, menu.0);
        }
    }

    #[cfg(any(feature = "v3_12", feature = "dox"))]
    fn set_use_popover(&self, use_popover: bool) {
        unsafe {
            ffi::gtk_menu_button_set_use_popover(self.to_glib_none().0, use_popover.to_glib());
        }
    }

    #[cfg(any(feature = "v3_6", feature = "dox"))]
    fn connect_property_align_widget_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::align-widget",
                transmute(notify_align_widget_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(any(feature = "v3_6", feature = "dox"))]
    fn connect_property_direction_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::direction",
                transmute(notify_direction_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(any(feature = "v3_6", feature = "dox"))]
    fn connect_property_menu_model_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::menu-model",
                transmute(notify_menu_model_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(any(feature = "v3_12", feature = "dox"))]
    fn connect_property_popover_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::popover",
                transmute(notify_popover_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(any(feature = "v3_6", feature = "dox"))]
    fn connect_property_popup_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::popup",
                transmute(notify_popup_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(any(feature = "v3_12", feature = "dox"))]
    fn connect_property_use_popover_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::use-popover",
                transmute(notify_use_popover_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

#[cfg(any(feature = "v3_6", feature = "dox"))]
unsafe extern "C" fn notify_align_widget_trampoline<P>(this: *mut ffi::GtkMenuButton, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<MenuButton> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&MenuButton::from_glib_borrow(this).downcast_unchecked())
}

#[cfg(any(feature = "v3_6", feature = "dox"))]
unsafe extern "C" fn notify_direction_trampoline<P>(this: *mut ffi::GtkMenuButton, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<MenuButton> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&MenuButton::from_glib_borrow(this).downcast_unchecked())
}

#[cfg(any(feature = "v3_6", feature = "dox"))]
unsafe extern "C" fn notify_menu_model_trampoline<P>(this: *mut ffi::GtkMenuButton, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<MenuButton> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&MenuButton::from_glib_borrow(this).downcast_unchecked())
}

#[cfg(any(feature = "v3_12", feature = "dox"))]
unsafe extern "C" fn notify_popover_trampoline<P>(this: *mut ffi::GtkMenuButton, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<MenuButton> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&MenuButton::from_glib_borrow(this).downcast_unchecked())
}

#[cfg(any(feature = "v3_6", feature = "dox"))]
unsafe extern "C" fn notify_popup_trampoline<P>(this: *mut ffi::GtkMenuButton, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<MenuButton> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&MenuButton::from_glib_borrow(this).downcast_unchecked())
}

#[cfg(any(feature = "v3_12", feature = "dox"))]
unsafe extern "C" fn notify_use_popover_trampoline<P>(this: *mut ffi::GtkMenuButton, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<MenuButton> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&MenuButton::from_glib_borrow(this).downcast_unchecked())
}
