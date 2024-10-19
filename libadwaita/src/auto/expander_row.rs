// This file was generated by gir (https://github.com/gtk-rs/gir)
// from
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT
#![allow(deprecated)]

use crate::{ffi, PreferencesRow};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;

glib::wrapper! {
    #[doc(alias = "AdwExpanderRow")]
    pub struct ExpanderRow(Object<ffi::AdwExpanderRow, ffi::AdwExpanderRowClass>) @extends PreferencesRow, gtk::ListBoxRow, gtk::Widget, @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Actionable;

    match fn {
        type_ => || ffi::adw_expander_row_get_type(),
    }
}

impl ExpanderRow {
    pub const NONE: Option<&'static ExpanderRow> = None;

    #[doc(alias = "adw_expander_row_new")]
    pub fn new() -> ExpanderRow {
        assert_initialized_main_thread!();
        unsafe { gtk::Widget::from_glib_none(ffi::adw_expander_row_new()).unsafe_cast() }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`ExpanderRow`] objects.
    ///
    /// This method returns an instance of [`ExpanderRowBuilder`](crate::builders::ExpanderRowBuilder) which can be used to create [`ExpanderRow`] objects.
    pub fn builder() -> ExpanderRowBuilder {
        ExpanderRowBuilder::new()
    }
}

impl Default for ExpanderRow {
    fn default() -> Self {
        Self::new()
    }
}

// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`ExpanderRow`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct ExpanderRowBuilder {
    builder: glib::object::ObjectBuilder<'static, ExpanderRow>,
}

impl ExpanderRowBuilder {
    fn new() -> Self {
        Self {
            builder: glib::object::Object::builder(),
        }
    }

    pub fn enable_expansion(self, enable_expansion: bool) -> Self {
        Self {
            builder: self.builder.property("enable-expansion", enable_expansion),
        }
    }

    pub fn expanded(self, expanded: bool) -> Self {
        Self {
            builder: self.builder.property("expanded", expanded),
        }
    }

