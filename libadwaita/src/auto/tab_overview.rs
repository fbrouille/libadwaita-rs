// This file was generated by gir (https://github.com/gtk-rs/gir)
// from
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::TabPage;
use crate::TabView;
use glib::object::Cast;
use glib::object::IsA;
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
    #[doc(alias = "AdwTabOverview")]
    pub struct TabOverview(Object<ffi::AdwTabOverview, ffi::AdwTabOverviewClass>) @extends gtk::Widget, @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;

    match fn {
        type_ => || ffi::adw_tab_overview_get_type(),
    }
}

impl TabOverview {
    #[doc(alias = "adw_tab_overview_new")]
    pub fn new() -> TabOverview {
        assert_initialized_main_thread!();
        unsafe { gtk::Widget::from_glib_none(ffi::adw_tab_overview_new()).unsafe_cast() }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`TabOverview`] objects.
    ///
    /// This method returns an instance of [`TabOverviewBuilder`](crate::builders::TabOverviewBuilder) which can be used to create [`TabOverview`] objects.
    pub fn builder() -> TabOverviewBuilder {
        TabOverviewBuilder::default()
    }

    #[doc(alias = "adw_tab_overview_get_child")]
    #[doc(alias = "get_child")]
    pub fn child(&self) -> Option<gtk::Widget> {
        unsafe { from_glib_none(ffi::adw_tab_overview_get_child(self.to_glib_none().0)) }
    }

