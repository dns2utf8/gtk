// This file was generated by gir (9261d77) from gir-files (469db10)
// DO NOT EDIT

use Box;
use Buildable;
use Container;
use FontChooser;
use Orientable;
use Widget;
use ffi;
use glib::object::Downcast;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct FontChooserWidget(Object<ffi::GtkFontChooserWidget, ffi::GtkFontChooserWidgetClass>): Box, Container, Widget, Buildable, Orientable, FontChooser;

    match fn {
        get_type => || ffi::gtk_font_chooser_widget_get_type(),
    }
}

impl FontChooserWidget {
    pub fn new() -> FontChooserWidget {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_font_chooser_widget_new()).downcast_unchecked()
        }
    }
}

impl Default for FontChooserWidget {
    fn default() -> Self {
        Self::new()
    }
}
