// This file was generated by gir (https://github.com/gtk-rs/gir)
// from
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::{TabPage, TabView};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;

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
        TabOverviewBuilder::new()
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

    #[cfg(feature = "v1_4")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_4")))]
    #[doc(alias = "adw_tab_overview_get_extra_drag_preferred_action")]
    #[doc(alias = "get_extra_drag_preferred_action")]
    pub fn extra_drag_preferred_action(&self) -> gdk::DragAction {
        unsafe {
            from_glib(ffi::adw_tab_overview_get_extra_drag_preferred_action(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "adw_tab_overview_get_extra_drag_preload")]
    #[doc(alias = "get_extra_drag_preload")]
    pub fn is_extra_drag_preload(&self) -> bool {
        unsafe {
            from_glib(ffi::adw_tab_overview_get_extra_drag_preload(
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

    #[doc(alias = "adw_tab_overview_set_extra_drag_preload")]
    pub fn set_extra_drag_preload(&self, preload: bool) {
        unsafe {
            ffi::adw_tab_overview_set_extra_drag_preload(
                self.to_glib_none().0,
                preload.into_glib(),
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

    #[cfg(feature = "v1_3")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_3")))]
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
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    create_tab_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v1_3")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_3")))]
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
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    extra_drag_drop_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v1_3")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_3")))]
    #[doc(alias = "extra-drag-value")]
    pub fn connect_extra_drag_value<
        F: Fn(&Self, &TabPage, &glib::Value) -> gdk::DragAction + 'static,
    >(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn extra_drag_value_trampoline<
            F: Fn(&TabOverview, &TabPage, &glib::Value) -> gdk::DragAction + 'static,
        >(
            this: *mut ffi::AdwTabOverview,
            page: *mut ffi::AdwTabPage,
            value: *mut glib::gobject_ffi::GValue,
            f: glib::ffi::gpointer,
        ) -> gdk::ffi::GdkDragAction {
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
                b"extra-drag-value\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    extra_drag_value_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v1_3")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_3")))]
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
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    notify_child_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v1_3")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_3")))]
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
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    notify_enable_new_tab_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v1_3")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_3")))]
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
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    notify_enable_search_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v1_4")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_4")))]
    #[doc(alias = "extra-drag-preferred-action")]
    pub fn connect_extra_drag_preferred_action_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_extra_drag_preferred_action_trampoline<
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
                b"notify::extra-drag-preferred-action\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    notify_extra_drag_preferred_action_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v1_3")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_3")))]
    #[doc(alias = "extra-drag-preload")]
    pub fn connect_extra_drag_preload_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_extra_drag_preload_trampoline<F: Fn(&TabOverview) + 'static>(
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
                b"notify::extra-drag-preload\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    notify_extra_drag_preload_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v1_3")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_3")))]
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
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    notify_inverted_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v1_3")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_3")))]
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
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    notify_open_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v1_3")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_3")))]
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
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    notify_search_active_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v1_3")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_3")))]
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
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    notify_secondary_menu_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v1_3")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_3")))]
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
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    notify_show_end_title_buttons_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v1_3")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_3")))]
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
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    notify_show_start_title_buttons_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v1_3")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_3")))]
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
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    notify_view_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

#[cfg(feature = "v1_3")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_3")))]
impl Default for TabOverview {
    fn default() -> Self {
        Self::new()
    }
}

// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`TabOverview`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct TabOverviewBuilder {
    builder: glib::object::ObjectBuilder<'static, TabOverview>,
}

impl TabOverviewBuilder {
    fn new() -> Self {
        Self {
            builder: glib::object::Object::builder(),
        }
    }

    #[cfg(feature = "v1_3")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_3")))]
    pub fn child(self, child: &impl IsA<gtk::Widget>) -> Self {
        Self {
            builder: self.builder.property("child", child.clone().upcast()),
        }
    }

    #[cfg(feature = "v1_3")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_3")))]
    pub fn enable_new_tab(self, enable_new_tab: bool) -> Self {
        Self {
            builder: self.builder.property("enable-new-tab", enable_new_tab),
        }
    }

    #[cfg(feature = "v1_3")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_3")))]
    pub fn enable_search(self, enable_search: bool) -> Self {
        Self {
            builder: self.builder.property("enable-search", enable_search),
        }
    }

    #[cfg(feature = "v1_3")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_3")))]
    pub fn extra_drag_preload(self, extra_drag_preload: bool) -> Self {
        Self {
            builder: self
                .builder
                .property("extra-drag-preload", extra_drag_preload),
        }
    }

    #[cfg(feature = "v1_3")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_3")))]
    pub fn inverted(self, inverted: bool) -> Self {
        Self {
            builder: self.builder.property("inverted", inverted),
        }
    }

    #[cfg(feature = "v1_3")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_3")))]
    pub fn open(self, open: bool) -> Self {
        Self {
            builder: self.builder.property("open", open),
        }
    }

    #[cfg(feature = "v1_3")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_3")))]
    pub fn secondary_menu(self, secondary_menu: &impl IsA<gio::MenuModel>) -> Self {
        Self {
            builder: self
                .builder
                .property("secondary-menu", secondary_menu.clone().upcast()),
        }
    }

    #[cfg(feature = "v1_3")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_3")))]
    pub fn show_end_title_buttons(self, show_end_title_buttons: bool) -> Self {
        Self {
            builder: self
                .builder
                .property("show-end-title-buttons", show_end_title_buttons),
        }
    }

    #[cfg(feature = "v1_3")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_3")))]
    pub fn show_start_title_buttons(self, show_start_title_buttons: bool) -> Self {
        Self {
            builder: self
                .builder
                .property("show-start-title-buttons", show_start_title_buttons),
        }
    }

    #[cfg(feature = "v1_3")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_3")))]
    pub fn view(self, view: &TabView) -> Self {
        Self {
            builder: self.builder.property("view", view.clone()),
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
    /// Build the [`TabOverview`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> TabOverview {
        self.builder.build()
    }
}