    #[doc(alias = "adw_tab_overview_get_enable_new_tab")]
    #[doc(alias = "get_enable_new_tab")]
    pub fn enables_new_tab(&self) -> bool {
        unsafe {
            from_glib(ffi::adw_tab_overview_get_enable_new_tab(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "adw_tab_overview_get_enable_search")]
    #[doc(alias = "get_enable_search")]
    pub fn enables_search(&self) -> bool {
        unsafe {
            from_glib(ffi::adw_tab_overview_get_enable_search(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "adw_tab_overview_get_inverted")]
    #[doc(alias = "get_inverted")]
    pub fn is_inverted(&self) -> bool {
        unsafe { from_glib(ffi::adw_tab_overview_get_inverted(self.to_glib_none().0)) }
    }

    #[doc(alias = "adw_tab_overview_get_open")]
    #[doc(alias = "get_open")]
    pub fn is_open(&self) -> bool {
        unsafe { from_glib(ffi::adw_tab_overview_get_open(self.to_glib_none().0)) }
    }

    #[doc(alias = "adw_tab_overview_get_search_active")]
    #[doc(alias = "get_search_active")]
    pub fn is_search_active(&self) -> bool {
        unsafe {
            from_glib(ffi::adw_tab_overview_get_search_active(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "adw_tab_overview_get_secondary_menu")]
    #[doc(alias = "get_secondary_menu")]
    pub fn secondary_menu(&self) -> Option<gio::MenuModel> {
        unsafe {
            from_glib_none(ffi::adw_tab_overview_get_secondary_menu(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "adw_tab_overview_get_show_end_title_buttons")]
    #[doc(alias = "get_show_end_title_buttons")]
    pub fn shows_end_title_buttons(&self) -> bool {
        unsafe {
            from_glib(ffi::adw_tab_overview_get_show_end_title_buttons(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "adw_tab_overview_get_show_start_title_buttons")]
    #[doc(alias = "get_show_start_title_buttons")]
    pub fn shows_start_title_buttons(&self) -> bool {
        unsafe {
            from_glib(ffi::adw_tab_overview_get_show_start_title_buttons(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "adw_tab_overview_get_view")]
    #[doc(alias = "get_view")]
    pub fn view(&self) -> Option<TabView> {
        unsafe { from_glib_none(ffi::adw_tab_overview_get_view(self.to_glib_none().0)) }
    }

    #[doc(alias = "adw_tab_overview_set_child")]
    pub fn set_child(&self, child: Option<&impl IsA<gtk::Widget>>) {
        unsafe {
            ffi::adw_tab_overview_set_child(
                self.to_glib_none().0,
                child.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    #[doc(alias = "adw_tab_overview_set_enable_new_tab")]
    pub fn set_enable_new_tab(&self, enable_new_tab: bool) {
        unsafe {
            ffi::adw_tab_overview_set_enable_new_tab(
                self.to_glib_none().0,
                enable_new_tab.into_glib(),
            );
        }
    }

    #[doc(alias = "adw_tab_overview_set_enable_search")]
    pub fn set_enable_search(&self, enable_search: bool) {
        unsafe {
            ffi::adw_tab_overview_set_enable_search(
                self.to_glib_none().0,
                enable_search.into_glib(),
            );
        }
    }

    #[doc(alias = "adw_tab_overview_set_inverted")]
    pub fn set_inverted(&self, inverted: bool) {
        unsafe {
            ffi::adw_tab_overview_set_inverted(self.to_glib_none().0, inverted.into_glib());
        }
    }

    #[doc(alias = "adw_tab_overview_set_open")]
    pub fn set_open(&self, open: bool) {
        unsafe {
            ffi::adw_tab_overview_set_open(self.to_glib_none().0, open.into_glib());
        }
    }

    #[doc(alias = "adw_tab_overview_set_secondary_menu")]
    pub fn set_secondary_menu(&self, secondary_menu: Option<&impl IsA<gio::MenuModel>>) {
        unsafe {
            ffi::adw_tab_overview_set_secondary_menu(
                self.to_glib_none().0,
                secondary_menu.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    #[doc(alias = "adw_tab_overview_set_show_end_title_buttons")]
    pub fn set_show_end_title_buttons(&self, show_end_title_buttons: bool) {
        unsafe {
            ffi::adw_tab_overview_set_show_end_title_buttons(
                self.to_glib_none().0,
                show_end_title_buttons.into_glib(),
            );
        }
    }

    #[doc(alias = "adw_tab_overview_set_show_start_title_buttons")]
    pub fn set_show_start_title_buttons(&self, show_start_title_buttons: bool) {
        unsafe {
            ffi::adw_tab_overview_set_show_start_title_buttons(
                self.to_glib_none().0,
                show_start_title_buttons.into_glib(),
            );
        }
    }

    #[doc(alias = "adw_tab_overview_set_view")]
    pub fn set_view(&self, view: Option<&TabView>) {
        unsafe {
            ffi::adw_tab_overview_set_view(self.to_glib_none().0, view.to_glib_none().0);
        }
    }

    //#[doc(alias = "adw_tab_overview_setup_extra_drop_target")]
    //pub fn setup_extra_drop_target(&self, actions: gdk::DragAction, types: /*Unimplemented*/Option<&CArray TypeId { ns_id: 0, id: 30 }>) {
    //    unsafe { TODO: call ffi:adw_tab_overview_setup_extra_drop_target() }
    //}

    #[cfg(any(feature = "v1_3", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_3")))]
    #[doc(alias = "create-tab")]
    pub fn connect_create_tab<F: Fn(&Self) -> TabPage + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn create_tab_trampoline<F: Fn(&TabOverview) -> TabPage + 'static>(
            this: *mut ffi::AdwTabOverview,
            f: glib::ffi::gpointer,
        ) -> *mut ffi::AdwTabPage {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this)) /*Not checked*/
                .to_glib_none()
                .0
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"create-tab\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    create_tab_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_3", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_3")))]
    #[doc(alias = "extra-drag-drop")]
    pub fn connect_extra_drag_drop<F: Fn(&Self, &TabPage, &glib::Value) -> bool + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn extra_drag_drop_trampoline<
            F: Fn(&TabOverview, &TabPage, &glib::Value) -> bool + 'static,
        >(
            this: *mut ffi::AdwTabOverview,
            page: *mut ffi::AdwTabPage,
            value: *mut glib::gobject_ffi::GValue,
            f: glib::ffi::gpointer,
        ) -> glib::ffi::gboolean {
            let f: &F = &*(f as *const F);
            f(
                &from_glib_borrow(this),
                &from_glib_borrow(page),
                &from_glib_borrow(value),
            )
            .into_glib()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"extra-drag-drop\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    extra_drag_drop_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_3", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_3")))]
    #[doc(alias = "child")]
    pub fn connect_child_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_child_trampoline<F: Fn(&TabOverview) + 'static>(
            this: *mut ffi::AdwTabOverview,
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
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_child_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_3", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_3")))]
    #[doc(alias = "enable-new-tab")]
    pub fn connect_enable_new_tab_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_enable_new_tab_trampoline<F: Fn(&TabOverview) + 'static>(
            this: *mut ffi::AdwTabOverview,
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
                b"notify::enable-new-tab\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_enable_new_tab_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_3", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_3")))]
    #[doc(alias = "enable-search")]
    pub fn connect_enable_search_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_enable_search_trampoline<F: Fn(&TabOverview) + 'static>(
            this: *mut ffi::AdwTabOverview,
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
                b"notify::enable-search\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_enable_search_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_3", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_3")))]
    #[doc(alias = "inverted")]
    pub fn connect_inverted_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_inverted_trampoline<F: Fn(&TabOverview) + 'static>(
            this: *mut ffi::AdwTabOverview,
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
                b"notify::inverted\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_inverted_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_3", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_3")))]
    #[doc(alias = "open")]
    pub fn connect_open_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_open_trampoline<F: Fn(&TabOverview) + 'static>(
            this: *mut ffi::AdwTabOverview,
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
                b"notify::open\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_open_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_3", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_3")))]
    #[doc(alias = "search-active")]
    pub fn connect_search_active_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_search_active_trampoline<F: Fn(&TabOverview) + 'static>(
            this: *mut ffi::AdwTabOverview,
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
                b"notify::search-active\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_search_active_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_3", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_3")))]
    #[doc(alias = "secondary-menu")]
    pub fn connect_secondary_menu_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_secondary_menu_trampoline<F: Fn(&TabOverview) + 'static>(
            this: *mut ffi::AdwTabOverview,
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
                b"notify::secondary-menu\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_secondary_menu_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_3", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_3")))]
    #[doc(alias = "show-end-title-buttons")]
    pub fn connect_show_end_title_buttons_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_show_end_title_buttons_trampoline<
            F: Fn(&TabOverview) + 'static,
        >(
            this: *mut ffi::AdwTabOverview,
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
                b"notify::show-end-title-buttons\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_show_end_title_buttons_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_3", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_3")))]
    #[doc(alias = "show-start-title-buttons")]
    pub fn connect_show_start_title_buttons_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_show_start_title_buttons_trampoline<
            F: Fn(&TabOverview) + 'static,
        >(
            this: *mut ffi::AdwTabOverview,
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
                b"notify::show-start-title-buttons\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_show_start_title_buttons_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_3", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_3")))]
    #[doc(alias = "view")]
    pub fn connect_view_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_view_trampoline<F: Fn(&TabOverview) + 'static>(
            this: *mut ffi::AdwTabOverview,
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
                b"notify::view\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_view_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

#[cfg(any(feature = "v1_3", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_3")))]
impl Default for TabOverview {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Default)]
// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`TabOverview`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct TabOverviewBuilder {
    #[cfg(any(feature = "v1_3", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_3")))]
    child: Option<gtk::Widget>,
    #[cfg(any(feature = "v1_3", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_3")))]
    enable_new_tab: Option<bool>,
    #[cfg(any(feature = "v1_3", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_3")))]
    enable_search: Option<bool>,
    #[cfg(any(feature = "v1_3", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_3")))]
    inverted: Option<bool>,
    #[cfg(any(feature = "v1_3", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_3")))]
    open: Option<bool>,
    #[cfg(any(feature = "v1_3", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_3")))]
    secondary_menu: Option<gio::MenuModel>,
    #[cfg(any(feature = "v1_3", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_3")))]
    show_end_title_buttons: Option<bool>,
    #[cfg(any(feature = "v1_3", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_3")))]
    show_start_title_buttons: Option<bool>,
    #[cfg(any(feature = "v1_3", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_3")))]
    view: Option<TabView>,
    can_focus: Option<bool>,
    can_target: Option<bool>,
    css_classes: Option<Vec<String>>,
    css_name: Option<String>,
    cursor: Option<gdk::Cursor>,
    focus_on_click: Option<bool>,
    focusable: Option<bool>,
    halign: Option<gtk::Align>,
    has_tooltip: Option<bool>,
    height_request: Option<i32>,
    hexpand: Option<bool>,
    hexpand_set: Option<bool>,
    layout_manager: Option<gtk::LayoutManager>,
    margin_bottom: Option<i32>,
    margin_end: Option<i32>,
    margin_start: Option<i32>,
    margin_top: Option<i32>,
    name: Option<String>,
    opacity: Option<f64>,
    overflow: Option<gtk::Overflow>,
    receives_default: Option<bool>,
    sensitive: Option<bool>,
    tooltip_markup: Option<String>,
    tooltip_text: Option<String>,
    valign: Option<gtk::Align>,
    vexpand: Option<bool>,
    vexpand_set: Option<bool>,
    visible: Option<bool>,
    width_request: Option<i32>,
    accessible_role: Option<gtk::AccessibleRole>,
}

impl TabOverviewBuilder {
    // rustdoc-stripper-ignore-next
    /// Create a new [`TabOverviewBuilder`].
    pub fn new() -> Self {
        Self::default()
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`TabOverview`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> TabOverview {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        #[cfg(any(feature = "v1_3", feature = "dox"))]
        if let Some(ref child) = self.child {
            properties.push(("child", child));
        }
        #[cfg(any(feature = "v1_3", feature = "dox"))]
        if let Some(ref enable_new_tab) = self.enable_new_tab {
            properties.push(("enable-new-tab", enable_new_tab));
        }
        #[cfg(any(feature = "v1_3", feature = "dox"))]
        if let Some(ref enable_search) = self.enable_search {
            properties.push(("enable-search", enable_search));
        }
        #[cfg(any(feature = "v1_3", feature = "dox"))]
        if let Some(ref inverted) = self.inverted {
            properties.push(("inverted", inverted));
        }
        #[cfg(any(feature = "v1_3", feature = "dox"))]
        if let Some(ref open) = self.open {
            properties.push(("open", open));
        }
        #[cfg(any(feature = "v1_3", feature = "dox"))]
        if let Some(ref secondary_menu) = self.secondary_menu {
            properties.push(("secondary-menu", secondary_menu));
        }
        #[cfg(any(feature = "v1_3", feature = "dox"))]
        if let Some(ref show_end_title_buttons) = self.show_end_title_buttons {
            properties.push(("show-end-title-buttons", show_end_title_buttons));
        }
        #[cfg(any(feature = "v1_3", feature = "dox"))]
        if let Some(ref show_start_title_buttons) = self.show_start_title_buttons {
            properties.push(("show-start-title-buttons", show_start_title_buttons));
        }
        #[cfg(any(feature = "v1_3", feature = "dox"))]
        if let Some(ref view) = self.view {
            properties.push(("view", view));
        }
        if let Some(ref can_focus) = self.can_focus {
            properties.push(("can-focus", can_focus));
        }
        if let Some(ref can_target) = self.can_target {
            properties.push(("can-target", can_target));
        }
        if let Some(ref css_classes) = self.css_classes {
            properties.push(("css-classes", css_classes));
        }
        if let Some(ref css_name) = self.css_name {
            properties.push(("css-name", css_name));
        }
        if let Some(ref cursor) = self.cursor {
            properties.push(("cursor", cursor));
        }
        if let Some(ref focus_on_click) = self.focus_on_click {
            properties.push(("focus-on-click", focus_on_click));
        }
        if let Some(ref focusable) = self.focusable {
            properties.push(("focusable", focusable));
        }
        if let Some(ref halign) = self.halign {
            properties.push(("halign", halign));
        }
        if let Some(ref has_tooltip) = self.has_tooltip {
            properties.push(("has-tooltip", has_tooltip));
        }
        if let Some(ref height_request) = self.height_request {
            properties.push(("height-request", height_request));
        }
        if let Some(ref hexpand) = self.hexpand {
            properties.push(("hexpand", hexpand));
        }
        if let Some(ref hexpand_set) = self.hexpand_set {
            properties.push(("hexpand-set", hexpand_set));
        }
        if let Some(ref layout_manager) = self.layout_manager {
            properties.push(("layout-manager", layout_manager));
        }
        if let Some(ref margin_bottom) = self.margin_bottom {
            properties.push(("margin-bottom", margin_bottom));
        }
        if let Some(ref margin_end) = self.margin_end {
            properties.push(("margin-end", margin_end));
        }
        if let Some(ref margin_start) = self.margin_start {
            properties.push(("margin-start", margin_start));
        }
        if let Some(ref margin_top) = self.margin_top {
            properties.push(("margin-top", margin_top));
        }
        if let Some(ref name) = self.name {
            properties.push(("name", name));
        }
        if let Some(ref opacity) = self.opacity {
            properties.push(("opacity", opacity));
        }
        if let Some(ref overflow) = self.overflow {
            properties.push(("overflow", overflow));
        }
        if let Some(ref receives_default) = self.receives_default {
            properties.push(("receives-default", receives_default));
        }
        if let Some(ref sensitive) = self.sensitive {
            properties.push(("sensitive", sensitive));
        }
        if let Some(ref tooltip_markup) = self.tooltip_markup {
            properties.push(("tooltip-markup", tooltip_markup));
        }
        if let Some(ref tooltip_text) = self.tooltip_text {
            properties.push(("tooltip-text", tooltip_text));
        }
        if let Some(ref valign) = self.valign {
            properties.push(("valign", valign));
        }
        if let Some(ref vexpand) = self.vexpand {
            properties.push(("vexpand", vexpand));
        }
        if let Some(ref vexpand_set) = self.vexpand_set {
            properties.push(("vexpand-set", vexpand_set));
        }
        if let Some(ref visible) = self.visible {
            properties.push(("visible", visible));
        }
        if let Some(ref width_request) = self.width_request {
            properties.push(("width-request", width_request));
        }
        if let Some(ref accessible_role) = self.accessible_role {
            properties.push(("accessible-role", accessible_role));
        }
        glib::Object::new::<TabOverview>(&properties)
    }

    #[cfg(any(feature = "v1_3", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_3")))]
    pub fn child(mut self, child: &impl IsA<gtk::Widget>) -> Self {
        self.child = Some(child.clone().upcast());
        self
    }

    #[cfg(any(feature = "v1_3", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_3")))]
    pub fn enable_new_tab(mut self, enable_new_tab: bool) -> Self {
        self.enable_new_tab = Some(enable_new_tab);
        self
    }

    #[cfg(any(feature = "v1_3", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_3")))]
    pub fn enable_search(mut self, enable_search: bool) -> Self {
        self.enable_search = Some(enable_search);
        self
    }

    #[cfg(any(feature = "v1_3", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_3")))]
    pub fn inverted(mut self, inverted: bool) -> Self {
        self.inverted = Some(inverted);
        self
    }

    #[cfg(any(feature = "v1_3", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_3")))]
    pub fn open(mut self, open: bool) -> Self {
        self.open = Some(open);
        self
    }

    #[cfg(any(feature = "v1_3", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_3")))]
    pub fn secondary_menu(mut self, secondary_menu: &impl IsA<gio::MenuModel>) -> Self {
        self.secondary_menu = Some(secondary_menu.clone().upcast());
        self
    }

    #[cfg(any(feature = "v1_3", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_3")))]
    pub fn show_end_title_buttons(mut self, show_end_title_buttons: bool) -> Self {
        self.show_end_title_buttons = Some(show_end_title_buttons);
        self
    }

    #[cfg(any(feature = "v1_3", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_3")))]
    pub fn show_start_title_buttons(mut self, show_start_title_buttons: bool) -> Self {
        self.show_start_title_buttons = Some(show_start_title_buttons);
        self
    }

    #[cfg(any(feature = "v1_3", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_3")))]
    pub fn view(mut self, view: &TabView) -> Self {
        self.view = Some(view.clone());
        self
    }

    pub fn can_focus(mut self, can_focus: bool) -> Self {
        self.can_focus = Some(can_focus);
        self
    }

    pub fn can_target(mut self, can_target: bool) -> Self {
        self.can_target = Some(can_target);
        self
    }

    pub fn css_classes(mut self, css_classes: Vec<String>) -> Self {
        self.css_classes = Some(css_classes);
        self
    }

    pub fn css_name(mut self, css_name: &str) -> Self {
        self.css_name = Some(css_name.to_string());
        self
    }

    pub fn cursor(mut self, cursor: &gdk::Cursor) -> Self {
        self.cursor = Some(cursor.clone());
        self
    }

    pub fn focus_on_click(mut self, focus_on_click: bool) -> Self {
        self.focus_on_click = Some(focus_on_click);
        self
    }

    pub fn focusable(mut self, focusable: bool) -> Self {
        self.focusable = Some(focusable);
        self
    }

    pub fn halign(mut self, halign: gtk::Align) -> Self {
        self.halign = Some(halign);
        self
    }

    pub fn has_tooltip(mut self, has_tooltip: bool) -> Self {
        self.has_tooltip = Some(has_tooltip);
        self
    }

    pub fn height_request(mut self, height_request: i32) -> Self {
        self.height_request = Some(height_request);
        self
    }

    pub fn hexpand(mut self, hexpand: bool) -> Self {
        self.hexpand = Some(hexpand);
        self
    }

    pub fn hexpand_set(mut self, hexpand_set: bool) -> Self {
        self.hexpand_set = Some(hexpand_set);
        self
    }

    pub fn layout_manager(mut self, layout_manager: &impl IsA<gtk::LayoutManager>) -> Self {
        self.layout_manager = Some(layout_manager.clone().upcast());
        self
    }

    pub fn margin_bottom(mut self, margin_bottom: i32) -> Self {
        self.margin_bottom = Some(margin_bottom);
        self
    }

    pub fn margin_end(mut self, margin_end: i32) -> Self {
        self.margin_end = Some(margin_end);
        self
    }

    pub fn margin_start(mut self, margin_start: i32) -> Self {
        self.margin_start = Some(margin_start);
        self
    }

    pub fn margin_top(mut self, margin_top: i32) -> Self {
        self.margin_top = Some(margin_top);
        self
    }

    pub fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    pub fn opacity(mut self, opacity: f64) -> Self {
        self.opacity = Some(opacity);
        self
    }

    pub fn overflow(mut self, overflow: gtk::Overflow) -> Self {
        self.overflow = Some(overflow);
        self
    }

    pub fn receives_default(mut self, receives_default: bool) -> Self {
        self.receives_default = Some(receives_default);
        self
    }

    pub fn sensitive(mut self, sensitive: bool) -> Self {
        self.sensitive = Some(sensitive);
        self
    }

    pub fn tooltip_markup(mut self, tooltip_markup: &str) -> Self {
        self.tooltip_markup = Some(tooltip_markup.to_string());
        self
    }

    pub fn tooltip_text(mut self, tooltip_text: &str) -> Self {
        self.tooltip_text = Some(tooltip_text.to_string());
        self
    }

    pub fn valign(mut self, valign: gtk::Align) -> Self {
        self.valign = Some(valign);
        self
    }

    pub fn vexpand(mut self, vexpand: bool) -> Self {
        self.vexpand = Some(vexpand);
        self
    }

    pub fn vexpand_set(mut self, vexpand_set: bool) -> Self {
        self.vexpand_set = Some(vexpand_set);
        self
    }

    pub fn visible(mut self, visible: bool) -> Self {
        self.visible = Some(visible);
        self
    }

    pub fn width_request(mut self, width_request: i32) -> Self {
        self.width_request = Some(width_request);
        self
    }

    pub fn accessible_role(mut self, accessible_role: gtk::AccessibleRole) -> Self {
        self.accessible_role = Some(accessible_role);
        self
    }
}

impl fmt::Display for TabOverview {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("TabOverview")
    }
}
