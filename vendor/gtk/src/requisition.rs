use ffi;
use glib::translate::*;
use std::mem;

#[repr(C)]
pub struct Requisition {
    pub width: i32,
    pub height: i32,
}

#[doc(hidden)]
impl Uninitialized for Requisition {
    #[inline]
    unsafe fn uninitialized() -> Self {
        mem::uninitialized()
    }
}

#[doc(hidden)]
impl<'a> ToGlibPtr<'a, *const ffi::GtkRequisition> for Requisition {
    type Storage = &'a Self;

    #[inline]
    fn to_glib_none(&'a self) -> Stash<'a, *const ffi::GtkRequisition, Self> {
        let ptr: *const Requisition = &*self;
        Stash(ptr as *const ffi::GtkRequisition, self)
    }
}

#[doc(hidden)]
impl<'a> ToGlibPtrMut<'a, *mut ffi::GtkRequisition> for Requisition {
    type Storage = &'a mut Self;

    #[inline]
    fn to_glib_none_mut(&'a mut self) -> StashMut<'a, *mut ffi::GtkRequisition, Self> {
        let ptr: *mut Requisition = &mut *self;
        StashMut(ptr as *mut ffi::GtkRequisition, self)
    }
}
