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
    #[doc(alias = "AdwStatusPage")]
    pub struct StatusPage(Object<ffi::AdwStatusPage, ffi::AdwStatusPageClass>) @extends gtk::Widget, @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;

    match fn {
        type_ => || ffi::adw_status_page_get_type(),
    }
}

impl StatusPage {
    #[doc(alias = "adw_status_page_new")]
    pub fn new() -> StatusPage {
        assert_initialized_main_thread!();
        unsafe { gtk::Widget::from_glib_none(ffi::adw_status_page_new()).unsafe_cast() }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`StatusPage`] objects.
    ///
    /// This method returns an instance of [`StatusPageBuilder`](crate::builders::StatusPageBuilder) which can be used to create [`StatusPage`] objects.
    pub fn builder() -> StatusPageBuilder {
        StatusPageBuilder::new()
    }

    #[doc(alias = "adw_status_page_get_child")]
    #[doc(alias = "get_child")]
    pub fn child(&self) -> Option<gtk::Widget> {
        unsafe { from_glib_none(ffi::adw_status_page_get_child(self.to_glib_none().0)) }
    }

    #[doc(alias = "adw_status_page_get_description")]
    #[doc(alias = "get_description")]
    pub fn description(&self) -> Option<glib::GString> {
        unsafe { from_glib_none(ffi::adw_status_page_get_description(self.to_glib_none().0)) }
    }

    #[doc(alias = "adw_status_page_get_icon_name")]
    #[doc(alias = "get_icon_name")]
    #[doc(alias = "icon-name")]
    pub fn icon_name(&self) -> Option<glib::GString> {
        unsafe { from_glib_none(ffi::adw_status_page_get_icon_name(self.to_glib_none().0)) }
    }

    #[doc(alias = "adw_status_page_get_paintable")]
    #[doc(alias = "get_paintable")]
    pub fn paintable(&self) -> Option<gdk::Paintable> {
        unsafe { from_glib_none(ffi::adw_status_page_get_paintable(self.to_glib_none().0)) }
    }

    #[doc(alias = "adw_status_page_get_title")]
    #[doc(alias = "get_title")]
    pub fn title(&self) -> glib::GString {
        unsafe { from_glib_none(ffi::adw_status_page_get_title(self.to_glib_none().0)) }
    }

    #[doc(alias = "adw_status_page_set_child")]
    #[doc(alias = "child")]
    pub fn set_child(&self, child: Option<&impl IsA<gtk::Widget>>) {
        unsafe {
            ffi::adw_status_page_set_child(
                self.to_glib_none().0,
                child.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    #[doc(alias = "adw_status_page_set_description")]
    #[doc(alias = "description")]
    pub fn set_description(&self, description: Option<&str>) {
        unsafe {
            ffi::adw_status_page_set_description(
                self.to_glib_none().0,
                description.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "adw_status_page_set_icon_name")]
    #[doc(alias = "icon-name")]
    pub fn set_icon_name(&self, icon_name: Option<&str>) {
        unsafe {
            ffi::adw_status_page_set_icon_name(self.to_glib_none().0, icon_name.to_glib_none().0);
        }
    }

    #[doc(alias = "adw_status_page_set_paintable")]
    #[doc(alias = "paintable")]
    pub fn set_paintable(&self, paintable: Option<&impl IsA<gdk::Paintable>>) {
        unsafe {
            ffi::adw_status_page_set_paintable(
                self.to_glib_none().0,
                paintable.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    #[doc(alias = "adw_status_page_set_title")]
    #[doc(alias = "title")]
    pub fn set_title(&self, title: &str) {
        unsafe {
            ffi::adw_status_page_set_title(self.to_glib_none().0, title.to_glib_none().0);
        }
    }

    #[doc(alias = "child")]
    pub fn connect_child_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_child_trampoline<F: Fn(&StatusPage) + 'static>(
            this: *mut ffi::AdwStatusPage,
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

    #[doc(alias = "description")]
    pub fn connect_description_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_description_trampoline<F: Fn(&StatusPage) + 'static>(
            this: *mut ffi::AdwStatusPage,
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
                b"notify::description\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_description_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "icon-name")]
    pub fn connect_icon_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_icon_name_trampoline<F: Fn(&StatusPage) + 'static>(
            this: *mut ffi::AdwStatusPage,
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

    #[doc(alias = "paintable")]
    pub fn connect_paintable_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_paintable_trampoline<F: Fn(&StatusPage) + 'static>(
            this: *mut ffi::AdwStatusPage,
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
                b"notify::paintable\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_paintable_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "title")]
    pub fn connect_title_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_title_trampoline<F: Fn(&StatusPage) + 'static>(
            this: *mut ffi::AdwStatusPage,
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
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_title_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl Default for StatusPage {
    fn default() -> Self {
        Self::new()
    }
}

// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`StatusPage`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct StatusPageBuilder {
    builder: glib::object::ObjectBuilder<'static, StatusPage>,
}

impl StatusPageBuilder {
    fn new() -> Self {
        Self {
            builder: glib::object::Object::builder(),
        }
    }

    pub fn child(self, child: &impl IsA<gtk::Widget>) -> Self {
        Self {
            builder: self.builder.property("child", child.clone().upcast()),
        }
    }

    pub fn description(self, description: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("description", description.into()),
        }
    }

    pub fn icon_name(self, icon_name: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("icon-name", icon_name.into()),
        }
    }

    pub fn paintable(self, paintable: &impl IsA<gdk::Paintable>) -> Self {
        Self {
            builder: self
                .builder
                .property("paintable", paintable.clone().upcast()),
        }
    }

    pub fn title(self, title: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("title", title.into()),
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
    /// Build the [`StatusPage`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> StatusPage {
        self.builder.build()
    }
}
