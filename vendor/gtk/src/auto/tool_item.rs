// This file was generated by gir (d933f9a) from gir-files (469db10)
// DO NOT EDIT

use Bin;
use Buildable;
use Container;
use Orientation;
use ReliefStyle;
use SizeGroup;
use ToolbarStyle;
use Widget;
use ffi;
use glib;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use pango;
use signal::Inhibit;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct ToolItem(Object<ffi::GtkToolItem, ffi::GtkToolItemClass>): Bin, Container, Widget, Buildable;

    match fn {
        get_type => || ffi::gtk_tool_item_get_type(),
    }
}

impl ToolItem {
    pub fn new() -> ToolItem {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(ffi::gtk_tool_item_new())
        }
    }
}

impl Default for ToolItem {
    fn default() -> Self {
        Self::new()
    }
}

pub trait ToolItemExt {
    fn get_ellipsize_mode(&self) -> pango::EllipsizeMode;

    fn get_expand(&self) -> bool;

    fn get_homogeneous(&self) -> bool;

    fn get_icon_size(&self) -> i32;

    fn get_is_important(&self) -> bool;

    fn get_orientation(&self) -> Orientation;

    fn get_proxy_menu_item(&self, menu_item_id: &str) -> Option<Widget>;

    fn get_relief_style(&self) -> ReliefStyle;

    fn get_text_alignment(&self) -> f32;

    fn get_text_orientation(&self) -> Orientation;

    fn get_text_size_group(&self) -> Option<SizeGroup>;

    fn get_toolbar_style(&self) -> ToolbarStyle;

    fn get_use_drag_window(&self) -> bool;

    fn get_visible_horizontal(&self) -> bool;

    fn get_visible_vertical(&self) -> bool;

    fn rebuild_menu(&self);

    fn retrieve_proxy_menu_item(&self) -> Option<Widget>;

    fn set_expand(&self, expand: bool);

    fn set_homogeneous(&self, homogeneous: bool);

    fn set_is_important(&self, is_important: bool);

    fn set_proxy_menu_item<P: IsA<Widget>>(&self, menu_item_id: &str, menu_item: &P);

    fn set_use_drag_window(&self, use_drag_window: bool);

    fn set_visible_horizontal(&self, visible_horizontal: bool);

    fn set_visible_vertical(&self, visible_vertical: bool);

    fn toolbar_reconfigured(&self);

