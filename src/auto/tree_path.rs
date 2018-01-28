// This file was generated by gir (9261d77) from gir-files (469db10)
// DO NOT EDIT

use ffi;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::cmp;
use std::fmt;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct TreePath(Boxed<ffi::GtkTreePath>);

    match fn {
        copy => |ptr| ffi::gtk_tree_path_copy(mut_override(ptr)),
        free => |ptr| ffi::gtk_tree_path_free(ptr),
        get_type => || ffi::gtk_tree_path_get_type(),
    }
}

impl TreePath {
    pub fn new() -> TreePath {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gtk_tree_path_new())
        }
    }

    pub fn new_first() -> TreePath {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gtk_tree_path_new_first())
        }
    }

    //pub fn new_from_indices(first_index: i32, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) -> TreePath {
    //    unsafe { TODO: call ffi::gtk_tree_path_new_from_indices() }
    //}

    #[cfg(any(feature = "v3_12", feature = "dox"))]
    pub fn new_from_indicesv(indices: &[i32]) -> TreePath {
        assert_initialized_main_thread!();
        let length = indices.len() as usize;
        unsafe {
            from_glib_full(ffi::gtk_tree_path_new_from_indicesv(indices.to_glib_none().0, length))
        }
    }

    pub fn new_from_string(path: &str) -> TreePath {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gtk_tree_path_new_from_string(path.to_glib_none().0))
        }
    }

    pub fn append_index(&mut self, index_: i32) {
        unsafe {
            ffi::gtk_tree_path_append_index(self.to_glib_none_mut().0, index_);
        }
    }

    fn compare(&self, b: &TreePath) -> i32 {
        unsafe {
            ffi::gtk_tree_path_compare(self.to_glib_none().0, b.to_glib_none().0)
        }
    }

    pub fn down(&mut self) {
        unsafe {
            ffi::gtk_tree_path_down(self.to_glib_none_mut().0);
        }
    }

    pub fn get_depth(&self) -> i32 {
        unsafe {
            ffi::gtk_tree_path_get_depth(mut_override(self.to_glib_none().0))
        }
    }

    pub fn get_indices_with_depth(&mut self) -> Vec<i32> {
        unsafe {
            let mut depth = mem::uninitialized();
            let ret = FromGlibContainer::from_glib_none_num(ffi::gtk_tree_path_get_indices_with_depth(self.to_glib_none_mut().0, &mut depth), depth as usize);
            ret
        }
    }

    pub fn is_ancestor(&self, descendant: &TreePath) -> bool {
        unsafe {
            from_glib(ffi::gtk_tree_path_is_ancestor(mut_override(self.to_glib_none().0), mut_override(descendant.to_glib_none().0)))
        }
    }

    pub fn is_descendant(&self, ancestor: &TreePath) -> bool {
        unsafe {
            from_glib(ffi::gtk_tree_path_is_descendant(mut_override(self.to_glib_none().0), mut_override(ancestor.to_glib_none().0)))
        }
    }

    pub fn next(&mut self) {
        unsafe {
            ffi::gtk_tree_path_next(self.to_glib_none_mut().0);
        }
    }

    pub fn prepend_index(&mut self, index_: i32) {
        unsafe {
            ffi::gtk_tree_path_prepend_index(self.to_glib_none_mut().0, index_);
        }
    }

    pub fn prev(&mut self) -> bool {
        unsafe {
            from_glib(ffi::gtk_tree_path_prev(self.to_glib_none_mut().0))
        }
    }

    fn to_string(&self) -> String {
        unsafe {
            from_glib_full(ffi::gtk_tree_path_to_string(mut_override(self.to_glib_none().0)))
        }
    }

    pub fn up(&mut self) -> bool {
        unsafe {
            from_glib(ffi::gtk_tree_path_up(self.to_glib_none_mut().0))
        }
    }
}

impl Default for TreePath {
    fn default() -> Self {
        Self::new()
    }
}

impl PartialEq for TreePath {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.compare(other) == 0
    }
}

impl Eq for TreePath {}

impl PartialOrd for TreePath {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        self.compare(other).partial_cmp(&0)
    }
}

impl Ord for TreePath {
    #[inline]
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.compare(other).cmp(&0)
    }
}

impl fmt::Display for TreePath {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