    #[cfg_attr(feature = "v1_3", deprecated = "Since 1.3")]
    pub fn icon_name(self, icon_name: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("icon-name", icon_name.into()),
        }
    }

    pub fn show_enable_switch(self, show_enable_switch: bool) -> Self {
        Self {
            builder: self
                .builder
                .property("show-enable-switch", show_enable_switch),
        }
    }

    pub fn subtitle(self, subtitle: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("subtitle", subtitle.into()),
        }
    }

    #[cfg(feature = "v1_3")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_3")))]
    pub fn subtitle_lines(self, subtitle_lines: i32) -> Self {
        Self {
            builder: self.builder.property("subtitle-lines", subtitle_lines),
        }
    }

    #[cfg(feature = "v1_3")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_3")))]
    pub fn title_lines(self, title_lines: i32) -> Self {
        Self {
            builder: self.builder.property("title-lines", title_lines),
        }
    }

    pub fn title(self, title: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("title", title.into()),
        }
    }

    #[cfg(feature = "v1_1")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_1")))]
    pub fn title_selectable(self, title_selectable: bool) -> Self {
        Self {
            builder: self.builder.property("title-selectable", title_selectable),
        }
    }

    #[cfg(feature = "v1_2")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_2")))]
    pub fn use_markup(self, use_markup: bool) -> Self {
        Self {
            builder: self.builder.property("use-markup", use_markup),
        }
    }

    pub fn use_underline(self, use_underline: bool) -> Self {
        Self {
            builder: self.builder.property("use-underline", use_underline),
        }
    }

    pub fn activatable(self, activatable: bool) -> Self {
        Self {
            builder: self.builder.property("activatable", activatable),
        }
    }

    pub fn child(self, child: &impl IsA<gtk::Widget>) -> Self {
        Self {
            builder: self.builder.property("child", child.clone().upcast()),
        }
    }

    pub fn selectable(self, selectable: bool) -> Self {
        Self {
            builder: self.builder.property("selectable", selectable),
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

    pub fn action_name(self, action_name: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("action-name", action_name.into()),
        }
    }

    pub fn action_target(self, action_target: &glib::Variant) -> Self {
        Self {
            builder: self
                .builder
                .property("action-target", action_target.clone()),
        }
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`ExpanderRow`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> ExpanderRow {
        assert_initialized_main_thread!();
        self.builder.build()
    }
}

pub trait ExpanderRowExt: IsA<ExpanderRow> + 'static {
    #[cfg_attr(feature = "v1_4", deprecated = "Since 1.4")]
    #[allow(deprecated)]
    #[doc(alias = "adw_expander_row_add_action")]
    fn add_action(&self, widget: &impl IsA<gtk::Widget>) {
        unsafe {
            ffi::adw_expander_row_add_action(
                self.as_ref().to_glib_none().0,
                widget.as_ref().to_glib_none().0,
            );
        }
    }

    #[doc(alias = "adw_expander_row_add_prefix")]
    fn add_prefix(&self, widget: &impl IsA<gtk::Widget>) {
        unsafe {
            ffi::adw_expander_row_add_prefix(
                self.as_ref().to_glib_none().0,
                widget.as_ref().to_glib_none().0,
            );
        }
    }

    #[doc(alias = "adw_expander_row_add_row")]
    fn add_row(&self, child: &impl IsA<gtk::Widget>) {
        unsafe {
            ffi::adw_expander_row_add_row(
                self.as_ref().to_glib_none().0,
                child.as_ref().to_glib_none().0,
            );
        }
    }

    #[cfg(feature = "v1_4")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_4")))]
    #[doc(alias = "adw_expander_row_add_suffix")]
    fn add_suffix(&self, widget: &impl IsA<gtk::Widget>) {
        unsafe {
            ffi::adw_expander_row_add_suffix(
                self.as_ref().to_glib_none().0,
                widget.as_ref().to_glib_none().0,
            );
        }
    }

    #[doc(alias = "adw_expander_row_get_enable_expansion")]
    #[doc(alias = "get_enable_expansion")]
    #[doc(alias = "enable-expansion")]
    fn enables_expansion(&self) -> bool {
        unsafe {
            from_glib(ffi::adw_expander_row_get_enable_expansion(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "adw_expander_row_get_expanded")]
    #[doc(alias = "get_expanded")]
    #[doc(alias = "expanded")]
    fn is_expanded(&self) -> bool {
        unsafe {
            from_glib(ffi::adw_expander_row_get_expanded(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg_attr(feature = "v1_3", deprecated = "Since 1.3")]
    #[allow(deprecated)]
    #[doc(alias = "adw_expander_row_get_icon_name")]
    #[doc(alias = "get_icon_name")]
    #[doc(alias = "icon-name")]
    fn icon_name(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::adw_expander_row_get_icon_name(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "adw_expander_row_get_show_enable_switch")]
    #[doc(alias = "get_show_enable_switch")]
    #[doc(alias = "show-enable-switch")]
    fn shows_enable_switch(&self) -> bool {
        unsafe {
            from_glib(ffi::adw_expander_row_get_show_enable_switch(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "adw_expander_row_get_subtitle")]
    #[doc(alias = "get_subtitle")]
    fn subtitle(&self) -> glib::GString {
        unsafe {
            from_glib_none(ffi::adw_expander_row_get_subtitle(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(feature = "v1_3")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_3")))]
    #[doc(alias = "adw_expander_row_get_subtitle_lines")]
    #[doc(alias = "get_subtitle_lines")]
    #[doc(alias = "subtitle-lines")]
    fn subtitle_lines(&self) -> i32 {
        unsafe { ffi::adw_expander_row_get_subtitle_lines(self.as_ref().to_glib_none().0) }
    }

    #[cfg(feature = "v1_3")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_3")))]
    #[doc(alias = "adw_expander_row_get_title_lines")]
    #[doc(alias = "get_title_lines")]
    #[doc(alias = "title-lines")]
    fn title_lines(&self) -> i32 {
        unsafe { ffi::adw_expander_row_get_title_lines(self.as_ref().to_glib_none().0) }
    }

    #[doc(alias = "adw_expander_row_remove")]
    fn remove(&self, child: &impl IsA<gtk::Widget>) {
        unsafe {
            ffi::adw_expander_row_remove(
                self.as_ref().to_glib_none().0,
                child.as_ref().to_glib_none().0,
            );
        }
    }

    #[doc(alias = "adw_expander_row_set_enable_expansion")]
    #[doc(alias = "enable-expansion")]
    fn set_enable_expansion(&self, enable_expansion: bool) {
        unsafe {
            ffi::adw_expander_row_set_enable_expansion(
                self.as_ref().to_glib_none().0,
                enable_expansion.into_glib(),
            );
        }
    }

    #[doc(alias = "adw_expander_row_set_expanded")]
    #[doc(alias = "expanded")]
    fn set_expanded(&self, expanded: bool) {
        unsafe {
            ffi::adw_expander_row_set_expanded(
                self.as_ref().to_glib_none().0,
                expanded.into_glib(),
            );
        }
    }

    #[cfg_attr(feature = "v1_3", deprecated = "Since 1.3")]
    #[allow(deprecated)]
    #[doc(alias = "adw_expander_row_set_icon_name")]
    #[doc(alias = "icon-name")]
    fn set_icon_name(&self, icon_name: Option<&str>) {
        unsafe {
            ffi::adw_expander_row_set_icon_name(
                self.as_ref().to_glib_none().0,
                icon_name.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "adw_expander_row_set_show_enable_switch")]
    #[doc(alias = "show-enable-switch")]
    fn set_show_enable_switch(&self, show_enable_switch: bool) {
        unsafe {
            ffi::adw_expander_row_set_show_enable_switch(
                self.as_ref().to_glib_none().0,
                show_enable_switch.into_glib(),
            );
        }
    }

    #[doc(alias = "adw_expander_row_set_subtitle")]
    #[doc(alias = "subtitle")]
    fn set_subtitle(&self, subtitle: &str) {
        unsafe {
            ffi::adw_expander_row_set_subtitle(
                self.as_ref().to_glib_none().0,
                subtitle.to_glib_none().0,
            );
        }
    }

    #[cfg(feature = "v1_3")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_3")))]
    #[doc(alias = "adw_expander_row_set_subtitle_lines")]
    #[doc(alias = "subtitle-lines")]
    fn set_subtitle_lines(&self, subtitle_lines: i32) {
        unsafe {
            ffi::adw_expander_row_set_subtitle_lines(
                self.as_ref().to_glib_none().0,
                subtitle_lines,
            );
        }
    }

    #[cfg(feature = "v1_3")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_3")))]
    #[doc(alias = "adw_expander_row_set_title_lines")]
    #[doc(alias = "title-lines")]
    fn set_title_lines(&self, title_lines: i32) {
        unsafe {
            ffi::adw_expander_row_set_title_lines(self.as_ref().to_glib_none().0, title_lines);
        }
    }

    #[doc(alias = "enable-expansion")]
    fn connect_enable_expansion_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_enable_expansion_trampoline<
            P: IsA<ExpanderRow>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::AdwExpanderRow,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(ExpanderRow::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::enable-expansion\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_enable_expansion_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "expanded")]
    fn connect_expanded_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_expanded_trampoline<
            P: IsA<ExpanderRow>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::AdwExpanderRow,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(ExpanderRow::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::expanded\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_expanded_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg_attr(feature = "v1_3", deprecated = "Since 1.3")]
    #[doc(alias = "icon-name")]
    fn connect_icon_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_icon_name_trampoline<
            P: IsA<ExpanderRow>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::AdwExpanderRow,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(ExpanderRow::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::icon-name\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_icon_name_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "show-enable-switch")]
    fn connect_show_enable_switch_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_show_enable_switch_trampoline<
            P: IsA<ExpanderRow>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::AdwExpanderRow,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(ExpanderRow::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::show-enable-switch\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_show_enable_switch_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "subtitle")]
    fn connect_subtitle_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_subtitle_trampoline<
            P: IsA<ExpanderRow>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::AdwExpanderRow,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(ExpanderRow::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::subtitle\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_subtitle_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v1_3")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_3")))]
    #[doc(alias = "subtitle-lines")]
    fn connect_subtitle_lines_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_subtitle_lines_trampoline<
            P: IsA<ExpanderRow>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::AdwExpanderRow,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(ExpanderRow::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::subtitle-lines\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_subtitle_lines_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v1_3")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_3")))]
    #[doc(alias = "title-lines")]
    fn connect_title_lines_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_title_lines_trampoline<
            P: IsA<ExpanderRow>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::AdwExpanderRow,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(ExpanderRow::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::title-lines\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_title_lines_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl<O: IsA<ExpanderRow>> ExpanderRowExt for O {}
