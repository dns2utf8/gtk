// This file was generated by gir (7dd2bcd) from gir-files (11e0e6d)
// DO NOT EDIT

use Buildable;
use ImageType;
use Misc;
use Widget;
use ffi;
use gdk_pixbuf;
use glib::object::Downcast;
use glib::object::IsA;
use glib::translate::*;
use std;

glib_wrapper! {
    pub struct Image(Object<ffi::GtkImage>): Misc, Widget, Buildable;

    match fn {
        get_type => || ffi::gtk_image_get_type(),
    }
}

impl Image {
    pub fn new() -> Image {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_image_new()).downcast_unchecked()
        }
    }

    pub fn new_from_animation<T: IsA<gdk_pixbuf::PixbufAnimation>>(animation: &T) -> Image {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_image_new_from_animation(animation.to_glib_none().0)).downcast_unchecked()
        }
    }

    pub fn new_from_file<T: AsRef<std::path::Path>>(filename: T) -> Image {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_image_new_from_file(filename.as_ref().to_glib_none().0)).downcast_unchecked()
        }
    }

    //pub fn new_from_gicon<T: IsA</*Ignored*/gio::Icon>>(icon: &T, size: i32) -> Image {
    //    unsafe { TODO: call ffi::gtk_image_new_from_gicon() }
    //}

    pub fn new_from_icon_name(icon_name: &str, size: i32) -> Image {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_image_new_from_icon_name(icon_name.to_glib_none().0, size)).downcast_unchecked()
        }
    }

    //pub fn new_from_icon_set(icon_set: /*Ignored*/&IconSet, size: i32) -> Image {
    //    unsafe { TODO: call ffi::gtk_image_new_from_icon_set() }
    //}

    pub fn new_from_pixbuf(pixbuf: Option<&gdk_pixbuf::Pixbuf>) -> Image {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_image_new_from_pixbuf(pixbuf.to_glib_none().0)).downcast_unchecked()
        }
    }

    pub fn new_from_resource(resource_path: &str) -> Image {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_image_new_from_resource(resource_path.to_glib_none().0)).downcast_unchecked()
        }
    }

    pub fn new_from_stock(stock_id: &str, size: i32) -> Image {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_image_new_from_stock(stock_id.to_glib_none().0, size)).downcast_unchecked()
        }
    }

    //#[cfg(feature = "3.10")]
    //pub fn new_from_surface(surface: /*Ignored*/Option<&mut cairo::Surface>) -> Image {
    //    unsafe { TODO: call ffi::gtk_image_new_from_surface() }
    //}

    pub fn clear(&self) {
        unsafe {
            ffi::gtk_image_clear(self.to_glib_none().0);
        }
    }

    pub fn get_animation(&self) -> Option<gdk_pixbuf::PixbufAnimation> {
        unsafe {
            from_glib_none(ffi::gtk_image_get_animation(self.to_glib_none().0))
        }
    }

    //pub fn get_gicon(&self, gicon: /*Ignored*/gio::Icon) -> i32 {
    //    unsafe { TODO: call ffi::gtk_image_get_gicon() }
    //}

    //pub fn get_icon_name(&self, icon_name: /*Unimplemented*/String) -> i32 {
    //    unsafe { TODO: call ffi::gtk_image_get_icon_name() }
    //}

    //pub fn get_icon_set(&self, icon_set: /*Ignored*/IconSet) -> i32 {
    //    unsafe { TODO: call ffi::gtk_image_get_icon_set() }
    //}

    pub fn get_pixbuf(&self) -> Option<gdk_pixbuf::Pixbuf> {
        unsafe {
            from_glib_none(ffi::gtk_image_get_pixbuf(self.to_glib_none().0))
        }
    }

    pub fn get_pixel_size(&self) -> i32 {
        unsafe {
            ffi::gtk_image_get_pixel_size(self.to_glib_none().0)
        }
    }

    //pub fn get_stock(&self, stock_id: /*Unimplemented*/String) -> i32 {
    //    unsafe { TODO: call ffi::gtk_image_get_stock() }
    //}

    pub fn get_storage_type(&self) -> ImageType {
        unsafe {
            ffi::gtk_image_get_storage_type(self.to_glib_none().0)
        }
    }

    pub fn set_from_animation<T: IsA<gdk_pixbuf::PixbufAnimation>>(&self, animation: &T) {
        unsafe {
            ffi::gtk_image_set_from_animation(self.to_glib_none().0, animation.to_glib_none().0);
        }
    }

    pub fn set_from_file<T: AsRef<std::path::Path>>(&self, filename: T) {
        unsafe {
            ffi::gtk_image_set_from_file(self.to_glib_none().0, filename.as_ref().to_glib_none().0);
        }
    }

    //pub fn set_from_gicon<T: IsA</*Ignored*/gio::Icon>>(&self, icon: &T, size: i32) {
    //    unsafe { TODO: call ffi::gtk_image_set_from_gicon() }
    //}

    pub fn set_from_icon_name(&self, icon_name: &str, size: i32) {
        unsafe {
            ffi::gtk_image_set_from_icon_name(self.to_glib_none().0, icon_name.to_glib_none().0, size);
        }
    }

    //pub fn set_from_icon_set(&self, icon_set: /*Ignored*/&IconSet, size: i32) {
    //    unsafe { TODO: call ffi::gtk_image_set_from_icon_set() }
    //}

    pub fn set_from_pixbuf(&self, pixbuf: Option<&gdk_pixbuf::Pixbuf>) {
        unsafe {
            ffi::gtk_image_set_from_pixbuf(self.to_glib_none().0, pixbuf.to_glib_none().0);
        }
    }

    pub fn set_from_resource(&self, resource_path: Option<&str>) {
        unsafe {
            ffi::gtk_image_set_from_resource(self.to_glib_none().0, resource_path.to_glib_none().0);
        }
    }

    pub fn set_from_stock(&self, stock_id: &str, size: i32) {
        unsafe {
            ffi::gtk_image_set_from_stock(self.to_glib_none().0, stock_id.to_glib_none().0, size);
        }
    }

    //#[cfg(feature = "3.10")]
    //pub fn set_from_surface(&self, surface: /*Ignored*/&mut cairo::Surface) {
    //    unsafe { TODO: call ffi::gtk_image_set_from_surface() }
    //}

    pub fn set_pixel_size(&self, pixel_size: i32) {
        unsafe {
            ffi::gtk_image_set_pixel_size(self.to_glib_none().0, pixel_size);
        }
    }
}
