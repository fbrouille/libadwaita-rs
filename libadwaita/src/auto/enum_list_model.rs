// This file was generated by gir (https://github.com/gtk-rs/gir)
// from
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use glib::translate::*;
use std::fmt;

glib::wrapper! {
    pub struct EnumListModel(Object<ffi::AdwEnumListModel, ffi::AdwEnumListModelClass>) @implements gio::ListModel;

    match fn {
        type_ => || ffi::adw_enum_list_model_get_type(),
    }
}

impl EnumListModel {
    #[doc(alias = "adw_enum_list_model_new")]
    pub fn new(enum_type: glib::types::Type) -> EnumListModel {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::adw_enum_list_model_new(enum_type.to_glib())) }
    }

    #[doc(alias = "adw_enum_list_model_find_position")]
    pub fn find_position(&self, value: i32) -> u32 {
        unsafe { ffi::adw_enum_list_model_find_position(self.to_glib_none().0, value) }
    }

    #[doc(alias = "adw_enum_list_model_get_enum_type")]
    pub fn enum_type(&self) -> glib::types::Type {
        unsafe {
            from_glib(ffi::adw_enum_list_model_get_enum_type(
                self.to_glib_none().0,
            ))
        }
    }
}

impl fmt::Display for EnumListModel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("EnumListModel")
    }
}