    fn connect_create_menu_proxy<F: Fn(&Self) -> Inhibit + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_toolbar_reconfigured<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_is_important_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_visible_horizontal_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_visible_vertical_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<ToolItem> + IsA<glib::object::Object>> ToolItemExt for O {
    fn get_ellipsize_mode(&self) -> pango::EllipsizeMode {
        unsafe {
            from_glib(ffi::gtk_tool_item_get_ellipsize_mode(self.to_glib_none().0))
        }
    }

    fn get_expand(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_tool_item_get_expand(self.to_glib_none().0))
        }
    }

    fn get_homogeneous(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_tool_item_get_homogeneous(self.to_glib_none().0))
        }
    }

    fn get_icon_size(&self) -> i32 {
        unsafe {
            ffi::gtk_tool_item_get_icon_size(self.to_glib_none().0)
        }
    }

    fn get_is_important(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_tool_item_get_is_important(self.to_glib_none().0))
        }
    }

    fn get_orientation(&self) -> Orientation {
        unsafe {
            from_glib(ffi::gtk_tool_item_get_orientation(self.to_glib_none().0))
        }
    }

    fn get_proxy_menu_item(&self, menu_item_id: &str) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_tool_item_get_proxy_menu_item(self.to_glib_none().0, menu_item_id.to_glib_none().0))
        }
    }

    fn get_relief_style(&self) -> ReliefStyle {
        unsafe {
            from_glib(ffi::gtk_tool_item_get_relief_style(self.to_glib_none().0))
        }
    }

    fn get_text_alignment(&self) -> f32 {
        unsafe {
            ffi::gtk_tool_item_get_text_alignment(self.to_glib_none().0)
        }
    }

    fn get_text_orientation(&self) -> Orientation {
        unsafe {
            from_glib(ffi::gtk_tool_item_get_text_orientation(self.to_glib_none().0))
        }
    }

    fn get_text_size_group(&self) -> Option<SizeGroup> {
        unsafe {
            from_glib_none(ffi::gtk_tool_item_get_text_size_group(self.to_glib_none().0))
        }
    }

    fn get_toolbar_style(&self) -> ToolbarStyle {
        unsafe {
            from_glib(ffi::gtk_tool_item_get_toolbar_style(self.to_glib_none().0))
        }
    }

    fn get_use_drag_window(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_tool_item_get_use_drag_window(self.to_glib_none().0))
        }
    }

    fn get_visible_horizontal(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_tool_item_get_visible_horizontal(self.to_glib_none().0))
        }
    }

    fn get_visible_vertical(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_tool_item_get_visible_vertical(self.to_glib_none().0))
        }
    }

    fn rebuild_menu(&self) {
        unsafe {
            ffi::gtk_tool_item_rebuild_menu(self.to_glib_none().0);
        }
    }

    fn retrieve_proxy_menu_item(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_tool_item_retrieve_proxy_menu_item(self.to_glib_none().0))
        }
    }

    fn set_expand(&self, expand: bool) {
        unsafe {
            ffi::gtk_tool_item_set_expand(self.to_glib_none().0, expand.to_glib());
        }
    }

    fn set_homogeneous(&self, homogeneous: bool) {
        unsafe {
            ffi::gtk_tool_item_set_homogeneous(self.to_glib_none().0, homogeneous.to_glib());
        }
    }

    fn set_is_important(&self, is_important: bool) {
        unsafe {
            ffi::gtk_tool_item_set_is_important(self.to_glib_none().0, is_important.to_glib());
        }
    }

    fn set_proxy_menu_item<P: IsA<Widget>>(&self, menu_item_id: &str, menu_item: &P) {
        unsafe {
            ffi::gtk_tool_item_set_proxy_menu_item(self.to_glib_none().0, menu_item_id.to_glib_none().0, menu_item.to_glib_none().0);
        }
    }

    fn set_use_drag_window(&self, use_drag_window: bool) {
        unsafe {
            ffi::gtk_tool_item_set_use_drag_window(self.to_glib_none().0, use_drag_window.to_glib());
        }
    }

    fn set_visible_horizontal(&self, visible_horizontal: bool) {
        unsafe {
            ffi::gtk_tool_item_set_visible_horizontal(self.to_glib_none().0, visible_horizontal.to_glib());
        }
    }

    fn set_visible_vertical(&self, visible_vertical: bool) {
        unsafe {
            ffi::gtk_tool_item_set_visible_vertical(self.to_glib_none().0, visible_vertical.to_glib());
        }
    }

    fn toolbar_reconfigured(&self) {
        unsafe {
            ffi::gtk_tool_item_toolbar_reconfigured(self.to_glib_none().0);
        }
    }

    fn connect_create_menu_proxy<F: Fn(&Self) -> Inhibit + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) -> Inhibit + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "create-menu-proxy",
                transmute(create_menu_proxy_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_toolbar_reconfigured<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "toolbar-reconfigured",
                transmute(toolbar_reconfigured_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_is_important_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::is-important",
                transmute(notify_is_important_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_visible_horizontal_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::visible-horizontal",
                transmute(notify_visible_horizontal_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_visible_vertical_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::visible-vertical",
                transmute(notify_visible_vertical_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn create_menu_proxy_trampoline<P>(this: *mut ffi::GtkToolItem, f: glib_ffi::gpointer) -> glib_ffi::gboolean
where P: IsA<ToolItem> {
    callback_guard!();
    let f: &&(Fn(&P) -> Inhibit + 'static) = transmute(f);
    f(&ToolItem::from_glib_borrow(this).downcast_unchecked()).to_glib()
}

unsafe extern "C" fn toolbar_reconfigured_trampoline<P>(this: *mut ffi::GtkToolItem, f: glib_ffi::gpointer)
where P: IsA<ToolItem> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&ToolItem::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_is_important_trampoline<P>(this: *mut ffi::GtkToolItem, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<ToolItem> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&ToolItem::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_visible_horizontal_trampoline<P>(this: *mut ffi::GtkToolItem, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<ToolItem> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&ToolItem::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_visible_vertical_trampoline<P>(this: *mut ffi::GtkToolItem, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<ToolItem> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&ToolItem::from_glib_borrow(this).downcast_unchecked())
}
