// This file was generated by gir (https://github.com/gtk-rs/gir)
// from
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

#[cfg(feature = "v1_2")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_2")))]
use glib::{bitflags::bitflags, prelude::*, translate::*};

#[cfg(feature = "v1_2")]
bitflags! {
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_2")))]
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    #[doc(alias = "AdwTabViewShortcuts")]
    pub struct TabViewShortcuts: u32 {
        #[doc(alias = "ADW_TAB_VIEW_SHORTCUT_NONE")]
        const NONE = ffi::ADW_TAB_VIEW_SHORTCUT_NONE as _;
        #[doc(alias = "ADW_TAB_VIEW_SHORTCUT_CONTROL_TAB")]
        const CONTROL_TAB = ffi::ADW_TAB_VIEW_SHORTCUT_CONTROL_TAB as _;
        #[doc(alias = "ADW_TAB_VIEW_SHORTCUT_CONTROL_SHIFT_TAB")]
        const CONTROL_SHIFT_TAB = ffi::ADW_TAB_VIEW_SHORTCUT_CONTROL_SHIFT_TAB as _;
        #[doc(alias = "ADW_TAB_VIEW_SHORTCUT_CONTROL_PAGE_UP")]
        const CONTROL_PAGE_UP = ffi::ADW_TAB_VIEW_SHORTCUT_CONTROL_PAGE_UP as _;
        #[doc(alias = "ADW_TAB_VIEW_SHORTCUT_CONTROL_PAGE_DOWN")]
        const CONTROL_PAGE_DOWN = ffi::ADW_TAB_VIEW_SHORTCUT_CONTROL_PAGE_DOWN as _;
        #[doc(alias = "ADW_TAB_VIEW_SHORTCUT_CONTROL_HOME")]
        const CONTROL_HOME = ffi::ADW_TAB_VIEW_SHORTCUT_CONTROL_HOME as _;
        #[doc(alias = "ADW_TAB_VIEW_SHORTCUT_CONTROL_END")]
        const CONTROL_END = ffi::ADW_TAB_VIEW_SHORTCUT_CONTROL_END as _;
        #[doc(alias = "ADW_TAB_VIEW_SHORTCUT_CONTROL_SHIFT_PAGE_UP")]
        const CONTROL_SHIFT_PAGE_UP = ffi::ADW_TAB_VIEW_SHORTCUT_CONTROL_SHIFT_PAGE_UP as _;
        #[doc(alias = "ADW_TAB_VIEW_SHORTCUT_CONTROL_SHIFT_PAGE_DOWN")]
        const CONTROL_SHIFT_PAGE_DOWN = ffi::ADW_TAB_VIEW_SHORTCUT_CONTROL_SHIFT_PAGE_DOWN as _;
        #[doc(alias = "ADW_TAB_VIEW_SHORTCUT_CONTROL_SHIFT_HOME")]
        const CONTROL_SHIFT_HOME = ffi::ADW_TAB_VIEW_SHORTCUT_CONTROL_SHIFT_HOME as _;
        #[doc(alias = "ADW_TAB_VIEW_SHORTCUT_CONTROL_SHIFT_END")]
        const CONTROL_SHIFT_END = ffi::ADW_TAB_VIEW_SHORTCUT_CONTROL_SHIFT_END as _;
        #[doc(alias = "ADW_TAB_VIEW_SHORTCUT_ALT_DIGITS")]
        const ALT_DIGITS = ffi::ADW_TAB_VIEW_SHORTCUT_ALT_DIGITS as _;
        #[doc(alias = "ADW_TAB_VIEW_SHORTCUT_ALT_ZERO")]
        const ALT_ZERO = ffi::ADW_TAB_VIEW_SHORTCUT_ALT_ZERO as _;
        #[doc(alias = "ADW_TAB_VIEW_SHORTCUT_ALL_SHORTCUTS")]
        const ALL_SHORTCUTS = ffi::ADW_TAB_VIEW_SHORTCUT_ALL_SHORTCUTS as _;
    }
}

#[cfg(feature = "v1_2")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_2")))]
#[doc(hidden)]
impl IntoGlib for TabViewShortcuts {
    type GlibType = ffi::AdwTabViewShortcuts;

    #[inline]
    fn into_glib(self) -> ffi::AdwTabViewShortcuts {
        self.bits()
    }
}

#[cfg(feature = "v1_2")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_2")))]
#[doc(hidden)]
impl FromGlib<ffi::AdwTabViewShortcuts> for TabViewShortcuts {
    #[inline]
    unsafe fn from_glib(value: ffi::AdwTabViewShortcuts) -> Self {
        skip_assert_initialized!();
        Self::from_bits_truncate(value)
    }
}

#[cfg(feature = "v1_2")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_2")))]
impl StaticType for TabViewShortcuts {
    #[inline]
    #[doc(alias = "adw_tab_view_shortcuts_get_type")]
    fn static_type() -> glib::Type {
        unsafe { from_glib(ffi::adw_tab_view_shortcuts_get_type()) }
    }
}

#[cfg(feature = "v1_2")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_2")))]
impl glib::HasParamSpec for TabViewShortcuts {
    type ParamSpec = glib::ParamSpecFlags;
    type SetValue = Self;
    type BuilderFn = fn(&str) -> glib::ParamSpecFlagsBuilder<Self>;

    fn param_spec_builder() -> Self::BuilderFn {
        Self::ParamSpec::builder
    }
}

#[cfg(feature = "v1_2")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_2")))]
impl glib::value::ValueType for TabViewShortcuts {
    type Type = Self;
}

#[cfg(feature = "v1_2")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_2")))]
unsafe impl<'a> glib::value::FromValue<'a> for TabViewShortcuts {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    #[inline]
    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

#[cfg(feature = "v1_2")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_2")))]
impl ToValue for TabViewShortcuts {
    #[inline]
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    #[inline]
    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

#[cfg(feature = "v1_2")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_2")))]
impl From<TabViewShortcuts> for glib::Value {
    #[inline]
    fn from(v: TabViewShortcuts) -> Self {
        skip_assert_initialized!();
        ToValue::to_value(&v)
    }
}
