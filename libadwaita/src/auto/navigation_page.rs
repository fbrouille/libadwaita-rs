// This file was generated by gir (https://github.com/gtk-rs/gir)
// from
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;

glib::wrapper! {
    #[doc(alias = "AdwNavigationPage")]
    pub struct NavigationPage(Object<ffi::AdwNavigationPage, ffi::AdwNavigationPageClass>) @extends gtk::Widget, @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;

    match fn {
        type_ => || ffi::adw_navigation_page_get_type(),
    }
}

impl NavigationPage {
    pub const NONE: Option<&'static NavigationPage> = None;

    #[doc(alias = "adw_navigation_page_new")]
    pub fn new(child: &impl IsA<gtk::Widget>, title: &str) -> NavigationPage {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(ffi::adw_navigation_page_new(
                child.as_ref().to_glib_none().0,
                title.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "adw_navigation_page_new_with_tag")]
    #[doc(alias = "new_with_tag")]
    pub fn with_tag(child: &impl IsA<gtk::Widget>, title: &str, tag: &str) -> NavigationPage {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(ffi::adw_navigation_page_new_with_tag(
                child.as_ref().to_glib_none().0,
                title.to_glib_none().0,
                tag.to_glib_none().0,
            ))
        }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`NavigationPage`] objects.
    ///
    /// This method returns an instance of [`NavigationPageBuilder`](crate::builders::NavigationPageBuilder) which can be used to create [`NavigationPage`] objects.
    pub fn builder() -> NavigationPageBuilder {
        NavigationPageBuilder::new()
    }
}

#[cfg(feature = "v1_4")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_4")))]
impl Default for NavigationPage {
    fn default() -> Self {
        glib::object::Object::new::<Self>()
    }
}

// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`NavigationPage`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct NavigationPageBuilder {
    builder: glib::object::ObjectBuilder<'static, NavigationPage>,
}

impl NavigationPageBuilder {
    fn new() -> Self {
        Self {
            builder: glib::object::Object::builder(),
        }
    }

    #[cfg(feature = "v1_4")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_4")))]
    pub fn can_pop(self, can_pop: bool) -> Self {
        Self {
            builder: self.builder.property("can-pop", can_pop),
        }
    }

    #[cfg(feature = "v1_4")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_4")))]
    pub fn child(self, child: &impl IsA<gtk::Widget>) -> Self {
        Self {
            builder: self.builder.property("child", child.clone().upcast()),
        }
    }

    #[cfg(feature = "v1_4")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_4")))]
    pub fn tag(self, tag: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("tag", tag.into()),
        }
    }

    #[cfg(feature = "v1_4")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_4")))]
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
    /// Build the [`NavigationPage`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> NavigationPage {
        self.builder.build()
    }
}

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::NavigationPage>> Sealed for T {}
}

pub trait NavigationPageExt: IsA<NavigationPage> + sealed::Sealed + 'static {
    #[doc(alias = "adw_navigation_page_get_can_pop")]
    #[doc(alias = "get_can_pop")]
    fn can_pop(&self) -> bool {
        unsafe {
            from_glib(ffi::adw_navigation_page_get_can_pop(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "adw_navigation_page_get_child")]
    #[doc(alias = "get_child")]
    fn child(&self) -> Option<gtk::Widget> {
        unsafe {
            from_glib_none(ffi::adw_navigation_page_get_child(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "adw_navigation_page_get_tag")]
    #[doc(alias = "get_tag")]
    fn tag(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::adw_navigation_page_get_tag(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "adw_navigation_page_get_title")]
    #[doc(alias = "get_title")]
    fn title(&self) -> glib::GString {
        unsafe {
            from_glib_none(ffi::adw_navigation_page_get_title(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "adw_navigation_page_set_can_pop")]
    fn set_can_pop(&self, can_pop: bool) {
        unsafe {
            ffi::adw_navigation_page_set_can_pop(
                self.as_ref().to_glib_none().0,
                can_pop.into_glib(),
            );
        }
    }

    #[doc(alias = "adw_navigation_page_set_child")]
    fn set_child(&self, child: Option<&impl IsA<gtk::Widget>>) {
        unsafe {
            ffi::adw_navigation_page_set_child(
                self.as_ref().to_glib_none().0,
                child.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    #[doc(alias = "adw_navigation_page_set_tag")]
    fn set_tag(&self, tag: Option<&str>) {
        unsafe {
            ffi::adw_navigation_page_set_tag(self.as_ref().to_glib_none().0, tag.to_glib_none().0);
        }
    }

    #[doc(alias = "adw_navigation_page_set_title")]
    fn set_title(&self, title: &str) {
        unsafe {
            ffi::adw_navigation_page_set_title(
                self.as_ref().to_glib_none().0,
                title.to_glib_none().0,
            );
        }
    }

    #[cfg(feature = "v1_4")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_4")))]
    #[doc(alias = "hidden")]
    fn connect_hidden<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn hidden_trampoline<P: IsA<NavigationPage>, F: Fn(&P) + 'static>(
            this: *mut ffi::AdwNavigationPage,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(NavigationPage::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"hidden\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    hidden_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v1_4")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_4")))]
    #[doc(alias = "hiding")]
    fn connect_hiding<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn hiding_trampoline<P: IsA<NavigationPage>, F: Fn(&P) + 'static>(
            this: *mut ffi::AdwNavigationPage,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(NavigationPage::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"hiding\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    hiding_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v1_4")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_4")))]
    #[doc(alias = "showing")]
    fn connect_showing<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn showing_trampoline<P: IsA<NavigationPage>, F: Fn(&P) + 'static>(
            this: *mut ffi::AdwNavigationPage,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(NavigationPage::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"showing\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    showing_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v1_4")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_4")))]
    #[doc(alias = "shown")]
    fn connect_shown<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn shown_trampoline<P: IsA<NavigationPage>, F: Fn(&P) + 'static>(
            this: *mut ffi::AdwNavigationPage,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(NavigationPage::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"shown\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    shown_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v1_4")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_4")))]
    #[doc(alias = "can-pop")]
    fn connect_can_pop_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_can_pop_trampoline<
            P: IsA<NavigationPage>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::AdwNavigationPage,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(NavigationPage::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::can-pop\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    notify_can_pop_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v1_4")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_4")))]
    #[doc(alias = "child")]
    fn connect_child_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_child_trampoline<
            P: IsA<NavigationPage>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::AdwNavigationPage,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(NavigationPage::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::child\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    notify_child_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v1_4")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_4")))]
    #[doc(alias = "tag")]
    fn connect_tag_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_tag_trampoline<P: IsA<NavigationPage>, F: Fn(&P) + 'static>(
            this: *mut ffi::AdwNavigationPage,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(NavigationPage::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::tag\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    notify_tag_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v1_4")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_4")))]
    #[doc(alias = "title")]
    fn connect_title_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_title_trampoline<
            P: IsA<NavigationPage>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::AdwNavigationPage,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(NavigationPage::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::title\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    notify_title_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl<O: IsA<NavigationPage>> NavigationPageExt for O {}
