// Copyright 2013-2016, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <https://opensource.org/licenses/MIT>

use glib::translate::*;
use gtk_sys;
use libc::{c_int, c_uint};

glib_wrapper! {
    pub struct EntryBuffer(Object<gtk_sys::GtkEntryBuffer, gtk_sys::GtkEntryBufferClass, EntryBufferClass>);

    match fn {
        get_type => || gtk_sys::gtk_entry_buffer_get_type(),
    }
}

macro_rules! to_u16 {
    ($e:expr) => (
        {
            let x = $e;
            assert!(x as usize <= u16::max_value() as usize,
                "Unexpected value exceeding `u16` range");
            x as u16
        }
    )
}

#[allow(clippy::cast_lossless)]
impl EntryBuffer {
    pub fn new(initial_chars: Option<&str>) -> EntryBuffer {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(gtk_sys::gtk_entry_buffer_new(
                initial_chars.to_glib_none().0,
                -1,
            ))
        }
    }

    pub fn delete_text(&self, position: u16, n_chars: Option<u16>) -> u16 {
        unsafe {
            to_u16!(gtk_sys::gtk_entry_buffer_delete_text(
                self.to_glib_none().0,
                position as c_uint,
                n_chars.map(|n| n as c_int).unwrap_or(-1)
            ))
        }
    }

    pub fn get_bytes(&self) -> u32 {
        unsafe { gtk_sys::gtk_entry_buffer_get_bytes(self.to_glib_none().0) as u32 }
    }

    pub fn get_length(&self) -> u16 {
        unsafe { to_u16!(gtk_sys::gtk_entry_buffer_get_length(self.to_glib_none().0)) }
    }

    pub fn get_max_length(&self) -> Option<u16> {
        unsafe {
            match gtk_sys::gtk_entry_buffer_get_max_length(self.to_glib_none().0) {
                0 => None,
                x => Some(to_u16!(x)),
            }
        }
    }

    pub fn get_text(&self) -> String {
        unsafe { from_glib_none(gtk_sys::gtk_entry_buffer_get_text(self.to_glib_none().0)) }
    }

    pub fn insert_text(&self, position: u16, chars: &str) -> u16 {
        unsafe {
            to_u16!(gtk_sys::gtk_entry_buffer_insert_text(
                self.to_glib_none().0,
                position as c_uint,
                chars.to_glib_none().0,
                -1
            ))
        }
    }

    pub fn set_max_length(&self, max_length: Option<u16>) {
        unsafe {
            assert_ne!(max_length, Some(0), "Zero maximum length not supported");
            gtk_sys::gtk_entry_buffer_set_max_length(
                self.to_glib_none().0,
                max_length.unwrap_or(0) as c_int,
            );
        }
    }

    pub fn set_text(&self, chars: &str) {
        unsafe {
            gtk_sys::gtk_entry_buffer_set_text(self.to_glib_none().0, chars.to_glib_none().0, -1);
        }
    }
}
