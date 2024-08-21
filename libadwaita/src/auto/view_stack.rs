// This file was generated by gir (https://github.com/gtk-rs/gir)
// from
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::{ffi, ViewStackPage};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;

glib::wrapper! {
    #[doc(alias = "AdwViewStack")]
    pub struct ViewStack(Object<ffi::AdwViewStack, ffi::AdwViewStackClass>) @extends gtk::Widget, @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;

    match fn {
        type_ => || ffi::adw_view_stack_get_type(),
    }
}

impl ViewStack {
    #[doc(alias = "adw_view_stack_new")]
    pub fn new() -> ViewStack {
        assert_initialized_main_thread!();
        unsafe { gtk::Widget::from_glib_none(ffi::adw_view_stack_new()).unsafe_cast() }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`ViewStack`] objects.
    ///
    /// This method returns an instance of [`ViewStackBuilder`](crate::builders::ViewStackBuilder) which can be used to create [`ViewStack`] objects.
    pub fn builder() -> ViewStackBuilder {
        ViewStackBuilder::new()
    }

    #[doc(alias = "adw_view_stack_add")]
    pub fn add(&self, child: &impl IsA<gtk::Widget>) -> ViewStackPage {
        unsafe {
            from_glib_none(ffi::adw_view_stack_add(
                self.to_glib_none().0,
                child.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "adw_view_stack_add_named")]
    pub fn add_named(&self, child: &impl IsA<gtk::Widget>, name: Option<&str>) -> ViewStackPage {
        unsafe {
            from_glib_none(ffi::adw_view_stack_add_named(
                self.to_glib_none().0,
                child.as_ref().to_glib_none().0,
                name.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "adw_view_stack_add_titled")]
    pub fn add_titled(
        &self,
        child: &impl IsA<gtk::Widget>,
        name: Option<&str>,
        title: &str,
    ) -> ViewStackPage {
        unsafe {
            from_glib_none(ffi::adw_view_stack_add_titled(
                self.to_glib_none().0,
                child.as_ref().to_glib_none().0,
                name.to_glib_none().0,
                title.to_glib_none().0,
            ))
        }
    }

    #[cfg(feature = "v1_2")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_2")))]
    #[doc(alias = "adw_view_stack_add_titled_with_icon")]
    pub fn add_titled_with_icon(
        &self,
        child: &impl IsA<gtk::Widget>,
        name: Option<&str>,
        title: &str,
        icon_name: &str,
    ) -> ViewStackPage {
        unsafe {
            from_glib_none(ffi::adw_view_stack_add_titled_with_icon(
                self.to_glib_none().0,
                child.as_ref().to_glib_none().0,
                name.to_glib_none().0,
                title.to_glib_none().0,
                icon_name.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "adw_view_stack_get_child_by_name")]
    #[doc(alias = "get_child_by_name")]
    pub fn child_by_name(&self, name: &str) -> Option<gtk::Widget> {
        unsafe {
            from_glib_none(ffi::adw_view_stack_get_child_by_name(
                self.to_glib_none().0,
                name.to_glib_none().0,
            ))
        }
    }

    #[cfg(feature = "v1_7")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_7")))]
    #[doc(alias = "adw_view_stack_get_enable_transitions")]
    #[doc(alias = "get_enable_transitions")]
    #[doc(alias = "enable-transitions")]
    pub fn enables_transitions(&self) -> bool {
        unsafe {
            from_glib(ffi::adw_view_stack_get_enable_transitions(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "adw_view_stack_get_hhomogeneous")]
    #[doc(alias = "get_hhomogeneous")]
    #[doc(alias = "hhomogeneous")]
    pub fn is_hhomogeneous(&self) -> bool {
        unsafe { from_glib(ffi::adw_view_stack_get_hhomogeneous(self.to_glib_none().0)) }
    }

    #[doc(alias = "adw_view_stack_get_page")]
    #[doc(alias = "get_page")]
    pub fn page(&self, child: &impl IsA<gtk::Widget>) -> ViewStackPage {
        unsafe {
            from_glib_none(ffi::adw_view_stack_get_page(
                self.to_glib_none().0,
                child.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "adw_view_stack_get_pages")]
    #[doc(alias = "get_pages")]
    pub fn pages(&self) -> gtk::SelectionModel {
        unsafe { from_glib_full(ffi::adw_view_stack_get_pages(self.to_glib_none().0)) }
    }

    #[cfg(feature = "v1_7")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_7")))]
    #[doc(alias = "adw_view_stack_get_transition_duration")]
    #[doc(alias = "get_transition_duration")]
    #[doc(alias = "transition-duration")]
    pub fn transition_duration(&self) -> u32 {
        unsafe { ffi::adw_view_stack_get_transition_duration(self.to_glib_none().0) }
    }

    #[cfg(feature = "v1_7")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_7")))]
    #[doc(alias = "adw_view_stack_get_transition_running")]
    #[doc(alias = "get_transition_running")]
    #[doc(alias = "transition-running")]
    pub fn is_transition_running(&self) -> bool {
        unsafe {
            from_glib(ffi::adw_view_stack_get_transition_running(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "adw_view_stack_get_vhomogeneous")]
    #[doc(alias = "get_vhomogeneous")]
    #[doc(alias = "vhomogeneous")]
    pub fn is_vhomogeneous(&self) -> bool {
        unsafe { from_glib(ffi::adw_view_stack_get_vhomogeneous(self.to_glib_none().0)) }
    }

    #[doc(alias = "adw_view_stack_get_visible_child")]
    #[doc(alias = "get_visible_child")]
    #[doc(alias = "visible-child")]
    pub fn visible_child(&self) -> Option<gtk::Widget> {
        unsafe { from_glib_none(ffi::adw_view_stack_get_visible_child(self.to_glib_none().0)) }
    }

    #[doc(alias = "adw_view_stack_get_visible_child_name")]
    #[doc(alias = "get_visible_child_name")]
    #[doc(alias = "visible-child-name")]
    pub fn visible_child_name(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::adw_view_stack_get_visible_child_name(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "adw_view_stack_remove")]
    pub fn remove(&self, child: &impl IsA<gtk::Widget>) {
        unsafe {
            ffi::adw_view_stack_remove(self.to_glib_none().0, child.as_ref().to_glib_none().0);
        }
    }

    #[cfg(feature = "v1_7")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_7")))]
    #[doc(alias = "adw_view_stack_set_enable_transitions")]
    #[doc(alias = "enable-transitions")]
    pub fn set_enable_transitions(&self, enable_transitions: bool) {
        unsafe {
            ffi::adw_view_stack_set_enable_transitions(
                self.to_glib_none().0,
                enable_transitions.into_glib(),
            );
        }
    }

    #[doc(alias = "adw_view_stack_set_hhomogeneous")]
    #[doc(alias = "hhomogeneous")]
    pub fn set_hhomogeneous(&self, hhomogeneous: bool) {
        unsafe {
            ffi::adw_view_stack_set_hhomogeneous(self.to_glib_none().0, hhomogeneous.into_glib());
        }
    }

    #[cfg(feature = "v1_7")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_7")))]
    #[doc(alias = "adw_view_stack_set_transition_duration")]
    #[doc(alias = "transition-duration")]
    pub fn set_transition_duration(&self, duration: u32) {
        unsafe {
            ffi::adw_view_stack_set_transition_duration(self.to_glib_none().0, duration);
        }
    }

    #[doc(alias = "adw_view_stack_set_vhomogeneous")]
    #[doc(alias = "vhomogeneous")]
    pub fn set_vhomogeneous(&self, vhomogeneous: bool) {
        unsafe {
            ffi::adw_view_stack_set_vhomogeneous(self.to_glib_none().0, vhomogeneous.into_glib());
        }
    }

    #[doc(alias = "adw_view_stack_set_visible_child")]
    #[doc(alias = "visible-child")]
    pub fn set_visible_child(&self, child: &impl IsA<gtk::Widget>) {
        unsafe {
            ffi::adw_view_stack_set_visible_child(
                self.to_glib_none().0,
                child.as_ref().to_glib_none().0,
            );
        }
    }

    #[doc(alias = "adw_view_stack_set_visible_child_name")]
    #[doc(alias = "visible-child-name")]
    pub fn set_visible_child_name(&self, name: &str) {
        unsafe {
            ffi::adw_view_stack_set_visible_child_name(
                self.to_glib_none().0,
                name.to_glib_none().0,
            );
        }
    }

    #[cfg(feature = "v1_7")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_7")))]
    #[doc(alias = "enable-transitions")]
    pub fn connect_enable_transitions_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_enable_transitions_trampoline<F: Fn(&ViewStack) + 'static>(
            this: *mut ffi::AdwViewStack,
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
                b"notify::enable-transitions\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_enable_transitions_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "hhomogeneous")]
    pub fn connect_hhomogeneous_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_hhomogeneous_trampoline<F: Fn(&ViewStack) + 'static>(
            this: *mut ffi::AdwViewStack,
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
                b"notify::hhomogeneous\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_hhomogeneous_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "pages")]
    pub fn connect_pages_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_pages_trampoline<F: Fn(&ViewStack) + 'static>(
            this: *mut ffi::AdwViewStack,
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
                b"notify::pages\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_pages_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v1_7")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_7")))]
    #[doc(alias = "transition-duration")]
    pub fn connect_transition_duration_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_transition_duration_trampoline<F: Fn(&ViewStack) + 'static>(
            this: *mut ffi::AdwViewStack,
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
                b"notify::transition-duration\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_transition_duration_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v1_7")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_7")))]
    #[doc(alias = "transition-running")]
    pub fn connect_transition_running_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_transition_running_trampoline<F: Fn(&ViewStack) + 'static>(
            this: *mut ffi::AdwViewStack,
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
                b"notify::transition-running\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_transition_running_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "vhomogeneous")]
    pub fn connect_vhomogeneous_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_vhomogeneous_trampoline<F: Fn(&ViewStack) + 'static>(
            this: *mut ffi::AdwViewStack,
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
                b"notify::vhomogeneous\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_vhomogeneous_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "visible-child")]
    pub fn connect_visible_child_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_visible_child_trampoline<F: Fn(&ViewStack) + 'static>(
            this: *mut ffi::AdwViewStack,
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
                b"notify::visible-child\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_visible_child_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "visible-child-name")]
    pub fn connect_visible_child_name_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_visible_child_name_trampoline<F: Fn(&ViewStack) + 'static>(
            this: *mut ffi::AdwViewStack,
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
                b"notify::visible-child-name\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_visible_child_name_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl Default for ViewStack {
    fn default() -> Self {
        Self::new()
    }
}

// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`ViewStack`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct ViewStackBuilder {
    builder: glib::object::ObjectBuilder<'static, ViewStack>,
}

impl ViewStackBuilder {
    fn new() -> Self {
        Self {
            builder: glib::object::Object::builder(),
        }
    }

    #[cfg(feature = "v1_7")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_7")))]
    pub fn enable_transitions(self, enable_transitions: bool) -> Self {
        Self {
            builder: self
                .builder
                .property("enable-transitions", enable_transitions),
        }
    }

    pub fn hhomogeneous(self, hhomogeneous: bool) -> Self {
        Self {
            builder: self.builder.property("hhomogeneous", hhomogeneous),
        }
    }

    #[cfg(feature = "v1_7")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_7")))]
    pub fn transition_duration(self, transition_duration: u32) -> Self {
        Self {
            builder: self
                .builder
                .property("transition-duration", transition_duration),
        }
    }

    pub fn vhomogeneous(self, vhomogeneous: bool) -> Self {
        Self {
            builder: self.builder.property("vhomogeneous", vhomogeneous),
        }
    }

    pub fn visible_child(self, visible_child: &impl IsA<gtk::Widget>) -> Self {
        Self {
            builder: self
                .builder
                .property("visible-child", visible_child.clone().upcast()),
        }
    }

    pub fn visible_child_name(self, visible_child_name: impl Into<glib::GString>) -> Self {
        Self {
            builder: self
                .builder
                .property("visible-child-name", visible_child_name.into()),
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
    /// Build the [`ViewStack`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> ViewStack {
        self.builder.build()
    }
}
