// This file was generated by gir (https://github.com/gtk-rs/gir)
// from
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::StyleManager;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::wrapper! {
    #[doc(alias = "AdwApplication")]
    pub struct Application(Object<ffi::AdwApplication, ffi::AdwApplicationClass>) @extends gtk::Application, gio::Application, @implements gio::ActionGroup, gio::ActionMap;

    match fn {
        type_ => || ffi::adw_application_get_type(),
    }
}

impl Application {
    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`Application`] objects.
    ///
    /// This method returns an instance of [`ApplicationBuilder`] which can be used to create [`Application`] objects.
    pub fn builder() -> ApplicationBuilder {
        ApplicationBuilder::default()
    }
}

#[derive(Clone, Default)]
// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`Application`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
pub struct ApplicationBuilder {
    menubar: Option<gio::MenuModel>,
    register_session: Option<bool>,
    action_group: Option<gio::ActionGroup>,
    application_id: Option<String>,
    flags: Option<gio::ApplicationFlags>,
    inactivity_timeout: Option<u32>,
    resource_base_path: Option<String>,
}

impl ApplicationBuilder {
    // rustdoc-stripper-ignore-next
    /// Create a new [`ApplicationBuilder`].
    pub fn new() -> Self {
        Self::default()
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`Application`].
    pub fn build(self) -> Application {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref menubar) = self.menubar {
            properties.push(("menubar", menubar));
        }
        if let Some(ref register_session) = self.register_session {
            properties.push(("register-session", register_session));
        }
        if let Some(ref action_group) = self.action_group {
            properties.push(("action-group", action_group));
        }
        if let Some(ref application_id) = self.application_id {
            properties.push(("application-id", application_id));
        }
        if let Some(ref flags) = self.flags {
            properties.push(("flags", flags));
        }
        if let Some(ref inactivity_timeout) = self.inactivity_timeout {
            properties.push(("inactivity-timeout", inactivity_timeout));
        }
        if let Some(ref resource_base_path) = self.resource_base_path {
            properties.push(("resource-base-path", resource_base_path));
        }
        let ret = glib::Object::new::<Application>(&properties)
            .expect("Failed to create an instance of Application");
        {
            Application::register_startup_hook(&ret);
        }
        ret
    }

    pub fn menubar(mut self, menubar: &impl IsA<gio::MenuModel>) -> Self {
        self.menubar = Some(menubar.clone().upcast());
        self
    }

    pub fn register_session(mut self, register_session: bool) -> Self {
        self.register_session = Some(register_session);
        self
    }

    pub fn action_group(mut self, action_group: &impl IsA<gio::ActionGroup>) -> Self {
        self.action_group = Some(action_group.clone().upcast());
        self
    }

    pub fn application_id(mut self, application_id: &str) -> Self {
        self.application_id = Some(application_id.to_string());
        self
    }

    pub fn flags(mut self, flags: gio::ApplicationFlags) -> Self {
        self.flags = Some(flags);
        self
    }

    pub fn inactivity_timeout(mut self, inactivity_timeout: u32) -> Self {
        self.inactivity_timeout = Some(inactivity_timeout);
        self
    }

    pub fn resource_base_path(mut self, resource_base_path: &str) -> Self {
        self.resource_base_path = Some(resource_base_path.to_string());
        self
    }
}

impl Application {
    pub const NONE: Option<&'static Application> = None;
}

pub trait AdwApplicationExt: 'static {
    #[doc(alias = "adw_application_get_style_manager")]
    #[doc(alias = "get_style_manager")]
    fn style_manager(&self) -> StyleManager;

    #[doc(alias = "style-manager")]
    fn connect_style_manager_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Application>> AdwApplicationExt for O {
    fn style_manager(&self) -> StyleManager {
        unsafe {
            from_glib_none(ffi::adw_application_get_style_manager(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn connect_style_manager_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_style_manager_trampoline<
            P: IsA<Application>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::AdwApplication,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Application::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::style-manager\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_style_manager_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for Application {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Application")
    }
}
