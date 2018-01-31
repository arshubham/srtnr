// Copyright 2013-2017, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use ffi;
use glib_ffi;
use glib::translate::*;

use Container;
use Fixed;
use IsA;
use Value;
use Widget;

// All this is in order to avoid the segfault. More info in :
// https://github.com/gtk-rs/gtk/issues/565
fn has_widget<O: IsA<Fixed> + IsA<Container>, T: IsA<Widget>>(c: &O, item: &T) -> bool {
    skip_assert_initialized!();
    unsafe {
        let glist = ffi::gtk_container_get_children(c.to_glib_none().0);
        let found = !glib_ffi::g_list_find(glist, item.to_glib_none().0 as _).is_null();
        glib_ffi::g_list_free(glist);
        found
    }
}

pub trait FixedExtManual {
    fn get_child_x<T: IsA<Widget>>(&self, item: &T) -> i32;

    fn set_child_x<T: IsA<Widget>>(&self, item: &T, x: i32);

    fn get_child_y<T: IsA<Widget>>(&self, item: &T) -> i32;

    fn set_child_y<T: IsA<Widget>>(&self, item: &T, y: i32);
}

impl<O: IsA<Fixed> + IsA<Container>> FixedExtManual for O {
    fn get_child_x<T: IsA<Widget>>(&self, item: &T) -> i32 {
        assert!(has_widget(self, item), "this item isn't in the Fixed's widget list");
        let mut value = Value::from(&0);
        unsafe {
            ffi::gtk_container_child_get_property(self.to_glib_none().0, item.to_glib_none().0, "x".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    fn set_child_x<T: IsA<Widget>>(&self, item: &T, x: i32) {
        assert!(has_widget(self, item), "this item isn't in the Fixed's widget list");
        unsafe {
            ffi::gtk_container_child_set_property(self.to_glib_none().0, item.to_glib_none().0, "x".to_glib_none().0, Value::from(&x).to_glib_none().0);
        }
    }

    fn get_child_y<T: IsA<Widget>>(&self, item: &T) -> i32 {
        assert!(has_widget(self, item), "this item isn't in the Fixed's widget list");
        let mut value = Value::from(&0);
        unsafe {
            ffi::gtk_container_child_get_property(self.to_glib_none().0, item.to_glib_none().0, "y".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    fn set_child_y<T: IsA<Widget>>(&self, item: &T, y: i32) {
        assert!(has_widget(self, item), "this item isn't in the Fixed's widget list");
        unsafe {
            ffi::gtk_container_child_set_property(self.to_glib_none().0, item.to_glib_none().0, "y".to_glib_none().0, Value::from(&y).to_glib_none().0);
        }
    }
}
