// This file was generated by gir (d933f9a) from gir-files (469db10)
// DO NOT EDIT

use SelectionMode;
use TreeIter;
use TreeModel;
use TreePath;
use TreeView;
use ffi;
use glib;
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
    pub struct TreeSelection(Object<ffi::GtkTreeSelection, ffi::GtkTreeSelectionClass>);

    match fn {
        get_type => || ffi::gtk_tree_selection_get_type(),
    }
}

pub trait TreeSelectionExt {
    fn count_selected_rows(&self) -> i32;

    fn get_mode(&self) -> SelectionMode;

    //fn get_select_function(&self) -> /*Unknown conversion*//*Unimplemented*/TreeSelectionFunc;

    fn get_selected(&self) -> Option<(TreeModel, TreeIter)>;

    fn get_selected_rows(&self) -> (Vec<TreePath>, TreeModel);

    fn get_tree_view(&self) -> Option<TreeView>;

    //fn get_user_data(&self) -> /*Unimplemented*/Option<Fundamental: Pointer>;

    fn iter_is_selected(&self, iter: &TreeIter) -> bool;

    fn path_is_selected(&self, path: &TreePath) -> bool;

    fn select_all(&self);

    fn select_iter(&self, iter: &TreeIter);

    fn select_path(&self, path: &TreePath);

    fn select_range(&self, start_path: &TreePath, end_path: &TreePath);

    //fn selected_foreach<P: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, func: /*Unknown conversion*//*Unimplemented*/TreeSelectionForeachFunc, data: P);

    fn set_mode(&self, type_: SelectionMode);

    //fn set_select_function<P: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, func: /*Unknown conversion*//*Unimplemented*/TreeSelectionFunc, data: P, destroy: /*Unknown conversion*//*Unimplemented*/DestroyNotify);

    fn unselect_all(&self);

    fn unselect_iter(&self, iter: &TreeIter);

    fn unselect_path(&self, path: &TreePath);

    fn unselect_range(&self, start_path: &TreePath, end_path: &TreePath);

    fn connect_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_mode_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<TreeSelection> + IsA<glib::object::Object>> TreeSelectionExt for O {
    fn count_selected_rows(&self) -> i32 {
        unsafe {
            ffi::gtk_tree_selection_count_selected_rows(self.to_glib_none().0)
        }
    }

    fn get_mode(&self) -> SelectionMode {
        unsafe {
            from_glib(ffi::gtk_tree_selection_get_mode(self.to_glib_none().0))
        }
    }

    //fn get_select_function(&self) -> /*Unknown conversion*//*Unimplemented*/TreeSelectionFunc {
    //    unsafe { TODO: call ffi::gtk_tree_selection_get_select_function() }
    //}

    fn get_selected(&self) -> Option<(TreeModel, TreeIter)> {
        unsafe {
            let mut model = ptr::null_mut();
            let mut iter = TreeIter::uninitialized();
            let ret = from_glib(ffi::gtk_tree_selection_get_selected(self.to_glib_none().0, &mut model, iter.to_glib_none_mut().0));
            if ret { Some((from_glib_none(model), iter)) } else { None }
        }
    }

    fn get_selected_rows(&self) -> (Vec<TreePath>, TreeModel) {
        unsafe {
            let mut model = ptr::null_mut();
            let ret = FromGlibPtrContainer::from_glib_full(ffi::gtk_tree_selection_get_selected_rows(self.to_glib_none().0, &mut model));
            (ret, from_glib_none(model))
        }
    }

    fn get_tree_view(&self) -> Option<TreeView> {
        unsafe {
            from_glib_none(ffi::gtk_tree_selection_get_tree_view(self.to_glib_none().0))
        }
    }

    //fn get_user_data(&self) -> /*Unimplemented*/Option<Fundamental: Pointer> {
    //    unsafe { TODO: call ffi::gtk_tree_selection_get_user_data() }
    //}

    fn iter_is_selected(&self, iter: &TreeIter) -> bool {
        unsafe {
            from_glib(ffi::gtk_tree_selection_iter_is_selected(self.to_glib_none().0, mut_override(iter.to_glib_none().0)))
        }
    }

    fn path_is_selected(&self, path: &TreePath) -> bool {
        unsafe {
            from_glib(ffi::gtk_tree_selection_path_is_selected(self.to_glib_none().0, mut_override(path.to_glib_none().0)))
        }
    }

    fn select_all(&self) {
        unsafe {
            ffi::gtk_tree_selection_select_all(self.to_glib_none().0);
        }
    }

    fn select_iter(&self, iter: &TreeIter) {
        unsafe {
            ffi::gtk_tree_selection_select_iter(self.to_glib_none().0, mut_override(iter.to_glib_none().0));
        }
    }

    fn select_path(&self, path: &TreePath) {
        unsafe {
            ffi::gtk_tree_selection_select_path(self.to_glib_none().0, mut_override(path.to_glib_none().0));
        }
    }

    fn select_range(&self, start_path: &TreePath, end_path: &TreePath) {
        unsafe {
            ffi::gtk_tree_selection_select_range(self.to_glib_none().0, mut_override(start_path.to_glib_none().0), mut_override(end_path.to_glib_none().0));
        }
    }

    //fn selected_foreach<P: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, func: /*Unknown conversion*//*Unimplemented*/TreeSelectionForeachFunc, data: P) {
    //    unsafe { TODO: call ffi::gtk_tree_selection_selected_foreach() }
    //}

    fn set_mode(&self, type_: SelectionMode) {
        unsafe {
            ffi::gtk_tree_selection_set_mode(self.to_glib_none().0, type_.to_glib());
        }
    }

    //fn set_select_function<P: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, func: /*Unknown conversion*//*Unimplemented*/TreeSelectionFunc, data: P, destroy: /*Unknown conversion*//*Unimplemented*/DestroyNotify) {
    //    unsafe { TODO: call ffi::gtk_tree_selection_set_select_function() }
    //}

    fn unselect_all(&self) {
        unsafe {
            ffi::gtk_tree_selection_unselect_all(self.to_glib_none().0);
        }
    }

    fn unselect_iter(&self, iter: &TreeIter) {
        unsafe {
            ffi::gtk_tree_selection_unselect_iter(self.to_glib_none().0, mut_override(iter.to_glib_none().0));
        }
    }

    fn unselect_path(&self, path: &TreePath) {
        unsafe {
            ffi::gtk_tree_selection_unselect_path(self.to_glib_none().0, mut_override(path.to_glib_none().0));
        }
    }

    fn unselect_range(&self, start_path: &TreePath, end_path: &TreePath) {
        unsafe {
            ffi::gtk_tree_selection_unselect_range(self.to_glib_none().0, mut_override(start_path.to_glib_none().0), mut_override(end_path.to_glib_none().0));
        }
    }

    fn connect_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "changed",
                transmute(changed_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_mode_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::mode",
                transmute(notify_mode_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn changed_trampoline<P>(this: *mut ffi::GtkTreeSelection, f: glib_ffi::gpointer)
where P: IsA<TreeSelection> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&TreeSelection::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_mode_trampoline<P>(this: *mut ffi::GtkTreeSelection, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<TreeSelection> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&TreeSelection::from_glib_borrow(this).downcast_unchecked())
}