// This file was generated by gir (https://github.com/gtk-rs/gir)
// from
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

#[cfg(feature = "v1_4")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_4")))]
use crate::Breakpoint;
#[cfg(feature = "v1_5")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_5")))]
use crate::Dialog;
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;

glib::wrapper! {
    #[doc(alias = "AdwApplicationWindow")]
    pub struct ApplicationWindow(Object<ffi::AdwApplicationWindow, ffi::AdwApplicationWindowClass>) @extends gtk::ApplicationWindow, gtk::Window, gtk::Widget, @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager, gio::ActionGroup, gio::ActionMap;

    match fn {
        type_ => || ffi::adw_application_window_get_type(),
    }
}

impl ApplicationWindow {
    pub const NONE: Option<&'static ApplicationWindow> = None;

    #[doc(alias = "adw_application_window_new")]
    pub fn new(app: &impl IsA<gtk::Application>) -> ApplicationWindow {
        assert_initialized_main_thread!();
        unsafe {
            gtk::Widget::from_glib_none(ffi::adw_application_window_new(
                app.as_ref().to_glib_none().0,
            ))
            .unsafe_cast()
        }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`ApplicationWindow`] objects.
    ///
    /// This method returns an instance of [`ApplicationWindowBuilder`](crate::builders::ApplicationWindowBuilder) which can be used to create [`ApplicationWindow`] objects.
    pub fn builder() -> ApplicationWindowBuilder {
        ApplicationWindowBuilder::new()
    }
}

impl Default for ApplicationWindow {
    fn default() -> Self {
        glib::object::Object::new::<Self>()
    }
}

// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`ApplicationWindow`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct ApplicationWindowBuilder {
    builder: glib::object::ObjectBuilder<'static, ApplicationWindow>,
}

impl ApplicationWindowBuilder {
    fn new() -> Self {
        Self {
            builder: glib::object::Object::builder(),
        }
    }

    pub fn content(self, content: &impl IsA<gtk::Widget>) -> Self {
        Self {
            builder: self.builder.property("content", content.clone().upcast()),
        }
    }

    pub fn show_menubar(self, show_menubar: bool) -> Self {
        Self {
            builder: self.builder.property("show-menubar", show_menubar),
        }
    }

    pub fn application(self, application: &impl IsA<gtk::Application>) -> Self {
        Self {
            builder: self
                .builder
                .property("application", application.clone().upcast()),
        }
    }

    pub fn decorated(self, decorated: bool) -> Self {
        Self {
            builder: self.builder.property("decorated", decorated),
        }
    }

    pub fn default_height(self, default_height: i32) -> Self {
        Self {
            builder: self.builder.property("default-height", default_height),
        }
    }

    pub fn default_widget(self, default_widget: &impl IsA<gtk::Widget>) -> Self {
        Self {
            builder: self
                .builder
                .property("default-widget", default_widget.clone().upcast()),
        }
    }

    pub fn default_width(self, default_width: i32) -> Self {
        Self {
            builder: self.builder.property("default-width", default_width),
        }
    }

    pub fn deletable(self, deletable: bool) -> Self {
        Self {
            builder: self.builder.property("deletable", deletable),
        }
    }

    pub fn destroy_with_parent(self, destroy_with_parent: bool) -> Self {
        Self {
            builder: self
                .builder
                .property("destroy-with-parent", destroy_with_parent),
        }
    }

    pub fn display(self, display: &gdk::Display) -> Self {
        Self {
            builder: self.builder.property("display", display.clone()),
        }
    }

    pub fn focus_visible(self, focus_visible: bool) -> Self {
        Self {
            builder: self.builder.property("focus-visible", focus_visible),
        }
    }

    pub fn focus_widget(self, focus_widget: &impl IsA<gtk::Widget>) -> Self {
        Self {
            builder: self
                .builder
                .property("focus-widget", focus_widget.clone().upcast()),
        }
    }

    pub fn fullscreened(self, fullscreened: bool) -> Self {
        Self {
            builder: self.builder.property("fullscreened", fullscreened),
        }
    }

    #[cfg(feature = "gtk_v4_2")]
    #[cfg_attr(docsrs, doc(cfg(feature = "gtk_v4_2")))]
    pub fn handle_menubar_accel(self, handle_menubar_accel: bool) -> Self {
        Self {
            builder: self
                .builder
                .property("handle-menubar-accel", handle_menubar_accel),
        }
    }

    pub fn hide_on_close(self, hide_on_close: bool) -> Self {
        Self {
            builder: self.builder.property("hide-on-close", hide_on_close),
        }
    }

