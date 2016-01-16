// This file was generated by gir (463de47) from gir-files (11e0e6d)
// DO NOT EDIT

use Adjustment;
use Buildable;
use Orientable;
use Orientation;
use Range;
use Widget;
use ffi;
use glib::object::Downcast;
use glib::translate::*;

glib_wrapper! {
    pub struct Scrollbar(Object<ffi::GtkScrollbar>): Widget, Range, Buildable, Orientable;

    match fn {
        get_type => || ffi::gtk_scrollbar_get_type(),
    }
}

impl Scrollbar {
    pub fn new(orientation: Orientation, adjustment: Option<&Adjustment>) -> Scrollbar {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_scrollbar_new(orientation, adjustment.to_glib_none().0)).downcast_unchecked()
        }
    }
}
