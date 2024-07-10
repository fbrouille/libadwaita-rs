// This file was generated by gir (https://github.com/gtk-rs/gir)
// from
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::{ffi, ToolbarStyle};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;

glib::wrapper! {
    #[doc(alias = "AdwToolbarView")]
    pub struct ToolbarView(Object<ffi::AdwToolbarView, ffi::AdwToolbarViewClass>) @extends gtk::Widget, @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;

    match fn {
        type_ => || ffi::adw_toolbar_view_get_type(),
    }
}

impl ToolbarView {
    #[doc(alias = "adw_toolbar_view_new")]
    pub fn new() -> ToolbarView {
        assert_initialized_main_thread!();
        unsafe { gtk::Widget::from_glib_none(ffi::adw_toolbar_view_new()).unsafe_cast() }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`ToolbarView`] objects.
    ///
    /// This method returns an instance of [`ToolbarViewBuilder`](crate::builders::ToolbarViewBuilder) which can be used to create [`ToolbarView`] objects.
    pub fn builder() -> ToolbarViewBuilder {
        ToolbarViewBuilder::new()
    }

    #[doc(alias = "adw_toolbar_view_add_bottom_bar")]
    pub fn add_bottom_bar(&self, widget: &impl IsA<gtk::Widget>) {
        unsafe {
            ffi::adw_toolbar_view_add_bottom_bar(
                self.to_glib_none().0,
                widget.as_ref().to_glib_none().0,
            );
        }
    }

    #[doc(alias = "adw_toolbar_view_add_top_bar")]
    pub fn add_top_bar(&self, widget: &impl IsA<gtk::Widget>) {
        unsafe {
            ffi::adw_toolbar_view_add_top_bar(
                self.to_glib_none().0,
                widget.as_ref().to_glib_none().0,
            );
        }
    }

    #[doc(alias = "adw_toolbar_view_get_bottom_bar_height")]
    #[doc(alias = "get_bottom_bar_height")]
    #[doc(alias = "bottom-bar-height")]
    pub fn bottom_bar_height(&self) -> i32 {
        unsafe { ffi::adw_toolbar_view_get_bottom_bar_height(self.to_glib_none().0) }
    }

