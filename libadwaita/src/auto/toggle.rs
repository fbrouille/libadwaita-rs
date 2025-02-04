// This file was generated by gir (https://github.com/gtk-rs/gir)
// from
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::ffi;
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;

glib::wrapper! {
    #[doc(alias = "AdwToggle")]
    pub struct Toggle(Object<ffi::AdwToggle, ffi::AdwToggleClass>);

    match fn {
        type_ => || ffi::adw_toggle_get_type(),
    }
}

impl Toggle {
    #[doc(alias = "adw_toggle_new")]
    pub fn new() -> Toggle {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::adw_toggle_new()) }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`Toggle`] objects.
    ///
    /// This method returns an instance of [`ToggleBuilder`](crate::builders::ToggleBuilder) which can be used to create [`Toggle`] objects.
    pub fn builder() -> ToggleBuilder {
        ToggleBuilder::new()
    }

    #[doc(alias = "adw_toggle_get_child")]
    #[doc(alias = "get_child")]
    pub fn child(&self) -> Option<gtk::Widget> {
        unsafe { from_glib_none(ffi::adw_toggle_get_child(self.to_glib_none().0)) }
    }

    #[doc(alias = "adw_toggle_get_enabled")]
    #[doc(alias = "get_enabled")]
    #[doc(alias = "enabled")]
    pub fn is_enabled(&self) -> bool {
        unsafe { from_glib(ffi::adw_toggle_get_enabled(self.to_glib_none().0)) }
    }

    #[doc(alias = "adw_toggle_get_icon_name")]
    #[doc(alias = "get_icon_name")]
    #[doc(alias = "icon-name")]
    pub fn icon_name(&self) -> Option<glib::GString> {
        unsafe { from_glib_none(ffi::adw_toggle_get_icon_name(self.to_glib_none().0)) }
    }

    #[doc(alias = "adw_toggle_get_index")]
    #[doc(alias = "get_index")]
    pub fn index(&self) -> u32 {
        unsafe { ffi::adw_toggle_get_index(self.to_glib_none().0) }
    }

    #[doc(alias = "adw_toggle_get_label")]
    #[doc(alias = "get_label")]
    pub fn label(&self) -> Option<glib::GString> {
        unsafe { from_glib_none(ffi::adw_toggle_get_label(self.to_glib_none().0)) }
    }

    #[doc(alias = "adw_toggle_get_name")]
    #[doc(alias = "get_name")]
    pub fn name(&self) -> glib::GString {
        unsafe { from_glib_none(ffi::adw_toggle_get_name(self.to_glib_none().0)) }
    }

    #[doc(alias = "adw_toggle_get_tooltip")]
    #[doc(alias = "get_tooltip")]
    pub fn tooltip(&self) -> glib::GString {
        unsafe { from_glib_none(ffi::adw_toggle_get_tooltip(self.to_glib_none().0)) }
    }

    #[doc(alias = "adw_toggle_get_use_underline")]
    #[doc(alias = "get_use_underline")]
    #[doc(alias = "use-underline")]
    pub fn uses_underline(&self) -> bool {
        unsafe { from_glib(ffi::adw_toggle_get_use_underline(self.to_glib_none().0)) }
    }

    #[doc(alias = "adw_toggle_set_child")]
    #[doc(alias = "child")]
    pub fn set_child(&self, child: Option<&impl IsA<gtk::Widget>>) {
        unsafe {
            ffi::adw_toggle_set_child(
                self.to_glib_none().0,
                child.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    #[doc(alias = "adw_toggle_set_enabled")]
    #[doc(alias = "enabled")]
    pub fn set_enabled(&self, enabled: bool) {
        unsafe {
            ffi::adw_toggle_set_enabled(self.to_glib_none().0, enabled.into_glib());
        }
    }

    #[doc(alias = "adw_toggle_set_icon_name")]
    #[doc(alias = "icon-name")]
    pub fn set_icon_name(&self, icon_name: Option<&str>) {
        unsafe {
            ffi::adw_toggle_set_icon_name(self.to_glib_none().0, icon_name.to_glib_none().0);
        }
    }

    #[doc(alias = "adw_toggle_set_label")]
    #[doc(alias = "label")]
    pub fn set_label(&self, label: Option<&str>) {
        unsafe {
            ffi::adw_toggle_set_label(self.to_glib_none().0, label.to_glib_none().0);
        }
    }

    #[doc(alias = "adw_toggle_set_name")]
    #[doc(alias = "name")]
    pub fn set_name(&self, name: Option<&str>) {
        unsafe {
            ffi::adw_toggle_set_name(self.to_glib_none().0, name.to_glib_none().0);
        }
    }

    #[doc(alias = "adw_toggle_set_tooltip")]
    #[doc(alias = "tooltip")]
    pub fn set_tooltip(&self, tooltip: &str) {
        unsafe {
            ffi::adw_toggle_set_tooltip(self.to_glib_none().0, tooltip.to_glib_none().0);
        }
    }

    #[doc(alias = "adw_toggle_set_use_underline")]
    #[doc(alias = "use-underline")]
    pub fn set_use_underline(&self, use_underline: bool) {
        unsafe {
            ffi::adw_toggle_set_use_underline(self.to_glib_none().0, use_underline.into_glib());
        }
    }

    #[cfg(feature = "v1_7")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_7")))]
    #[doc(alias = "child")]
    pub fn connect_child_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_child_trampoline<F: Fn(&Toggle) + 'static>(
            this: *mut ffi::AdwToggle,
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
                b"notify::child\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_child_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v1_7")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_7")))]
    #[doc(alias = "enabled")]
    pub fn connect_enabled_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_enabled_trampoline<F: Fn(&Toggle) + 'static>(
            this: *mut ffi::AdwToggle,
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
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_enabled_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v1_7")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_7")))]
    #[doc(alias = "icon-name")]
    pub fn connect_icon_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_icon_name_trampoline<F: Fn(&Toggle) + 'static>(
            this: *mut ffi::AdwToggle,
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
                b"notify::icon-name\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_icon_name_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v1_7")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_7")))]
    #[doc(alias = "label")]
    pub fn connect_label_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_label_trampoline<F: Fn(&Toggle) + 'static>(
            this: *mut ffi::AdwToggle,
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
                b"notify::label\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_label_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v1_7")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_7")))]
    #[doc(alias = "name")]
    pub fn connect_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_name_trampoline<F: Fn(&Toggle) + 'static>(
            this: *mut ffi::AdwToggle,
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
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_name_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v1_7")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_7")))]
    #[doc(alias = "tooltip")]
    pub fn connect_tooltip_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_tooltip_trampoline<F: Fn(&Toggle) + 'static>(
            this: *mut ffi::AdwToggle,
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
                b"notify::tooltip\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_tooltip_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v1_7")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_7")))]
    #[doc(alias = "use-underline")]
    pub fn connect_use_underline_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_use_underline_trampoline<F: Fn(&Toggle) + 'static>(
            this: *mut ffi::AdwToggle,
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
                b"notify::use-underline\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_use_underline_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

