// This file was generated by gir (https://github.com/gtk-rs/gir)
// from
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT
#![allow(deprecated)]

use crate::{ffi, ResponseAppearance};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;

glib::wrapper! {
    #[doc(alias = "AdwMessageDialog")]
    pub struct MessageDialog(Object<ffi::AdwMessageDialog, ffi::AdwMessageDialogClass>) @extends gtk::Window, gtk::Widget, @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;

    match fn {
        type_ => || ffi::adw_message_dialog_get_type(),
    }
}

impl MessageDialog {
    pub const NONE: Option<&'static MessageDialog> = None;

    #[cfg_attr(feature = "v1_6", deprecated = "Since 1.6")]
    #[allow(deprecated)]
    #[doc(alias = "adw_message_dialog_new")]
    pub fn new(
        parent: Option<&impl IsA<gtk::Window>>,
        heading: Option<&str>,
        body: Option<&str>,
    ) -> MessageDialog {
        assert_initialized_main_thread!();
        unsafe {
            gtk::Widget::from_glib_none(ffi::adw_message_dialog_new(
                parent.map(|p| p.as_ref()).to_glib_none().0,
                heading.to_glib_none().0,
                body.to_glib_none().0,
            ))
            .unsafe_cast()
        }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`MessageDialog`] objects.
    ///
    /// This method returns an instance of [`MessageDialogBuilder`](crate::builders::MessageDialogBuilder) which can be used to create [`MessageDialog`] objects.
    pub fn builder() -> MessageDialogBuilder {
        MessageDialogBuilder::new()
    }
}

#[cfg(feature = "v1_2")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_2")))]
impl Default for MessageDialog {
    fn default() -> Self {
        glib::object::Object::new::<Self>()
    }
}

// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`MessageDialog`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct MessageDialogBuilder {
    builder: glib::object::ObjectBuilder<'static, MessageDialog>,
}

impl MessageDialogBuilder {
    fn new() -> Self {
        Self {
            builder: glib::object::Object::builder(),
        }
    }

    #[cfg(feature = "v1_2")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_2")))]
    #[cfg_attr(feature = "v1_6", deprecated = "Since 1.6")]
    pub fn body(self, body: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("body", body.into()),
        }
    }

    #[cfg(feature = "v1_2")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_2")))]
    #[cfg_attr(feature = "v1_6", deprecated = "Since 1.6")]
    pub fn body_use_markup(self, body_use_markup: bool) -> Self {
        Self {
            builder: self.builder.property("body-use-markup", body_use_markup),
        }
    }

    #[cfg(feature = "v1_2")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_2")))]
    #[cfg_attr(feature = "v1_6", deprecated = "Since 1.6")]
    pub fn close_response(self, close_response: impl Into<glib::GString>) -> Self {
        Self {
            builder: self
                .builder
                .property("close-response", close_response.into()),
        }
    }

    #[cfg(feature = "v1_2")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_2")))]
    #[cfg_attr(feature = "v1_6", deprecated = "Since 1.6")]
    pub fn default_response(self, default_response: impl Into<glib::GString>) -> Self {
        Self {
            builder: self
                .builder
                .property("default-response", default_response.into()),
        }
    }

    #[cfg(feature = "v1_2")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_2")))]
    #[cfg_attr(feature = "v1_6", deprecated = "Since 1.6")]
    pub fn extra_child(self, extra_child: &impl IsA<gtk::Widget>) -> Self {
        Self {
            builder: self
                .builder
                .property("extra-child", extra_child.clone().upcast()),
        }
    }

    #[cfg(feature = "v1_2")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_2")))]
    #[cfg_attr(feature = "v1_6", deprecated = "Since 1.6")]
    pub fn heading(self, heading: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("heading", heading.into()),
        }
    }

    #[cfg(feature = "v1_2")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_2")))]
    #[cfg_attr(feature = "v1_6", deprecated = "Since 1.6")]
    pub fn heading_use_markup(self, heading_use_markup: bool) -> Self {
        Self {
            builder: self
                .builder
                .property("heading-use-markup", heading_use_markup),
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
    /// Build the [`MessageDialog`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> MessageDialog {
        assert_initialized_main_thread!();
        self.builder.build()
    }
}

