// This file was generated by gir (https://github.com/gtk-rs/gir)
// from
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::ToastPriority;
use glib::object::Cast;
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
    #[doc(alias = "AdwToast")]
    pub struct Toast(Object<ffi::AdwToast, ffi::AdwToastClass>);

    match fn {
        type_ => || ffi::adw_toast_get_type(),
    }
}

impl Toast {
    #[doc(alias = "adw_toast_new")]
    pub fn new(title: &str) -> Toast {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::adw_toast_new(title.to_glib_none().0)) }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`Toast`] objects.
    ///
    /// This method returns an instance of [`ToastBuilder`] which can be used to create [`Toast`] objects.
    pub fn builder() -> ToastBuilder {
        ToastBuilder::default()
    }

    #[doc(alias = "adw_toast_dismiss")]
    pub fn dismiss(&self) {
        unsafe {
            ffi::adw_toast_dismiss(self.to_glib_none().0);
        }
    }

    #[doc(alias = "adw_toast_get_action_name")]
    #[doc(alias = "get_action_name")]
    pub fn action_name(&self) -> Option<glib::GString> {
        unsafe { from_glib_none(ffi::adw_toast_get_action_name(self.to_glib_none().0)) }
    }

    #[doc(alias = "adw_toast_get_action_target_value")]
    #[doc(alias = "get_action_target_value")]
    pub fn action_target_value(&self) -> Option<glib::Variant> {
        unsafe {
            from_glib_none(ffi::adw_toast_get_action_target_value(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "adw_toast_get_button_label")]
    #[doc(alias = "get_button_label")]
    pub fn button_label(&self) -> Option<glib::GString> {
        unsafe { from_glib_none(ffi::adw_toast_get_button_label(self.to_glib_none().0)) }
    }

    #[doc(alias = "adw_toast_get_priority")]
    #[doc(alias = "get_priority")]
    pub fn priority(&self) -> ToastPriority {
        unsafe { from_glib(ffi::adw_toast_get_priority(self.to_glib_none().0)) }
    }

    #[doc(alias = "adw_toast_get_title")]
    #[doc(alias = "get_title")]
    pub fn title(&self) -> Option<glib::GString> {
        unsafe { from_glib_none(ffi::adw_toast_get_title(self.to_glib_none().0)) }
    }

    #[doc(alias = "adw_toast_set_action_name")]
    pub fn set_action_name(&self, action_name: Option<&str>) {
        unsafe {
            ffi::adw_toast_set_action_name(self.to_glib_none().0, action_name.to_glib_none().0);
        }
    }

    #[doc(alias = "adw_toast_set_action_target_value")]
    pub fn set_action_target_value(&self, action_target: Option<&glib::Variant>) {
        unsafe {
            ffi::adw_toast_set_action_target_value(
                self.to_glib_none().0,
                action_target.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "adw_toast_set_button_label")]
    pub fn set_button_label(&self, button_label: Option<&str>) {
        unsafe {
            ffi::adw_toast_set_button_label(self.to_glib_none().0, button_label.to_glib_none().0);
        }
    }

    #[doc(alias = "adw_toast_set_detailed_action_name")]
    pub fn set_detailed_action_name(&self, detailed_action_name: Option<&str>) {
        unsafe {
            ffi::adw_toast_set_detailed_action_name(
                self.to_glib_none().0,
                detailed_action_name.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "adw_toast_set_priority")]
    pub fn set_priority(&self, priority: ToastPriority) {
        unsafe {
            ffi::adw_toast_set_priority(self.to_glib_none().0, priority.into_glib());
        }
    }

    #[doc(alias = "adw_toast_set_title")]
    pub fn set_title(&self, title: &str) {
        unsafe {
            ffi::adw_toast_set_title(self.to_glib_none().0, title.to_glib_none().0);
        }
    }

    #[doc(alias = "action-target")]
    pub fn action_target(&self) -> Option<glib::Variant> {
        unsafe {
            let mut value = glib::Value::from_type(<glib::Variant as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"action-target\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `action-target` getter")
        }
    }

    #[doc(alias = "action-target")]
    pub fn set_action_target(&self, action_target: Option<&glib::Variant>) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"action-target\0".as_ptr() as *const _,
                action_target.to_value().to_glib_none().0,
            );
        }
    }

    #[doc(alias = "dismissed")]
    pub fn connect_dismissed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn dismissed_trampoline<F: Fn(&Toast) + 'static>(
            this: *mut ffi::AdwToast,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"dismissed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    dismissed_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "action-name")]
    pub fn connect_action_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_action_name_trampoline<F: Fn(&Toast) + 'static>(
            this: *mut ffi::AdwToast,
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
                b"notify::action-name\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_action_name_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "action-target")]
    pub fn connect_action_target_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_action_target_trampoline<F: Fn(&Toast) + 'static>(
            this: *mut ffi::AdwToast,
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
                b"notify::action-target\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_action_target_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "button-label")]
    pub fn connect_button_label_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_button_label_trampoline<F: Fn(&Toast) + 'static>(
            this: *mut ffi::AdwToast,
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
                b"notify::button-label\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_button_label_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "priority")]
    pub fn connect_priority_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_priority_trampoline<F: Fn(&Toast) + 'static>(
            this: *mut ffi::AdwToast,
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
                b"notify::priority\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_priority_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "title")]
    pub fn connect_title_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_title_trampoline<F: Fn(&Toast) + 'static>(
            this: *mut ffi::AdwToast,
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
                b"notify::title\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_title_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

#[derive(Clone, Default)]
// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`Toast`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
pub struct ToastBuilder {
    action_name: Option<String>,
    action_target: Option<glib::Variant>,
    button_label: Option<String>,
    priority: Option<ToastPriority>,
    title: Option<String>,
}

impl ToastBuilder {
    // rustdoc-stripper-ignore-next
    /// Create a new [`ToastBuilder`].
    pub fn new() -> Self {
        Self::default()
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`Toast`].
    pub fn build(self) -> Toast {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref action_name) = self.action_name {
            properties.push(("action-name", action_name));
        }
        if let Some(ref action_target) = self.action_target {
            properties.push(("action-target", action_target));
        }
        if let Some(ref button_label) = self.button_label {
            properties.push(("button-label", button_label));
        }
        if let Some(ref priority) = self.priority {
            properties.push(("priority", priority));
        }
        if let Some(ref title) = self.title {
            properties.push(("title", title));
        }
        glib::Object::new::<Toast>(&properties).expect("Failed to create an instance of Toast")
    }

    pub fn action_name(mut self, action_name: &str) -> Self {
        self.action_name = Some(action_name.to_string());
        self
    }

    pub fn action_target(mut self, action_target: &glib::Variant) -> Self {
        self.action_target = Some(action_target.clone());
        self
    }

    pub fn button_label(mut self, button_label: &str) -> Self {
        self.button_label = Some(button_label.to_string());
        self
    }

    pub fn priority(mut self, priority: ToastPriority) -> Self {
        self.priority = Some(priority);
        self
    }

    pub fn title(mut self, title: &str) -> Self {
        self.title = Some(title.to_string());
        self
    }
}

impl fmt::Display for Toast {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Toast")
    }
}