    pub fn icon_name(self, icon_name: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("icon-name", icon_name.into()),
        }
    }

    pub fn maximized(self, maximized: bool) -> Self {
        Self {
            builder: self.builder.property("maximized", maximized),
        }
    }

    pub fn mnemonics_visible(self, mnemonics_visible: bool) -> Self {
        Self {
            builder: self
                .builder
                .property("mnemonics-visible", mnemonics_visible),
        }
    }

    pub fn modal(self, modal: bool) -> Self {
        Self {
            builder: self.builder.property("modal", modal),
        }
    }

    pub fn resizable(self, resizable: bool) -> Self {
        Self {
            builder: self.builder.property("resizable", resizable),
        }
    }

    pub fn startup_id(self, startup_id: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("startup-id", startup_id.into()),
        }
    }

    pub fn title(self, title: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("title", title.into()),
        }
    }

    #[cfg(feature = "gtk_v4_6")]
    #[cfg_attr(docsrs, doc(cfg(feature = "gtk_v4_6")))]
    pub fn titlebar(self, titlebar: &impl IsA<gtk::Widget>) -> Self {
        Self {
            builder: self.builder.property("titlebar", titlebar.clone().upcast()),
        }
    }

    pub fn transient_for(self, transient_for: &impl IsA<gtk::Window>) -> Self {
        Self {
            builder: self
                .builder
                .property("transient-for", transient_for.clone().upcast()),
        }
    }

    pub fn can_focus(self, can_focus: bool) -> Self {
        Self {
            builder: self.builder.property("can-focus", can_focus),
        }
    }

    pub fn can_target(self, can_target: bool) -> Self {
        Self {
            builder: self.builder.property("can-target", can_target),
        }
    }

    pub fn css_classes(self, css_classes: impl Into<glib::StrV>) -> Self {
        Self {
            builder: self.builder.property("css-classes", css_classes.into()),
        }
    }

    pub fn css_name(self, css_name: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("css-name", css_name.into()),
        }
    }

    pub fn cursor(self, cursor: &gdk::Cursor) -> Self {
        Self {
            builder: self.builder.property("cursor", cursor.clone()),
        }
    }

    pub fn focus_on_click(self, focus_on_click: bool) -> Self {
        Self {
            builder: self.builder.property("focus-on-click", focus_on_click),
        }
    }

    pub fn focusable(self, focusable: bool) -> Self {
        Self {
            builder: self.builder.property("focusable", focusable),
        }
    }

    pub fn halign(self, halign: gtk::Align) -> Self {
        Self {
            builder: self.builder.property("halign", halign),
        }
    }

    pub fn has_tooltip(self, has_tooltip: bool) -> Self {
        Self {
            builder: self.builder.property("has-tooltip", has_tooltip),
        }
    }

    pub fn height_request(self, height_request: i32) -> Self {
        Self {
            builder: self.builder.property("height-request", height_request),
        }
    }

    pub fn hexpand(self, hexpand: bool) -> Self {
        Self {
            builder: self.builder.property("hexpand", hexpand),
        }
    }

    pub fn hexpand_set(self, hexpand_set: bool) -> Self {
        Self {
            builder: self.builder.property("hexpand-set", hexpand_set),
        }
    }

    pub fn layout_manager(self, layout_manager: &impl IsA<gtk::LayoutManager>) -> Self {
        Self {
            builder: self
                .builder
                .property("layout-manager", layout_manager.clone().upcast()),
        }
    }

    pub fn margin_bottom(self, margin_bottom: i32) -> Self {
        Self {
            builder: self.builder.property("margin-bottom", margin_bottom),
        }
    }

    pub fn margin_end(self, margin_end: i32) -> Self {
        Self {
            builder: self.builder.property("margin-end", margin_end),
        }
    }

    pub fn margin_start(self, margin_start: i32) -> Self {
        Self {
            builder: self.builder.property("margin-start", margin_start),
        }
    }

    pub fn margin_top(self, margin_top: i32) -> Self {
        Self {
            builder: self.builder.property("margin-top", margin_top),
        }
    }

    pub fn name(self, name: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("name", name.into()),
        }
    }

    pub fn opacity(self, opacity: f64) -> Self {
        Self {
            builder: self.builder.property("opacity", opacity),
        }
    }

    pub fn overflow(self, overflow: gtk::Overflow) -> Self {
        Self {
            builder: self.builder.property("overflow", overflow),
        }
    }

    pub fn receives_default(self, receives_default: bool) -> Self {
        Self {
            builder: self.builder.property("receives-default", receives_default),
        }
    }

    pub fn sensitive(self, sensitive: bool) -> Self {
        Self {
            builder: self.builder.property("sensitive", sensitive),
        }
    }

    pub fn tooltip_markup(self, tooltip_markup: impl Into<glib::GString>) -> Self {
        Self {
            builder: self
                .builder
                .property("tooltip-markup", tooltip_markup.into()),
        }
    }

    pub fn tooltip_text(self, tooltip_text: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("tooltip-text", tooltip_text.into()),
        }
    }

    pub fn valign(self, valign: gtk::Align) -> Self {
        Self {
            builder: self.builder.property("valign", valign),
        }
    }

    pub fn vexpand(self, vexpand: bool) -> Self {
        Self {
            builder: self.builder.property("vexpand", vexpand),
        }
    }

    pub fn vexpand_set(self, vexpand_set: bool) -> Self {
        Self {
            builder: self.builder.property("vexpand-set", vexpand_set),
        }
    }

    pub fn visible(self, visible: bool) -> Self {
        Self {
            builder: self.builder.property("visible", visible),
        }
    }

    pub fn width_request(self, width_request: i32) -> Self {
        Self {
            builder: self.builder.property("width-request", width_request),
        }
    }

    pub fn accessible_role(self, accessible_role: gtk::AccessibleRole) -> Self {
        Self {
            builder: self.builder.property("accessible-role", accessible_role),
        }
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`ApplicationWindow`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> ApplicationWindow {
        self.builder.build()
    }
}

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::ApplicationWindow>> Sealed for T {}
}