pub trait MessageDialogExt: IsA<MessageDialog> + 'static {
    #[cfg_attr(feature = "v1_6", deprecated = "Since 1.6")]
    #[allow(deprecated)]
    #[doc(alias = "adw_message_dialog_add_response")]
    fn add_response(&self, id: &str, label: &str) {
        unsafe {
            ffi::adw_message_dialog_add_response(
                self.as_ref().to_glib_none().0,
                id.to_glib_none().0,
                label.to_glib_none().0,
            );
        }
    }

    #[cfg_attr(feature = "v1_6", deprecated = "Since 1.6")]
    #[allow(deprecated)]
    #[doc(alias = "adw_message_dialog_get_body")]
    #[doc(alias = "get_body")]
    fn body(&self) -> glib::GString {
        unsafe {
            from_glib_none(ffi::adw_message_dialog_get_body(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg_attr(feature = "v1_6", deprecated = "Since 1.6")]
    #[allow(deprecated)]
    #[doc(alias = "adw_message_dialog_get_body_use_markup")]
    #[doc(alias = "get_body_use_markup")]
    #[doc(alias = "body-use-markup")]
    fn is_body_use_markup(&self) -> bool {
        unsafe {
            from_glib(ffi::adw_message_dialog_get_body_use_markup(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg_attr(feature = "v1_6", deprecated = "Since 1.6")]
    #[allow(deprecated)]
    #[doc(alias = "adw_message_dialog_get_close_response")]
    #[doc(alias = "get_close_response")]
    #[doc(alias = "close-response")]
    fn close_response(&self) -> glib::GString {
        unsafe {
            from_glib_none(ffi::adw_message_dialog_get_close_response(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg_attr(feature = "v1_6", deprecated = "Since 1.6")]
    #[allow(deprecated)]
    #[doc(alias = "adw_message_dialog_get_default_response")]
    #[doc(alias = "get_default_response")]
    #[doc(alias = "default-response")]
    fn default_response(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::adw_message_dialog_get_default_response(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg_attr(feature = "v1_6", deprecated = "Since 1.6")]
    #[allow(deprecated)]
    #[doc(alias = "adw_message_dialog_get_extra_child")]
    #[doc(alias = "get_extra_child")]
    #[doc(alias = "extra-child")]
    fn extra_child(&self) -> Option<gtk::Widget> {
        unsafe {
            from_glib_none(ffi::adw_message_dialog_get_extra_child(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg_attr(feature = "v1_6", deprecated = "Since 1.6")]
    #[allow(deprecated)]
    #[doc(alias = "adw_message_dialog_get_heading")]
    #[doc(alias = "get_heading")]
    fn heading(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::adw_message_dialog_get_heading(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg_attr(feature = "v1_6", deprecated = "Since 1.6")]
    #[allow(deprecated)]
    #[doc(alias = "adw_message_dialog_get_heading_use_markup")]
    #[doc(alias = "get_heading_use_markup")]
    #[doc(alias = "heading-use-markup")]
    fn is_heading_use_markup(&self) -> bool {
        unsafe {
            from_glib(ffi::adw_message_dialog_get_heading_use_markup(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg_attr(feature = "v1_6", deprecated = "Since 1.6")]
    #[allow(deprecated)]
    #[doc(alias = "adw_message_dialog_get_response_appearance")]
    #[doc(alias = "get_response_appearance")]
    fn response_appearance(&self, response: &str) -> ResponseAppearance {
        unsafe {
            from_glib(ffi::adw_message_dialog_get_response_appearance(
                self.as_ref().to_glib_none().0,
                response.to_glib_none().0,
            ))
        }
    }

    #[cfg_attr(feature = "v1_6", deprecated = "Since 1.6")]
    #[allow(deprecated)]
    #[doc(alias = "adw_message_dialog_get_response_enabled")]
    #[doc(alias = "get_response_enabled")]
    fn is_response_enabled(&self, response: &str) -> bool {
        unsafe {
            from_glib(ffi::adw_message_dialog_get_response_enabled(
                self.as_ref().to_glib_none().0,
                response.to_glib_none().0,
            ))
        }
    }

    #[cfg_attr(feature = "v1_6", deprecated = "Since 1.6")]
    #[allow(deprecated)]
    #[doc(alias = "adw_message_dialog_has_response")]
    fn has_response(&self, response: &str) -> bool {
        unsafe {
            from_glib(ffi::adw_message_dialog_has_response(
                self.as_ref().to_glib_none().0,
                response.to_glib_none().0,
            ))
        }
    }

    #[cfg_attr(feature = "v1_6", deprecated = "Since 1.6")]
    #[cfg(feature = "v1_5")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_5")))]
    #[allow(deprecated)]
    #[doc(alias = "adw_message_dialog_remove_response")]
    fn remove_response(&self, id: &str) {
        unsafe {
            ffi::adw_message_dialog_remove_response(
                self.as_ref().to_glib_none().0,
                id.to_glib_none().0,
            );
        }
    }

    #[cfg_attr(feature = "v1_6", deprecated = "Since 1.6")]
    #[allow(deprecated)]
    #[doc(alias = "adw_message_dialog_response")]
    fn response(&self, response: &str) {
        unsafe {
            ffi::adw_message_dialog_response(
                self.as_ref().to_glib_none().0,
                response.to_glib_none().0,
            );
        }
    }

    #[cfg_attr(feature = "v1_6", deprecated = "Since 1.6")]
    #[allow(deprecated)]
    #[doc(alias = "adw_message_dialog_set_body")]
    #[doc(alias = "body")]
    fn set_body(&self, body: &str) {
        unsafe {
            ffi::adw_message_dialog_set_body(self.as_ref().to_glib_none().0, body.to_glib_none().0);
        }
    }

    #[cfg_attr(feature = "v1_6", deprecated = "Since 1.6")]
    #[allow(deprecated)]
    #[doc(alias = "adw_message_dialog_set_body_use_markup")]
    #[doc(alias = "body-use-markup")]
    fn set_body_use_markup(&self, use_markup: bool) {
        unsafe {
            ffi::adw_message_dialog_set_body_use_markup(
                self.as_ref().to_glib_none().0,
                use_markup.into_glib(),
            );
        }
    }

    #[cfg_attr(feature = "v1_6", deprecated = "Since 1.6")]
    #[allow(deprecated)]
    #[doc(alias = "adw_message_dialog_set_close_response")]
    #[doc(alias = "close-response")]
    fn set_close_response(&self, response: &str) {
        unsafe {
            ffi::adw_message_dialog_set_close_response(
                self.as_ref().to_glib_none().0,
                response.to_glib_none().0,
            );
        }
    }

    #[cfg_attr(feature = "v1_6", deprecated = "Since 1.6")]
    #[allow(deprecated)]
    #[doc(alias = "adw_message_dialog_set_default_response")]
    #[doc(alias = "default-response")]
    fn set_default_response(&self, response: Option<&str>) {
        unsafe {
            ffi::adw_message_dialog_set_default_response(
                self.as_ref().to_glib_none().0,
                response.to_glib_none().0,
            );
        }
    }

    #[cfg_attr(feature = "v1_6", deprecated = "Since 1.6")]
    #[allow(deprecated)]
    #[doc(alias = "adw_message_dialog_set_extra_child")]
    #[doc(alias = "extra-child")]
    fn set_extra_child(&self, child: Option<&impl IsA<gtk::Widget>>) {
        unsafe {
            ffi::adw_message_dialog_set_extra_child(
                self.as_ref().to_glib_none().0,
                child.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    #[cfg_attr(feature = "v1_6", deprecated = "Since 1.6")]
    #[allow(deprecated)]
    #[doc(alias = "adw_message_dialog_set_heading")]
    #[doc(alias = "heading")]
    fn set_heading(&self, heading: Option<&str>) {
        unsafe {
            ffi::adw_message_dialog_set_heading(
                self.as_ref().to_glib_none().0,
                heading.to_glib_none().0,
            );
        }
    }

    #[cfg_attr(feature = "v1_6", deprecated = "Since 1.6")]
    #[allow(deprecated)]
    #[doc(alias = "adw_message_dialog_set_heading_use_markup")]
    #[doc(alias = "heading-use-markup")]
    fn set_heading_use_markup(&self, use_markup: bool) {
        unsafe {
            ffi::adw_message_dialog_set_heading_use_markup(
                self.as_ref().to_glib_none().0,
                use_markup.into_glib(),
            );
        }
    }

    #[cfg_attr(feature = "v1_6", deprecated = "Since 1.6")]
    #[allow(deprecated)]
    #[doc(alias = "adw_message_dialog_set_response_appearance")]
    fn set_response_appearance(&self, response: &str, appearance: ResponseAppearance) {
        unsafe {
            ffi::adw_message_dialog_set_response_appearance(
                self.as_ref().to_glib_none().0,
                response.to_glib_none().0,
                appearance.into_glib(),
            );
        }
    }

    #[cfg_attr(feature = "v1_6", deprecated = "Since 1.6")]
    #[allow(deprecated)]
    #[doc(alias = "adw_message_dialog_set_response_enabled")]
    fn set_response_enabled(&self, response: &str, enabled: bool) {
        unsafe {
            ffi::adw_message_dialog_set_response_enabled(
                self.as_ref().to_glib_none().0,
                response.to_glib_none().0,
                enabled.into_glib(),
            );
        }
    }

    #[cfg_attr(feature = "v1_6", deprecated = "Since 1.6")]
    #[allow(deprecated)]
    #[doc(alias = "adw_message_dialog_set_response_label")]
    fn set_response_label(&self, response: &str, label: &str) {
        unsafe {
            ffi::adw_message_dialog_set_response_label(
                self.as_ref().to_glib_none().0,
                response.to_glib_none().0,
                label.to_glib_none().0,
            );
        }
    }

    #[cfg_attr(feature = "v1_6", deprecated = "Since 1.6")]
    #[cfg(feature = "v1_2")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_2")))]
    #[doc(alias = "response")]
    fn connect_response<F: Fn(&Self, &str) + 'static>(
        &self,
        detail: Option<&str>,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn response_trampoline<
            P: IsA<MessageDialog>,
            F: Fn(&P, &str) + 'static,
        >(
            this: *mut ffi::AdwMessageDialog,
            response: *mut std::ffi::c_char,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                MessageDialog::from_glib_borrow(this).unsafe_cast_ref(),
                &glib::GString::from_glib_borrow(response),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            let detailed_signal_name = detail.map(|name| format!("response::{name}\0"));
            let signal_name: &[u8] = detailed_signal_name
                .as_ref()
                .map_or(&b"response\0"[..], |n| n.as_bytes());
            connect_raw(
                self.as_ptr() as *mut _,
                signal_name.as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    response_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg_attr(feature = "v1_6", deprecated = "Since 1.6")]
    #[cfg(feature = "v1_2")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_2")))]
    #[doc(alias = "body")]
    fn connect_body_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_body_trampoline<P: IsA<MessageDialog>, F: Fn(&P) + 'static>(
            this: *mut ffi::AdwMessageDialog,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(MessageDialog::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::body\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_body_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg_attr(feature = "v1_6", deprecated = "Since 1.6")]
    #[cfg(feature = "v1_2")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_2")))]
    #[doc(alias = "body-use-markup")]
    fn connect_body_use_markup_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_body_use_markup_trampoline<
            P: IsA<MessageDialog>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::AdwMessageDialog,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(MessageDialog::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::body-use-markup\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_body_use_markup_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg_attr(feature = "v1_6", deprecated = "Since 1.6")]
    #[cfg(feature = "v1_2")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_2")))]
    #[doc(alias = "close-response")]
    fn connect_close_response_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_close_response_trampoline<
            P: IsA<MessageDialog>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::AdwMessageDialog,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(MessageDialog::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::close-response\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_close_response_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg_attr(feature = "v1_6", deprecated = "Since 1.6")]
    #[cfg(feature = "v1_2")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_2")))]
    #[doc(alias = "default-response")]
    fn connect_default_response_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_default_response_trampoline<
            P: IsA<MessageDialog>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::AdwMessageDialog,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(MessageDialog::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::default-response\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_default_response_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg_attr(feature = "v1_6", deprecated = "Since 1.6")]
    #[cfg(feature = "v1_2")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_2")))]
    #[doc(alias = "extra-child")]
    fn connect_extra_child_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_extra_child_trampoline<
            P: IsA<MessageDialog>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::AdwMessageDialog,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(MessageDialog::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::extra-child\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_extra_child_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg_attr(feature = "v1_6", deprecated = "Since 1.6")]
    #[cfg(feature = "v1_2")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_2")))]
    #[doc(alias = "heading")]
    fn connect_heading_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_heading_trampoline<
            P: IsA<MessageDialog>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::AdwMessageDialog,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(MessageDialog::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::heading\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_heading_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg_attr(feature = "v1_6", deprecated = "Since 1.6")]
    #[cfg(feature = "v1_2")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_2")))]
    #[doc(alias = "heading-use-markup")]
    fn connect_heading_use_markup_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_heading_use_markup_trampoline<
            P: IsA<MessageDialog>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::AdwMessageDialog,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(MessageDialog::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::heading-use-markup\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_heading_use_markup_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl<O: IsA<MessageDialog>> MessageDialogExt for O {}
