// This file was generated by gir (https://github.com/gtk-rs/gir)
// from
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::{ffi, LengthUnit, NavigationPage};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;

glib::wrapper! {
    #[doc(alias = "AdwNavigationSplitView")]
    pub struct NavigationSplitView(Object<ffi::AdwNavigationSplitView, ffi::AdwNavigationSplitViewClass>) @extends gtk::Widget, @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;

    match fn {
        type_ => || ffi::adw_navigation_split_view_get_type(),
    }
}

impl NavigationSplitView {
    #[doc(alias = "adw_navigation_split_view_new")]
    pub fn new() -> NavigationSplitView {
        assert_initialized_main_thread!();
        unsafe { gtk::Widget::from_glib_none(ffi::adw_navigation_split_view_new()).unsafe_cast() }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`NavigationSplitView`] objects.
    ///
    /// This method returns an instance of [`NavigationSplitViewBuilder`](crate::builders::NavigationSplitViewBuilder) which can be used to create [`NavigationSplitView`] objects.
    pub fn builder() -> NavigationSplitViewBuilder {
        NavigationSplitViewBuilder::new()
    }

    #[doc(alias = "adw_navigation_split_view_get_collapsed")]
    #[doc(alias = "get_collapsed")]
    #[doc(alias = "collapsed")]
    pub fn is_collapsed(&self) -> bool {
        unsafe {
            from_glib(ffi::adw_navigation_split_view_get_collapsed(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "adw_navigation_split_view_get_content")]
    #[doc(alias = "get_content")]
    pub fn content(&self) -> Option<NavigationPage> {
        unsafe {
            from_glib_none(ffi::adw_navigation_split_view_get_content(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "adw_navigation_split_view_get_max_sidebar_width")]
    #[doc(alias = "get_max_sidebar_width")]
    #[doc(alias = "max-sidebar-width")]
    pub fn max_sidebar_width(&self) -> f64 {
        unsafe { ffi::adw_navigation_split_view_get_max_sidebar_width(self.to_glib_none().0) }
    }

    #[doc(alias = "adw_navigation_split_view_get_min_sidebar_width")]
    #[doc(alias = "get_min_sidebar_width")]
    #[doc(alias = "min-sidebar-width")]
    pub fn min_sidebar_width(&self) -> f64 {
        unsafe { ffi::adw_navigation_split_view_get_min_sidebar_width(self.to_glib_none().0) }
    }

    #[doc(alias = "adw_navigation_split_view_get_show_content")]
    #[doc(alias = "get_show_content")]
    #[doc(alias = "show-content")]
    pub fn shows_content(&self) -> bool {
        unsafe {
            from_glib(ffi::adw_navigation_split_view_get_show_content(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "adw_navigation_split_view_get_sidebar")]
    #[doc(alias = "get_sidebar")]
    pub fn sidebar(&self) -> Option<NavigationPage> {
        unsafe {
            from_glib_none(ffi::adw_navigation_split_view_get_sidebar(
                self.to_glib_none().0,
            ))
        }
    }

    #[cfg(feature = "v1_7")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_7")))]
    #[doc(alias = "adw_navigation_split_view_get_sidebar_position")]
    #[doc(alias = "get_sidebar_position")]
    #[doc(alias = "sidebar-position")]
    pub fn sidebar_position(&self) -> gtk::PackType {
        unsafe {
            from_glib(ffi::adw_navigation_split_view_get_sidebar_position(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "adw_navigation_split_view_get_sidebar_width_fraction")]
    #[doc(alias = "get_sidebar_width_fraction")]
    #[doc(alias = "sidebar-width-fraction")]
    pub fn sidebar_width_fraction(&self) -> f64 {
        unsafe { ffi::adw_navigation_split_view_get_sidebar_width_fraction(self.to_glib_none().0) }
    }

    #[doc(alias = "adw_navigation_split_view_get_sidebar_width_unit")]
    #[doc(alias = "get_sidebar_width_unit")]
    #[doc(alias = "sidebar-width-unit")]
    pub fn sidebar_width_unit(&self) -> LengthUnit {
        unsafe {
            from_glib(ffi::adw_navigation_split_view_get_sidebar_width_unit(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "adw_navigation_split_view_set_collapsed")]
    #[doc(alias = "collapsed")]
    pub fn set_collapsed(&self, collapsed: bool) {
        unsafe {
            ffi::adw_navigation_split_view_set_collapsed(
                self.to_glib_none().0,
                collapsed.into_glib(),
            );
        }
    }

    #[doc(alias = "adw_navigation_split_view_set_content")]
    #[doc(alias = "content")]
    pub fn set_content(&self, content: Option<&impl IsA<NavigationPage>>) {
        unsafe {
            ffi::adw_navigation_split_view_set_content(
                self.to_glib_none().0,
                content.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    #[doc(alias = "adw_navigation_split_view_set_max_sidebar_width")]
    #[doc(alias = "max-sidebar-width")]
    pub fn set_max_sidebar_width(&self, width: f64) {
        unsafe {
            ffi::adw_navigation_split_view_set_max_sidebar_width(self.to_glib_none().0, width);
        }
    }

    #[doc(alias = "adw_navigation_split_view_set_min_sidebar_width")]
    #[doc(alias = "min-sidebar-width")]
    pub fn set_min_sidebar_width(&self, width: f64) {
        unsafe {
            ffi::adw_navigation_split_view_set_min_sidebar_width(self.to_glib_none().0, width);
        }
    }

    #[doc(alias = "adw_navigation_split_view_set_show_content")]
    #[doc(alias = "show-content")]
    pub fn set_show_content(&self, show_content: bool) {
        unsafe {
            ffi::adw_navigation_split_view_set_show_content(
                self.to_glib_none().0,
                show_content.into_glib(),
            );
        }
    }

    #[doc(alias = "adw_navigation_split_view_set_sidebar")]
    #[doc(alias = "sidebar")]
    pub fn set_sidebar(&self, sidebar: Option<&impl IsA<NavigationPage>>) {
        unsafe {
            ffi::adw_navigation_split_view_set_sidebar(
                self.to_glib_none().0,
                sidebar.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    #[cfg(feature = "v1_7")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_7")))]
    #[doc(alias = "adw_navigation_split_view_set_sidebar_position")]
    #[doc(alias = "sidebar-position")]
    pub fn set_sidebar_position(&self, position: gtk::PackType) {
        unsafe {
            ffi::adw_navigation_split_view_set_sidebar_position(
                self.to_glib_none().0,
                position.into_glib(),
            );
        }
    }

    #[doc(alias = "adw_navigation_split_view_set_sidebar_width_fraction")]
    #[doc(alias = "sidebar-width-fraction")]
    pub fn set_sidebar_width_fraction(&self, fraction: f64) {
        unsafe {
            ffi::adw_navigation_split_view_set_sidebar_width_fraction(
                self.to_glib_none().0,
                fraction,
            );
        }
    }

    #[doc(alias = "adw_navigation_split_view_set_sidebar_width_unit")]
    #[doc(alias = "sidebar-width-unit")]
    pub fn set_sidebar_width_unit(&self, unit: LengthUnit) {
        unsafe {
            ffi::adw_navigation_split_view_set_sidebar_width_unit(
                self.to_glib_none().0,
                unit.into_glib(),
            );
        }
    }

    #[cfg(feature = "v1_4")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_4")))]
    #[doc(alias = "collapsed")]
    pub fn connect_collapsed_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_collapsed_trampoline<F: Fn(&NavigationSplitView) + 'static>(
            this: *mut ffi::AdwNavigationSplitView,
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
                b"notify::collapsed\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_collapsed_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v1_4")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_4")))]
    #[doc(alias = "content")]
    pub fn connect_content_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_content_trampoline<F: Fn(&NavigationSplitView) + 'static>(
            this: *mut ffi::AdwNavigationSplitView,
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
    #[doc(alias = "max-sidebar-width")]
    pub fn connect_max_sidebar_width_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_max_sidebar_width_trampoline<
            F: Fn(&NavigationSplitView) + 'static,
        >(
            this: *mut ffi::AdwNavigationSplitView,
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
                b"notify::max-sidebar-width\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_max_sidebar_width_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v1_4")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_4")))]
    #[doc(alias = "min-sidebar-width")]
    pub fn connect_min_sidebar_width_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_min_sidebar_width_trampoline<
            F: Fn(&NavigationSplitView) + 'static,
        >(
            this: *mut ffi::AdwNavigationSplitView,
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
                b"notify::min-sidebar-width\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_min_sidebar_width_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v1_4")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_4")))]
    #[doc(alias = "show-content")]
    pub fn connect_show_content_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_show_content_trampoline<
            F: Fn(&NavigationSplitView) + 'static,
        >(
            this: *mut ffi::AdwNavigationSplitView,
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
                b"notify::show-content\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_show_content_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v1_4")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_4")))]
    #[doc(alias = "sidebar")]
    pub fn connect_sidebar_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_sidebar_trampoline<F: Fn(&NavigationSplitView) + 'static>(
            this: *mut ffi::AdwNavigationSplitView,
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
                b"notify::sidebar\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_sidebar_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v1_7")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_7")))]
    #[doc(alias = "sidebar-position")]
    pub fn connect_sidebar_position_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_sidebar_position_trampoline<
            F: Fn(&NavigationSplitView) + 'static,
        >(
            this: *mut ffi::AdwNavigationSplitView,
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
                b"notify::sidebar-position\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_sidebar_position_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v1_4")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_4")))]
    #[doc(alias = "sidebar-width-fraction")]
    pub fn connect_sidebar_width_fraction_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_sidebar_width_fraction_trampoline<
            F: Fn(&NavigationSplitView) + 'static,
        >(
            this: *mut ffi::AdwNavigationSplitView,
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
                b"notify::sidebar-width-fraction\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_sidebar_width_fraction_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v1_4")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_4")))]
    #[doc(alias = "sidebar-width-unit")]
    pub fn connect_sidebar_width_unit_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_sidebar_width_unit_trampoline<
            F: Fn(&NavigationSplitView) + 'static,
        >(
            this: *mut ffi::AdwNavigationSplitView,
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
                b"notify::sidebar-width-unit\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_sidebar_width_unit_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

#[cfg(feature = "v1_4")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_4")))]
impl Default for NavigationSplitView {
    fn default() -> Self {
        Self::new()
    }
}

// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`NavigationSplitView`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct NavigationSplitViewBuilder {
    builder: glib::object::ObjectBuilder<'static, NavigationSplitView>,
}

impl NavigationSplitViewBuilder {
    fn new() -> Self {
        Self {
            builder: glib::object::Object::builder(),
        }
    }

    #[cfg(feature = "v1_4")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_4")))]
    pub fn collapsed(self, collapsed: bool) -> Self {
        Self {
            builder: self.builder.property("collapsed", collapsed),
        }
    }

    #[cfg(feature = "v1_4")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_4")))]
    pub fn content(self, content: &impl IsA<NavigationPage>) -> Self {
        Self {
            builder: self.builder.property("content", content.clone().upcast()),
        }
    }

    #[cfg(feature = "v1_4")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_4")))]
    pub fn max_sidebar_width(self, max_sidebar_width: f64) -> Self {
        Self {
            builder: self
                .builder
                .property("max-sidebar-width", max_sidebar_width),
        }
    }

    #[cfg(feature = "v1_4")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_4")))]
    pub fn min_sidebar_width(self, min_sidebar_width: f64) -> Self {
        Self {
            builder: self
                .builder
                .property("min-sidebar-width", min_sidebar_width),
        }
    }

    #[cfg(feature = "v1_4")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_4")))]
    pub fn show_content(self, show_content: bool) -> Self {
        Self {
            builder: self.builder.property("show-content", show_content),
        }
    }

    #[cfg(feature = "v1_4")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_4")))]
    pub fn sidebar(self, sidebar: &impl IsA<NavigationPage>) -> Self {
        Self {
            builder: self.builder.property("sidebar", sidebar.clone().upcast()),
        }
    }

    #[cfg(feature = "v1_7")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_7")))]
    pub fn sidebar_position(self, sidebar_position: gtk::PackType) -> Self {
        Self {
            builder: self.builder.property("sidebar-position", sidebar_position),
        }
    }

    #[cfg(feature = "v1_4")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_4")))]
    pub fn sidebar_width_fraction(self, sidebar_width_fraction: f64) -> Self {
        Self {
            builder: self
                .builder
                .property("sidebar-width-fraction", sidebar_width_fraction),
        }
    }

    #[cfg(feature = "v1_4")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_4")))]
    pub fn sidebar_width_unit(self, sidebar_width_unit: LengthUnit) -> Self {
        Self {
            builder: self
                .builder
                .property("sidebar-width-unit", sidebar_width_unit),
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
    /// Build the [`NavigationSplitView`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> NavigationSplitView {
        self.builder.build()
    }
}
