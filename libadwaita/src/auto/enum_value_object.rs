// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use glib::object::ObjectType as ObjectType_;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::wrapper! {
    pub struct EnumValueObject(Object<ffi::AdwEnumValueObject, ffi::AdwEnumValueObjectClass>);

    match fn {
        get_type => || ffi::adw_enum_value_object_get_type(),
    }
}

impl EnumValueObject {
    #[doc(alias = "adw_enum_value_object_get_name")]
    pub fn name(&self) -> Option<glib::GString> {
        unsafe { from_glib_none(ffi::adw_enum_value_object_get_name(self.to_glib_none().0)) }
    }

    #[doc(alias = "adw_enum_value_object_get_nick")]
    pub fn nick(&self) -> Option<glib::GString> {
        unsafe { from_glib_none(ffi::adw_enum_value_object_get_nick(self.to_glib_none().0)) }
    }

    #[doc(alias = "adw_enum_value_object_get_value")]
    pub fn value(&self) -> i32 {
        unsafe { ffi::adw_enum_value_object_get_value(self.to_glib_none().0) }
    }

    pub fn connect_property_name_notify<F: Fn(&EnumValueObject) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_name_trampoline<F: Fn(&EnumValueObject) + 'static>(
            this: *mut ffi::AdwEnumValueObject,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::name\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_name_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_nick_notify<F: Fn(&EnumValueObject) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_nick_trampoline<F: Fn(&EnumValueObject) + 'static>(
            this: *mut ffi::AdwEnumValueObject,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::nick\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_nick_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_value_notify<F: Fn(&EnumValueObject) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_value_trampoline<F: Fn(&EnumValueObject) + 'static>(
            this: *mut ffi::AdwEnumValueObject,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::value\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_value_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for EnumValueObject {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("EnumValueObject")
    }
}