pub trait AdwApplicationWindowExt: IsA<ApplicationWindow> + sealed::Sealed + 'static {
    #[cfg(feature = "v1_4")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_4")))]
    #[doc(alias = "adw_application_window_add_breakpoint")]
    fn add_breakpoint(&self, breakpoint: Breakpoint) {
        unsafe {
            ffi::adw_application_window_add_breakpoint(
                self.as_ref().to_glib_none().0,
                breakpoint.into_glib_ptr(),
            );
        }
    }

    #[doc(alias = "adw_application_window_get_content")]
    #[doc(alias = "get_content")]
    fn content(&self) -> Option<gtk::Widget> {
        unsafe {
            from_glib_none(ffi::adw_application_window_get_content(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(feature = "v1_4")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_4")))]
    #[doc(alias = "adw_application_window_get_current_breakpoint")]
    #[doc(alias = "get_current_breakpoint")]
    fn current_breakpoint(&self) -> Option<Breakpoint> {
        unsafe {
            from_glib_none(ffi::adw_application_window_get_current_breakpoint(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(feature = "v1_5")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_5")))]
    #[doc(alias = "adw_application_window_get_dialogs")]
    #[doc(alias = "get_dialogs")]
    fn dialogs(&self) -> gio::ListModel {
        unsafe {
            from_glib_full(ffi::adw_application_window_get_dialogs(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(feature = "v1_5")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_5")))]
    #[doc(alias = "adw_application_window_get_visible_dialog")]
    #[doc(alias = "get_visible_dialog")]
    fn visible_dialog(&self) -> Option<Dialog> {
        unsafe {
            from_glib_none(ffi::adw_application_window_get_visible_dialog(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "adw_application_window_set_content")]
    fn set_content(&self, content: Option<&impl IsA<gtk::Widget>>) {
        unsafe {
            ffi::adw_application_window_set_content(
                self.as_ref().to_glib_none().0,
                content.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    #[doc(alias = "content")]
    fn connect_content_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_content_trampoline<
            P: IsA<ApplicationWindow>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::AdwApplicationWindow,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(ApplicationWindow::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::content\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    notify_content_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v1_4")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_4")))]
    #[doc(alias = "current-breakpoint")]
    fn connect_current_breakpoint_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_current_breakpoint_trampoline<
            P: IsA<ApplicationWindow>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::AdwApplicationWindow,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(ApplicationWindow::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::current-breakpoint\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    notify_current_breakpoint_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v1_5")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_5")))]
    #[doc(alias = "dialogs")]
    fn connect_dialogs_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_dialogs_trampoline<
            P: IsA<ApplicationWindow>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::AdwApplicationWindow,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(ApplicationWindow::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::dialogs\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    notify_dialogs_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v1_5")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_5")))]
    #[doc(alias = "visible-dialog")]
    fn connect_visible_dialog_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_visible_dialog_trampoline<
            P: IsA<ApplicationWindow>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::AdwApplicationWindow,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(ApplicationWindow::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::visible-dialog\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    notify_visible_dialog_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl<O: IsA<ApplicationWindow>> AdwApplicationWindowExt for O {}