#[cfg(feature = "v1_7")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_7")))]
impl Default for Toggle {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for Toggle {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(&self.name())
    }
}

// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`Toggle`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct ToggleBuilder {
    builder: glib::object::ObjectBuilder<'static, Toggle>,
}

impl ToggleBuilder {
    fn new() -> Self {
        Self {
            builder: glib::object::Object::builder(),
        }
    }

    #[cfg(feature = "v1_7")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_7")))]
    pub fn child(self, child: &impl IsA<gtk::Widget>) -> Self {
        Self {
            builder: self.builder.property("child", child.clone().upcast()),
        }
    }

    #[cfg(feature = "v1_7")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_7")))]
    pub fn enabled(self, enabled: bool) -> Self {
        Self {
            builder: self.builder.property("enabled", enabled),
        }
    }

    #[cfg(feature = "v1_7")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_7")))]
    pub fn icon_name(self, icon_name: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("icon-name", icon_name.into()),
        }
    }

    #[cfg(feature = "v1_7")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_7")))]
    pub fn label(self, label: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("label", label.into()),
        }
    }

    #[cfg(feature = "v1_7")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_7")))]
    pub fn name(self, name: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("name", name.into()),
        }
    }

    #[cfg(feature = "v1_7")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_7")))]
    pub fn tooltip(self, tooltip: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("tooltip", tooltip.into()),
        }
    }

    #[cfg(feature = "v1_7")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_7")))]
    pub fn use_underline(self, use_underline: bool) -> Self {
        Self {
            builder: self.builder.property("use-underline", use_underline),
        }
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`Toggle`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> Toggle {
        assert_initialized_main_thread!();
        self.builder.build()
    }
}
