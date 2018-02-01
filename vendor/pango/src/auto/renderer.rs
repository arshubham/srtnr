// This file was generated by gir (0409d73) from gir-files (469db10)
// DO NOT EDIT

use Color;
use Font;
use Glyph;
use GlyphItem;
use GlyphString;
use Layout;
use LayoutLine;
use Matrix;
use RenderPart;
use ffi;
use glib::object::IsA;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct Renderer(Object<ffi::PangoRenderer, ffi::PangoRendererClass>);

    match fn {
        get_type => || ffi::pango_renderer_get_type(),
    }
}

pub trait RendererExt {
    fn activate(&self);

    fn deactivate(&self);

    fn draw_error_underline(&self, x: i32, y: i32, width: i32, height: i32);

    fn draw_glyph(&self, font: &Font, glyph: Glyph, x: f64, y: f64);

    fn draw_glyph_item<'a, P: Into<Option<&'a str>>>(&self, text: P, glyph_item: &mut GlyphItem, x: i32, y: i32);

    fn draw_glyphs(&self, font: &Font, glyphs: &mut GlyphString, x: i32, y: i32);

    fn draw_layout(&self, layout: &Layout, x: i32, y: i32);

    fn draw_layout_line(&self, line: &LayoutLine, x: i32, y: i32);

    fn draw_rectangle(&self, part: RenderPart, x: i32, y: i32, width: i32, height: i32);

    fn draw_trapezoid(&self, part: RenderPart, y1_: f64, x11: f64, x21: f64, y2: f64, x12: f64, x22: f64);

    #[cfg(any(feature = "v1_38", feature = "dox"))]
    fn get_alpha(&self, part: RenderPart) -> u16;

    fn get_color(&self, part: RenderPart) -> Option<Color>;

    fn get_layout(&self) -> Option<Layout>;

    fn get_layout_line(&self) -> Option<LayoutLine>;

    fn get_matrix(&self) -> Option<Matrix>;

    fn part_changed(&self, part: RenderPart);

    #[cfg(any(feature = "v1_38", feature = "dox"))]
    fn set_alpha(&self, part: RenderPart, alpha: u16);

    fn set_color<'a, P: Into<Option<&'a Color>>>(&self, part: RenderPart, color: P);

    fn set_matrix<'a, P: Into<Option<&'a Matrix>>>(&self, matrix: P);
}

impl<O: IsA<Renderer>> RendererExt for O {
    fn activate(&self) {
        unsafe {
            ffi::pango_renderer_activate(self.to_glib_none().0);
        }
    }

    fn deactivate(&self) {
        unsafe {
            ffi::pango_renderer_deactivate(self.to_glib_none().0);
        }
    }

    fn draw_error_underline(&self, x: i32, y: i32, width: i32, height: i32) {
        unsafe {
            ffi::pango_renderer_draw_error_underline(self.to_glib_none().0, x, y, width, height);
        }
    }

    fn draw_glyph(&self, font: &Font, glyph: Glyph, x: f64, y: f64) {
        unsafe {
            ffi::pango_renderer_draw_glyph(self.to_glib_none().0, font.to_glib_none().0, glyph, x, y);
        }
    }

    fn draw_glyph_item<'a, P: Into<Option<&'a str>>>(&self, text: P, glyph_item: &mut GlyphItem, x: i32, y: i32) {
        let text = text.into();
        let text = text.to_glib_none();
        unsafe {
            ffi::pango_renderer_draw_glyph_item(self.to_glib_none().0, text.0, glyph_item.to_glib_none_mut().0, x, y);
        }
    }

    fn draw_glyphs(&self, font: &Font, glyphs: &mut GlyphString, x: i32, y: i32) {
        unsafe {
            ffi::pango_renderer_draw_glyphs(self.to_glib_none().0, font.to_glib_none().0, glyphs.to_glib_none_mut().0, x, y);
        }
    }

    fn draw_layout(&self, layout: &Layout, x: i32, y: i32) {
        unsafe {
            ffi::pango_renderer_draw_layout(self.to_glib_none().0, layout.to_glib_none().0, x, y);
        }
    }

    fn draw_layout_line(&self, line: &LayoutLine, x: i32, y: i32) {
        unsafe {
            ffi::pango_renderer_draw_layout_line(self.to_glib_none().0, line.to_glib_none().0, x, y);
        }
    }

    fn draw_rectangle(&self, part: RenderPart, x: i32, y: i32, width: i32, height: i32) {
        unsafe {
            ffi::pango_renderer_draw_rectangle(self.to_glib_none().0, part.to_glib(), x, y, width, height);
        }
    }

    fn draw_trapezoid(&self, part: RenderPart, y1_: f64, x11: f64, x21: f64, y2: f64, x12: f64, x22: f64) {
        unsafe {
            ffi::pango_renderer_draw_trapezoid(self.to_glib_none().0, part.to_glib(), y1_, x11, x21, y2, x12, x22);
        }
    }

    #[cfg(any(feature = "v1_38", feature = "dox"))]
    fn get_alpha(&self, part: RenderPart) -> u16 {
        unsafe {
            ffi::pango_renderer_get_alpha(self.to_glib_none().0, part.to_glib())
        }
    }

    fn get_color(&self, part: RenderPart) -> Option<Color> {
        unsafe {
            from_glib_none(ffi::pango_renderer_get_color(self.to_glib_none().0, part.to_glib()))
        }
    }

    fn get_layout(&self) -> Option<Layout> {
        unsafe {
            from_glib_none(ffi::pango_renderer_get_layout(self.to_glib_none().0))
        }
    }

    fn get_layout_line(&self) -> Option<LayoutLine> {
        unsafe {
            from_glib_none(ffi::pango_renderer_get_layout_line(self.to_glib_none().0))
        }
    }

    fn get_matrix(&self) -> Option<Matrix> {
        unsafe {
            from_glib_none(ffi::pango_renderer_get_matrix(self.to_glib_none().0))
        }
    }

    fn part_changed(&self, part: RenderPart) {
        unsafe {
            ffi::pango_renderer_part_changed(self.to_glib_none().0, part.to_glib());
        }
    }

    #[cfg(any(feature = "v1_38", feature = "dox"))]
    fn set_alpha(&self, part: RenderPart, alpha: u16) {
        unsafe {
            ffi::pango_renderer_set_alpha(self.to_glib_none().0, part.to_glib(), alpha);
        }
    }

    fn set_color<'a, P: Into<Option<&'a Color>>>(&self, part: RenderPart, color: P) {
        let color = color.into();
        let color = color.to_glib_none();
        unsafe {
            ffi::pango_renderer_set_color(self.to_glib_none().0, part.to_glib(), color.0);
        }
    }

    fn set_matrix<'a, P: Into<Option<&'a Matrix>>>(&self, matrix: P) {
        let matrix = matrix.into();
        let matrix = matrix.to_glib_none();
        unsafe {
            ffi::pango_renderer_set_matrix(self.to_glib_none().0, matrix.0);
        }
    }
}