// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use glib::object::Cast;
use glib::object::IsA;
use glib::object::ObjectType as ObjectType_;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::wrapper! {
    pub struct SqueezerPage(Object<ffi::AdwSqueezerPage, ffi::AdwSqueezerPageClass>);

    match fn {
        get_type => || ffi::adw_squeezer_page_get_type(),
    }
}

impl SqueezerPage {
    #[doc(alias = "adw_squeezer_page_get_child")]
    pub fn child(&self) -> Option<gtk::Widget> {
        unsafe { from_glib_none(ffi::adw_squeezer_page_get_child(self.to_glib_none().0)) }
    }

    #[doc(alias = "adw_squeezer_page_get_enabled")]
    pub fn is_enabled(&self) -> bool {
        unsafe { from_glib(ffi::adw_squeezer_page_get_enabled(self.to_glib_none().0)) }
    }

    #[doc(alias = "adw_squeezer_page_set_enabled")]
    pub fn set_enabled(&self, enabled: bool) {
        unsafe {
            ffi::adw_squeezer_page_set_enabled(self.to_glib_none().0, enabled.to_glib());
        }
    }

    pub fn connect_property_enabled_notify<F: Fn(&SqueezerPage) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_enabled_trampoline<F: Fn(&SqueezerPage) + 'static>(
            this: *mut ffi::AdwSqueezerPage,
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
                b"notify::enabled\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_enabled_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

#[derive(Clone, Default)]
pub struct SqueezerPageBuilder {
    child: Option<gtk::Widget>,
    enabled: Option<bool>,
}

impl SqueezerPageBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> SqueezerPage {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref child) = self.child {
            properties.push(("child", child));
        }
        if let Some(ref enabled) = self.enabled {
            properties.push(("enabled", enabled));
        }
        let ret = glib::Object::new::<SqueezerPage>(&properties).expect("object new");
        ret
    }

    pub fn child<P: IsA<gtk::Widget>>(mut self, child: &P) -> Self {
        self.child = Some(child.clone().upcast());
        self
    }

    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = Some(enabled);
        self
    }
}

impl fmt::Display for SqueezerPage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("SqueezerPage")
    }
}
