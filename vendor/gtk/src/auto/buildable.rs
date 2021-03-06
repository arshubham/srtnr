// This file was generated by gir (d933f9a) from gir-files (469db10)
// DO NOT EDIT

use Builder;
use ffi;
use glib;
use glib::object::IsA;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct Buildable(Object<ffi::GtkBuildable, ffi::GtkBuildableIface>);

    match fn {
        get_type => || ffi::gtk_buildable_get_type(),
    }
}

pub trait BuildableExt {
    fn add_child<'a, P: IsA<glib::Object>, Q: Into<Option<&'a str>>>(&self, builder: &Builder, child: &P, type_: Q);

    fn construct_child(&self, builder: &Builder, name: &str) -> Option<glib::Object>;

    //fn custom_finished<'a, P: IsA<glib::Object> + 'a, Q: Into<Option<&'a P>>, R: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, builder: &Builder, child: Q, tagname: &str, data: R);

    //fn custom_tag_end<'a, P: IsA<glib::Object> + 'a, Q: Into<Option<&'a P>>, R: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, builder: &Builder, child: Q, tagname: &str, data: R);

    //fn custom_tag_start<'a, P: IsA<glib::Object> + 'a, Q: Into<Option<&'a P>>>(&self, builder: &Builder, child: Q, tagname: &str, parser: /*Ignored*/glib::MarkupParser, data: /*Unimplemented*/&mut Option<Fundamental: Pointer>) -> bool;

    fn get_internal_child(&self, builder: &Builder, childname: &str) -> Option<glib::Object>;

    fn parser_finished(&self, builder: &Builder);

    fn set_buildable_property(&self, builder: &Builder, name: &str, value: &glib::Value);
}

impl<O: IsA<Buildable>> BuildableExt for O {
    fn add_child<'a, P: IsA<glib::Object>, Q: Into<Option<&'a str>>>(&self, builder: &Builder, child: &P, type_: Q) {
        let type_ = type_.into();
        let type_ = type_.to_glib_none();
        unsafe {
            ffi::gtk_buildable_add_child(self.to_glib_none().0, builder.to_glib_none().0, child.to_glib_none().0, type_.0);
        }
    }

    fn construct_child(&self, builder: &Builder, name: &str) -> Option<glib::Object> {
        unsafe {
            from_glib_full(ffi::gtk_buildable_construct_child(self.to_glib_none().0, builder.to_glib_none().0, name.to_glib_none().0))
        }
    }

    //fn custom_finished<'a, P: IsA<glib::Object> + 'a, Q: Into<Option<&'a P>>, R: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, builder: &Builder, child: Q, tagname: &str, data: R) {
    //    unsafe { TODO: call ffi::gtk_buildable_custom_finished() }
    //}

    //fn custom_tag_end<'a, P: IsA<glib::Object> + 'a, Q: Into<Option<&'a P>>, R: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, builder: &Builder, child: Q, tagname: &str, data: R) {
    //    unsafe { TODO: call ffi::gtk_buildable_custom_tag_end() }
    //}

    //fn custom_tag_start<'a, P: IsA<glib::Object> + 'a, Q: Into<Option<&'a P>>>(&self, builder: &Builder, child: Q, tagname: &str, parser: /*Ignored*/glib::MarkupParser, data: /*Unimplemented*/&mut Option<Fundamental: Pointer>) -> bool {
    //    unsafe { TODO: call ffi::gtk_buildable_custom_tag_start() }
    //}

    fn get_internal_child(&self, builder: &Builder, childname: &str) -> Option<glib::Object> {
        unsafe {
            from_glib_none(ffi::gtk_buildable_get_internal_child(self.to_glib_none().0, builder.to_glib_none().0, childname.to_glib_none().0))
        }
    }

    fn parser_finished(&self, builder: &Builder) {
        unsafe {
            ffi::gtk_buildable_parser_finished(self.to_glib_none().0, builder.to_glib_none().0);
        }
    }

    fn set_buildable_property(&self, builder: &Builder, name: &str, value: &glib::Value) {
        unsafe {
            ffi::gtk_buildable_set_buildable_property(self.to_glib_none().0, builder.to_glib_none().0, name.to_glib_none().0, value.to_glib_none().0);
        }
    }
}
