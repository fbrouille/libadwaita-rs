// This file was generated by gir (https://github.com/gtk-rs/gir)
// from
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use glib::object::IsA;
use glib::object::ObjectType as ObjectType_;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::wrapper! {
    #[doc(alias = "AdwTabPage")]
    pub struct TabPage(Object<ffi::AdwTabPage, ffi::AdwTabPageClass>);

    match fn {
        type_ => || ffi::adw_tab_page_get_type(),
    }
}

impl TabPage {
    #[doc(alias = "adw_tab_page_get_child")]
    #[doc(alias = "get_child")]
    pub fn child(&self) -> Option<gtk::Widget> {
        unsafe { from_glib_none(ffi::adw_tab_page_get_child(self.to_glib_none().0)) }
    }

    #[doc(alias = "adw_tab_page_get_icon")]
    #[doc(alias = "get_icon")]
    pub fn icon(&self) -> Option<gio::Icon> {
        unsafe { from_glib_none(ffi::adw_tab_page_get_icon(self.to_glib_none().0)) }
    }

    #[doc(alias = "adw_tab_page_get_indicator_activatable")]
    #[doc(alias = "get_indicator_activatable")]
    pub fn is_indicator_activatable(&self) -> bool {
        unsafe {
            from_glib(ffi::adw_tab_page_get_indicator_activatable(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "adw_tab_page_get_indicator_icon")]
    #[doc(alias = "get_indicator_icon")]
    pub fn indicator_icon(&self) -> Option<gio::Icon> {
        unsafe { from_glib_none(ffi::adw_tab_page_get_indicator_icon(self.to_glib_none().0)) }
    }

    #[doc(alias = "adw_tab_page_get_loading")]
    #[doc(alias = "get_loading")]
    pub fn is_loading(&self) -> bool {
        unsafe { from_glib(ffi::adw_tab_page_get_loading(self.to_glib_none().0)) }
    }

    #[doc(alias = "adw_tab_page_get_needs_attention")]
    #[doc(alias = "get_needs_attention")]
    pub fn needs_attention(&self) -> bool {
        unsafe { from_glib(ffi::adw_tab_page_get_needs_attention(self.to_glib_none().0)) }
    }

    #[doc(alias = "adw_tab_page_get_parent")]
    #[doc(alias = "get_parent")]
    pub fn parent(&self) -> Option<TabPage> {
        unsafe { from_glib_none(ffi::adw_tab_page_get_parent(self.to_glib_none().0)) }
    }

    #[doc(alias = "adw_tab_page_get_pinned")]
    #[doc(alias = "get_pinned")]
    pub fn is_pinned(&self) -> bool {
        unsafe { from_glib(ffi::adw_tab_page_get_pinned(self.to_glib_none().0)) }
    }

    #[doc(alias = "adw_tab_page_get_selected")]
    #[doc(alias = "get_selected")]
    pub fn is_selected(&self) -> bool {
        unsafe { from_glib(ffi::adw_tab_page_get_selected(self.to_glib_none().0)) }
    }

    #[doc(alias = "adw_tab_page_get_title")]
    #[doc(alias = "get_title")]
    pub fn title(&self) -> Option<glib::GString> {
        unsafe { from_glib_none(ffi::adw_tab_page_get_title(self.to_glib_none().0)) }
    }

    #[doc(alias = "adw_tab_page_get_tooltip")]
    #[doc(alias = "get_tooltip")]
    pub fn tooltip(&self) -> Option<glib::GString> {
        unsafe { from_glib_none(ffi::adw_tab_page_get_tooltip(self.to_glib_none().0)) }
    }

    #[doc(alias = "adw_tab_page_set_icon")]
    pub fn set_icon<P: IsA<gio::Icon>>(&self, icon: Option<&P>) {
        unsafe {
            ffi::adw_tab_page_set_icon(
                self.to_glib_none().0,
                icon.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    #[doc(alias = "adw_tab_page_set_indicator_activatable")]
    pub fn set_indicator_activatable(&self, activatable: bool) {
        unsafe {
            ffi::adw_tab_page_set_indicator_activatable(
                self.to_glib_none().0,
                activatable.into_glib(),
            );
        }
    }

    #[doc(alias = "adw_tab_page_set_indicator_icon")]
    pub fn set_indicator_icon<P: IsA<gio::Icon>>(&self, indicator_icon: Option<&P>) {
        unsafe {
            ffi::adw_tab_page_set_indicator_icon(
                self.to_glib_none().0,
                indicator_icon.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    #[doc(alias = "adw_tab_page_set_loading")]
    pub fn set_loading(&self, loading: bool) {
        unsafe {
            ffi::adw_tab_page_set_loading(self.to_glib_none().0, loading.into_glib());
        }
    }

    #[doc(alias = "adw_tab_page_set_needs_attention")]
    pub fn set_needs_attention(&self, needs_attention: bool) {
        unsafe {
            ffi::adw_tab_page_set_needs_attention(
                self.to_glib_none().0,
                needs_attention.into_glib(),
            );
        }
    }

    #[doc(alias = "adw_tab_page_set_title")]
    pub fn set_title(&self, title: Option<&str>) {
        unsafe {
            ffi::adw_tab_page_set_title(self.to_glib_none().0, title.to_glib_none().0);
        }
    }

    #[doc(alias = "adw_tab_page_set_tooltip")]
    pub fn set_tooltip(&self, tooltip: Option<&str>) {
        unsafe {
            ffi::adw_tab_page_set_tooltip(self.to_glib_none().0, tooltip.to_glib_none().0);
        }
    }

    #[doc(alias = "icon")]
    pub fn connect_icon_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_icon_trampoline<F: Fn(&TabPage) + 'static>(
            this: *mut ffi::AdwTabPage,
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
                b"notify::icon\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_icon_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "indicator-activatable")]
    pub fn connect_indicator_activatable_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_indicator_activatable_trampoline<F: Fn(&TabPage) + 'static>(
            this: *mut ffi::AdwTabPage,
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
                b"notify::indicator-activatable\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_indicator_activatable_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "indicator-icon")]
    pub fn connect_indicator_icon_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_indicator_icon_trampoline<F: Fn(&TabPage) + 'static>(
            this: *mut ffi::AdwTabPage,
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
                b"notify::indicator-icon\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_indicator_icon_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "loading")]
    pub fn connect_loading_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_loading_trampoline<F: Fn(&TabPage) + 'static>(
            this: *mut ffi::AdwTabPage,
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
                b"notify::loading\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_loading_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "needs-attention")]
    pub fn connect_needs_attention_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_needs_attention_trampoline<F: Fn(&TabPage) + 'static>(
            this: *mut ffi::AdwTabPage,
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
                b"notify::needs-attention\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_needs_attention_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "pinned")]
    pub fn connect_pinned_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_pinned_trampoline<F: Fn(&TabPage) + 'static>(
            this: *mut ffi::AdwTabPage,
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
                b"notify::pinned\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_pinned_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "selected")]
    pub fn connect_selected_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_selected_trampoline<F: Fn(&TabPage) + 'static>(
            this: *mut ffi::AdwTabPage,
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
                b"notify::selected\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_selected_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "title")]
    pub fn connect_title_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_title_trampoline<F: Fn(&TabPage) + 'static>(
            this: *mut ffi::AdwTabPage,
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
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_title_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "tooltip")]
    pub fn connect_tooltip_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_tooltip_trampoline<F: Fn(&TabPage) + 'static>(
            this: *mut ffi::AdwTabPage,
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
                b"notify::tooltip\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_tooltip_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for TabPage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("TabPage")
    }
}