    #[doc(alias = "adw_toolbar_view_get_bottom_bar_style")]
    #[doc(alias = "get_bottom_bar_style")]
    #[doc(alias = "bottom-bar-style")]
    pub fn bottom_bar_style(&self) -> ToolbarStyle {
        unsafe {
            from_glib(ffi::adw_toolbar_view_get_bottom_bar_style(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "adw_toolbar_view_get_content")]
    #[doc(alias = "get_content")]
    pub fn content(&self) -> Option<gtk::Widget> {
        unsafe { from_glib_none(ffi::adw_toolbar_view_get_content(self.to_glib_none().0)) }
    }

    #[doc(alias = "adw_toolbar_view_get_extend_content_to_bottom_edge")]
    #[doc(alias = "get_extend_content_to_bottom_edge")]
    #[doc(alias = "extend-content-to-bottom-edge")]
    pub fn is_extend_content_to_bottom_edge(&self) -> bool {
        unsafe {
            from_glib(ffi::adw_toolbar_view_get_extend_content_to_bottom_edge(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "adw_toolbar_view_get_extend_content_to_top_edge")]
    #[doc(alias = "get_extend_content_to_top_edge")]
    #[doc(alias = "extend-content-to-top-edge")]
    pub fn is_extend_content_to_top_edge(&self) -> bool {
        unsafe {
            from_glib(ffi::adw_toolbar_view_get_extend_content_to_top_edge(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "adw_toolbar_view_get_reveal_bottom_bars")]
    #[doc(alias = "get_reveal_bottom_bars")]
    #[doc(alias = "reveal-bottom-bars")]
    pub fn reveals_bottom_bars(&self) -> bool {
        unsafe {
            from_glib(ffi::adw_toolbar_view_get_reveal_bottom_bars(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "adw_toolbar_view_get_reveal_top_bars")]
    #[doc(alias = "get_reveal_top_bars")]
    #[doc(alias = "reveal-top-bars")]
    pub fn reveals_top_bars(&self) -> bool {
        unsafe {
            from_glib(ffi::adw_toolbar_view_get_reveal_top_bars(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "adw_toolbar_view_get_top_bar_height")]
    #[doc(alias = "get_top_bar_height")]
    #[doc(alias = "top-bar-height")]
    pub fn top_bar_height(&self) -> i32 {
        unsafe { ffi::adw_toolbar_view_get_top_bar_height(self.to_glib_none().0) }
    }

    #[doc(alias = "adw_toolbar_view_get_top_bar_style")]
    #[doc(alias = "get_top_bar_style")]
    #[doc(alias = "top-bar-style")]
    pub fn top_bar_style(&self) -> ToolbarStyle {
        unsafe {
            from_glib(ffi::adw_toolbar_view_get_top_bar_style(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "adw_toolbar_view_remove")]
    pub fn remove(&self, widget: &impl IsA<gtk::Widget>) {
        unsafe {
            ffi::adw_toolbar_view_remove(self.to_glib_none().0, widget.as_ref().to_glib_none().0);
        }
    }

    #[doc(alias = "adw_toolbar_view_set_bottom_bar_style")]
    #[doc(alias = "bottom-bar-style")]
    pub fn set_bottom_bar_style(&self, style: ToolbarStyle) {
        unsafe {
            ffi::adw_toolbar_view_set_bottom_bar_style(self.to_glib_none().0, style.into_glib());
        }
    }

    #[doc(alias = "adw_toolbar_view_set_content")]
    #[doc(alias = "content")]
    pub fn set_content(&self, content: Option<&impl IsA<gtk::Widget>>) {
        unsafe {
            ffi::adw_toolbar_view_set_content(
                self.to_glib_none().0,
                content.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    #[doc(alias = "adw_toolbar_view_set_extend_content_to_bottom_edge")]
    #[doc(alias = "extend-content-to-bottom-edge")]
    pub fn set_extend_content_to_bottom_edge(&self, extend: bool) {
        unsafe {
            ffi::adw_toolbar_view_set_extend_content_to_bottom_edge(
                self.to_glib_none().0,
                extend.into_glib(),
            );
        }
    }

    #[doc(alias = "adw_toolbar_view_set_extend_content_to_top_edge")]
    #[doc(alias = "extend-content-to-top-edge")]
    pub fn set_extend_content_to_top_edge(&self, extend: bool) {
        unsafe {
            ffi::adw_toolbar_view_set_extend_content_to_top_edge(
                self.to_glib_none().0,
                extend.into_glib(),
            );
        }
    }

    #[doc(alias = "adw_toolbar_view_set_reveal_bottom_bars")]
    #[doc(alias = "reveal-bottom-bars")]
    pub fn set_reveal_bottom_bars(&self, reveal: bool) {
        unsafe {
            ffi::adw_toolbar_view_set_reveal_bottom_bars(self.to_glib_none().0, reveal.into_glib());
        }
    }

    #[doc(alias = "adw_toolbar_view_set_reveal_top_bars")]
    #[doc(alias = "reveal-top-bars")]
    pub fn set_reveal_top_bars(&self, reveal: bool) {
        unsafe {
            ffi::adw_toolbar_view_set_reveal_top_bars(self.to_glib_none().0, reveal.into_glib());
        }
    }

    #[doc(alias = "adw_toolbar_view_set_top_bar_style")]
    #[doc(alias = "top-bar-style")]
    pub fn set_top_bar_style(&self, style: ToolbarStyle) {
        unsafe {
            ffi::adw_toolbar_view_set_top_bar_style(self.to_glib_none().0, style.into_glib());
        }
    }

    #[cfg(feature = "v1_4")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_4")))]
    #[doc(alias = "bottom-bar-height")]
    pub fn connect_bottom_bar_height_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_bottom_bar_height_trampoline<F: Fn(&ToolbarView) + 'static>(
            this: *mut ffi::AdwToolbarView,
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
                b"notify::bottom-bar-height\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_bottom_bar_height_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v1_4")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_4")))]
    #[doc(alias = "bottom-bar-style")]
    pub fn connect_bottom_bar_style_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_bottom_bar_style_trampoline<F: Fn(&ToolbarView) + 'static>(
            this: *mut ffi::AdwToolbarView,
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
                b"notify::bottom-bar-style\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_bottom_bar_style_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v1_4")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_4")))]
    #[doc(alias = "content")]
    pub fn connect_content_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_content_trampoline<F: Fn(&ToolbarView) + 'static>(
            this: *mut ffi::AdwToolbarView,
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
                b"notify::content\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_content_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v1_4")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_4")))]
    #[doc(alias = "extend-content-to-bottom-edge")]
    pub fn connect_extend_content_to_bottom_edge_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_extend_content_to_bottom_edge_trampoline<
            F: Fn(&ToolbarView) + 'static,
        >(
            this: *mut ffi::AdwToolbarView,
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
                b"notify::extend-content-to-bottom-edge\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_extend_content_to_bottom_edge_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v1_4")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_4")))]
    #[doc(alias = "extend-content-to-top-edge")]
    pub fn connect_extend_content_to_top_edge_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_extend_content_to_top_edge_trampoline<
            F: Fn(&ToolbarView) + 'static,
        >(
            this: *mut ffi::AdwToolbarView,
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
                b"notify::extend-content-to-top-edge\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_extend_content_to_top_edge_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v1_4")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_4")))]
    #[doc(alias = "reveal-bottom-bars")]
    pub fn connect_reveal_bottom_bars_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_reveal_bottom_bars_trampoline<F: Fn(&ToolbarView) + 'static>(
            this: *mut ffi::AdwToolbarView,
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
                b"notify::reveal-bottom-bars\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_reveal_bottom_bars_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v1_4")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_4")))]
    #[doc(alias = "reveal-top-bars")]
    pub fn connect_reveal_top_bars_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_reveal_top_bars_trampoline<F: Fn(&ToolbarView) + 'static>(
            this: *mut ffi::AdwToolbarView,
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
                b"notify::reveal-top-bars\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_reveal_top_bars_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v1_4")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_4")))]
    #[doc(alias = "top-bar-height")]
    pub fn connect_top_bar_height_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_top_bar_height_trampoline<F: Fn(&ToolbarView) + 'static>(
            this: *mut ffi::AdwToolbarView,
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
                b"notify::top-bar-height\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_top_bar_height_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v1_4")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_4")))]
    #[doc(alias = "top-bar-style")]
    pub fn connect_top_bar_style_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_top_bar_style_trampoline<F: Fn(&ToolbarView) + 'static>(
            this: *mut ffi::AdwToolbarView,
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
                b"notify::top-bar-style\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_top_bar_style_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

#[cfg(feature = "v1_4")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_4")))]
impl Default for ToolbarView {
    fn default() -> Self {
        Self::new()
    }
}

// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`ToolbarView`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct ToolbarViewBuilder {
    builder: glib::object::ObjectBuilder<'static, ToolbarView>,
}

impl ToolbarViewBuilder {
    fn new() -> Self {
        Self {
            builder: glib::object::Object::builder(),
        }
    }

    #[cfg(feature = "v1_4")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_4")))]
    pub fn bottom_bar_style(self, bottom_bar_style: ToolbarStyle) -> Self {
        Self {
            builder: self.builder.property("bottom-bar-style", bottom_bar_style),
        }
    }

    #[cfg(feature = "v1_4")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_4")))]
    pub fn content(self, content: &impl IsA<gtk::Widget>) -> Self {
        Self {
            builder: self.builder.property("content", content.clone().upcast()),
        }
    }

    #[cfg(feature = "v1_4")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_4")))]
    pub fn extend_content_to_bottom_edge(self, extend_content_to_bottom_edge: bool) -> Self {
        Self {
            builder: self.builder.property(
                "extend-content-to-bottom-edge",
                extend_content_to_bottom_edge,
            ),
        }
    }

    #[cfg(feature = "v1_4")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_4")))]
    pub fn extend_content_to_top_edge(self, extend_content_to_top_edge: bool) -> Self {
        Self {
            builder: self
                .builder
                .property("extend-content-to-top-edge", extend_content_to_top_edge),
        }
    }

    #[cfg(feature = "v1_4")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_4")))]
    pub fn reveal_bottom_bars(self, reveal_bottom_bars: bool) -> Self {
        Self {
            builder: self
                .builder
                .property("reveal-bottom-bars", reveal_bottom_bars),
        }
    }

    #[cfg(feature = "v1_4")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_4")))]
    pub fn reveal_top_bars(self, reveal_top_bars: bool) -> Self {
        Self {
            builder: self.builder.property("reveal-top-bars", reveal_top_bars),
        }
    }

    #[cfg(feature = "v1_4")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_4")))]
    pub fn top_bar_style(self, top_bar_style: ToolbarStyle) -> Self {
        Self {
            builder: self.builder.property("top-bar-style", top_bar_style),
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
    /// Build the [`ToolbarView`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> ToolbarView {
        self.builder.build()
    }
}
