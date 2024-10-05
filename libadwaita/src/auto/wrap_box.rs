// This file was generated by gir (https://github.com/gtk-rs/gir)
// from
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::{ffi, JustifyMode, LengthUnit, PackDirection, WrapPolicy};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;

glib::wrapper! {
    #[doc(alias = "AdwWrapBox")]
    pub struct WrapBox(Object<ffi::AdwWrapBox, ffi::AdwWrapBoxClass>) @extends gtk::Widget, @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;

    match fn {
        type_ => || ffi::adw_wrap_box_get_type(),
    }
}

impl WrapBox {
    #[doc(alias = "adw_wrap_box_new")]
    pub fn new() -> WrapBox {
        assert_initialized_main_thread!();
        unsafe { gtk::Widget::from_glib_none(ffi::adw_wrap_box_new()).unsafe_cast() }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`WrapBox`] objects.
    ///
    /// This method returns an instance of [`WrapBoxBuilder`](crate::builders::WrapBoxBuilder) which can be used to create [`WrapBox`] objects.
    pub fn builder() -> WrapBoxBuilder {
        WrapBoxBuilder::new()
    }

    #[doc(alias = "adw_wrap_box_append")]
    pub fn append(&self, child: &impl IsA<gtk::Widget>) {
        unsafe {
            ffi::adw_wrap_box_append(self.to_glib_none().0, child.as_ref().to_glib_none().0);
        }
    }

    #[doc(alias = "adw_wrap_box_get_align")]
    #[doc(alias = "get_align")]
    pub fn align(&self) -> f32 {
        unsafe { ffi::adw_wrap_box_get_align(self.to_glib_none().0) }
    }

    #[doc(alias = "adw_wrap_box_get_child_spacing")]
    #[doc(alias = "get_child_spacing")]
    #[doc(alias = "child-spacing")]
    pub fn child_spacing(&self) -> i32 {
        unsafe { ffi::adw_wrap_box_get_child_spacing(self.to_glib_none().0) }
    }

    #[doc(alias = "adw_wrap_box_get_child_spacing_unit")]
    #[doc(alias = "get_child_spacing_unit")]
    #[doc(alias = "child-spacing-unit")]
    pub fn child_spacing_unit(&self) -> LengthUnit {
        unsafe {
            from_glib(ffi::adw_wrap_box_get_child_spacing_unit(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "adw_wrap_box_get_justify")]
    #[doc(alias = "get_justify")]
    pub fn justify(&self) -> JustifyMode {
        unsafe { from_glib(ffi::adw_wrap_box_get_justify(self.to_glib_none().0)) }
    }

    #[doc(alias = "adw_wrap_box_get_justify_last_line")]
    #[doc(alias = "get_justify_last_line")]
    #[doc(alias = "justify-last-line")]
    pub fn is_justify_last_line(&self) -> bool {
        unsafe {
            from_glib(ffi::adw_wrap_box_get_justify_last_line(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "adw_wrap_box_get_line_homogeneous")]
    #[doc(alias = "get_line_homogeneous")]
    #[doc(alias = "line-homogeneous")]
    pub fn is_line_homogeneous(&self) -> bool {
        unsafe {
            from_glib(ffi::adw_wrap_box_get_line_homogeneous(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "adw_wrap_box_get_line_spacing")]
    #[doc(alias = "get_line_spacing")]
    #[doc(alias = "line-spacing")]
    pub fn line_spacing(&self) -> i32 {
        unsafe { ffi::adw_wrap_box_get_line_spacing(self.to_glib_none().0) }
    }

    #[doc(alias = "adw_wrap_box_get_line_spacing_unit")]
    #[doc(alias = "get_line_spacing_unit")]
    #[doc(alias = "line-spacing-unit")]
    pub fn line_spacing_unit(&self) -> LengthUnit {
        unsafe {
            from_glib(ffi::adw_wrap_box_get_line_spacing_unit(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "adw_wrap_box_get_natural_line_length")]
    #[doc(alias = "get_natural_line_length")]
    #[doc(alias = "natural-line-length")]
    pub fn natural_line_length(&self) -> i32 {
        unsafe { ffi::adw_wrap_box_get_natural_line_length(self.to_glib_none().0) }
    }

    #[doc(alias = "adw_wrap_box_get_natural_line_length_unit")]
    #[doc(alias = "get_natural_line_length_unit")]
    #[doc(alias = "natural-line-length-unit")]
    pub fn natural_line_length_unit(&self) -> LengthUnit {
        unsafe {
            from_glib(ffi::adw_wrap_box_get_natural_line_length_unit(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "adw_wrap_box_get_pack_direction")]
    #[doc(alias = "get_pack_direction")]
    #[doc(alias = "pack-direction")]
    pub fn pack_direction(&self) -> PackDirection {
        unsafe { from_glib(ffi::adw_wrap_box_get_pack_direction(self.to_glib_none().0)) }
    }

    #[doc(alias = "adw_wrap_box_get_wrap_policy")]
    #[doc(alias = "get_wrap_policy")]
    #[doc(alias = "wrap-policy")]
    pub fn wrap_policy(&self) -> WrapPolicy {
        unsafe { from_glib(ffi::adw_wrap_box_get_wrap_policy(self.to_glib_none().0)) }
    }

    #[doc(alias = "adw_wrap_box_get_wrap_reverse")]
    #[doc(alias = "get_wrap_reverse")]
    #[doc(alias = "wrap-reverse")]
    pub fn wraps_reverse(&self) -> bool {
        unsafe { from_glib(ffi::adw_wrap_box_get_wrap_reverse(self.to_glib_none().0)) }
    }

    #[doc(alias = "adw_wrap_box_insert_child_after")]
    pub fn insert_child_after(
        &self,
        child: &impl IsA<gtk::Widget>,
        sibling: Option<&impl IsA<gtk::Widget>>,
    ) {
        unsafe {
            ffi::adw_wrap_box_insert_child_after(
                self.to_glib_none().0,
                child.as_ref().to_glib_none().0,
                sibling.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    #[doc(alias = "adw_wrap_box_prepend")]
    pub fn prepend(&self, child: &impl IsA<gtk::Widget>) {
        unsafe {
            ffi::adw_wrap_box_prepend(self.to_glib_none().0, child.as_ref().to_glib_none().0);
        }
    }

    #[doc(alias = "adw_wrap_box_remove")]
    pub fn remove(&self, child: &impl IsA<gtk::Widget>) {
        unsafe {
            ffi::adw_wrap_box_remove(self.to_glib_none().0, child.as_ref().to_glib_none().0);
        }
    }

    #[doc(alias = "adw_wrap_box_reorder_child_after")]
    pub fn reorder_child_after(
        &self,
        child: &impl IsA<gtk::Widget>,
        sibling: Option<&impl IsA<gtk::Widget>>,
    ) {
        unsafe {
            ffi::adw_wrap_box_reorder_child_after(
                self.to_glib_none().0,
                child.as_ref().to_glib_none().0,
                sibling.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    #[doc(alias = "adw_wrap_box_set_align")]
    #[doc(alias = "align")]
    pub fn set_align(&self, align: f32) {
        unsafe {
            ffi::adw_wrap_box_set_align(self.to_glib_none().0, align);
        }
    }

    #[doc(alias = "adw_wrap_box_set_child_spacing")]
    #[doc(alias = "child-spacing")]
    pub fn set_child_spacing(&self, child_spacing: i32) {
        unsafe {
            ffi::adw_wrap_box_set_child_spacing(self.to_glib_none().0, child_spacing);
        }
    }

    #[doc(alias = "adw_wrap_box_set_child_spacing_unit")]
    #[doc(alias = "child-spacing-unit")]
    pub fn set_child_spacing_unit(&self, unit: LengthUnit) {
        unsafe {
            ffi::adw_wrap_box_set_child_spacing_unit(self.to_glib_none().0, unit.into_glib());
        }
    }

    #[doc(alias = "adw_wrap_box_set_justify")]
    #[doc(alias = "justify")]
    pub fn set_justify(&self, justify: JustifyMode) {
        unsafe {
            ffi::adw_wrap_box_set_justify(self.to_glib_none().0, justify.into_glib());
        }
    }

    #[doc(alias = "adw_wrap_box_set_justify_last_line")]
    #[doc(alias = "justify-last-line")]
    pub fn set_justify_last_line(&self, justify_last_line: bool) {
        unsafe {
            ffi::adw_wrap_box_set_justify_last_line(
                self.to_glib_none().0,
                justify_last_line.into_glib(),
            );
        }
    }

    #[doc(alias = "adw_wrap_box_set_line_homogeneous")]
    #[doc(alias = "line-homogeneous")]
    pub fn set_line_homogeneous(&self, homogeneous: bool) {
        unsafe {
            ffi::adw_wrap_box_set_line_homogeneous(self.to_glib_none().0, homogeneous.into_glib());
        }
    }

    #[doc(alias = "adw_wrap_box_set_line_spacing")]
    #[doc(alias = "line-spacing")]
    pub fn set_line_spacing(&self, line_spacing: i32) {
        unsafe {
            ffi::adw_wrap_box_set_line_spacing(self.to_glib_none().0, line_spacing);
        }
    }

    #[doc(alias = "adw_wrap_box_set_line_spacing_unit")]
    #[doc(alias = "line-spacing-unit")]
    pub fn set_line_spacing_unit(&self, unit: LengthUnit) {
        unsafe {
            ffi::adw_wrap_box_set_line_spacing_unit(self.to_glib_none().0, unit.into_glib());
        }
    }

    #[doc(alias = "adw_wrap_box_set_natural_line_length")]
    #[doc(alias = "natural-line-length")]
    pub fn set_natural_line_length(&self, natural_line_length: i32) {
        unsafe {
            ffi::adw_wrap_box_set_natural_line_length(self.to_glib_none().0, natural_line_length);
        }
    }

    #[doc(alias = "adw_wrap_box_set_natural_line_length_unit")]
    #[doc(alias = "natural-line-length-unit")]
    pub fn set_natural_line_length_unit(&self, unit: LengthUnit) {
        unsafe {
            ffi::adw_wrap_box_set_natural_line_length_unit(self.to_glib_none().0, unit.into_glib());
        }
    }

    #[doc(alias = "adw_wrap_box_set_pack_direction")]
    #[doc(alias = "pack-direction")]
    pub fn set_pack_direction(&self, pack_direction: PackDirection) {
        unsafe {
            ffi::adw_wrap_box_set_pack_direction(self.to_glib_none().0, pack_direction.into_glib());
        }
    }

    #[doc(alias = "adw_wrap_box_set_wrap_policy")]
    #[doc(alias = "wrap-policy")]
    pub fn set_wrap_policy(&self, wrap_policy: WrapPolicy) {
        unsafe {
            ffi::adw_wrap_box_set_wrap_policy(self.to_glib_none().0, wrap_policy.into_glib());
        }
    }

    #[doc(alias = "adw_wrap_box_set_wrap_reverse")]
    #[doc(alias = "wrap-reverse")]
    pub fn set_wrap_reverse(&self, wrap_reverse: bool) {
        unsafe {
            ffi::adw_wrap_box_set_wrap_reverse(self.to_glib_none().0, wrap_reverse.into_glib());
        }
    }

    #[cfg(feature = "v1_7")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_7")))]
    #[doc(alias = "align")]
    pub fn connect_align_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_align_trampoline<F: Fn(&WrapBox) + 'static>(
            this: *mut ffi::AdwWrapBox,
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
                b"notify::align\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_align_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v1_7")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_7")))]
    #[doc(alias = "child-spacing")]
    pub fn connect_child_spacing_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_child_spacing_trampoline<F: Fn(&WrapBox) + 'static>(
            this: *mut ffi::AdwWrapBox,
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
                b"notify::child-spacing\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_child_spacing_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v1_7")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_7")))]
    #[doc(alias = "child-spacing-unit")]
    pub fn connect_child_spacing_unit_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_child_spacing_unit_trampoline<F: Fn(&WrapBox) + 'static>(
            this: *mut ffi::AdwWrapBox,
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
                b"notify::child-spacing-unit\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_child_spacing_unit_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v1_7")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_7")))]
    #[doc(alias = "justify")]
    pub fn connect_justify_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_justify_trampoline<F: Fn(&WrapBox) + 'static>(
            this: *mut ffi::AdwWrapBox,
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
                b"notify::justify\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_justify_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v1_7")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_7")))]
    #[doc(alias = "justify-last-line")]
    pub fn connect_justify_last_line_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_justify_last_line_trampoline<F: Fn(&WrapBox) + 'static>(
            this: *mut ffi::AdwWrapBox,
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
                b"notify::justify-last-line\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_justify_last_line_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v1_7")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_7")))]
    #[doc(alias = "line-homogeneous")]
    pub fn connect_line_homogeneous_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_line_homogeneous_trampoline<F: Fn(&WrapBox) + 'static>(
            this: *mut ffi::AdwWrapBox,
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
                b"notify::line-homogeneous\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_line_homogeneous_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v1_7")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_7")))]
    #[doc(alias = "line-spacing")]
    pub fn connect_line_spacing_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_line_spacing_trampoline<F: Fn(&WrapBox) + 'static>(
            this: *mut ffi::AdwWrapBox,
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
                b"notify::line-spacing\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_line_spacing_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v1_7")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_7")))]
    #[doc(alias = "line-spacing-unit")]
    pub fn connect_line_spacing_unit_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_line_spacing_unit_trampoline<F: Fn(&WrapBox) + 'static>(
            this: *mut ffi::AdwWrapBox,
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
                b"notify::line-spacing-unit\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_line_spacing_unit_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v1_7")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_7")))]
    #[doc(alias = "natural-line-length")]
    pub fn connect_natural_line_length_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_natural_line_length_trampoline<F: Fn(&WrapBox) + 'static>(
            this: *mut ffi::AdwWrapBox,
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
                b"notify::natural-line-length\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_natural_line_length_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v1_7")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_7")))]
    #[doc(alias = "natural-line-length-unit")]
    pub fn connect_natural_line_length_unit_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_natural_line_length_unit_trampoline<
            F: Fn(&WrapBox) + 'static,
        >(
            this: *mut ffi::AdwWrapBox,
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
                b"notify::natural-line-length-unit\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_natural_line_length_unit_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v1_7")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_7")))]
    #[doc(alias = "pack-direction")]
    pub fn connect_pack_direction_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_pack_direction_trampoline<F: Fn(&WrapBox) + 'static>(
            this: *mut ffi::AdwWrapBox,
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
                b"notify::pack-direction\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_pack_direction_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v1_7")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_7")))]
    #[doc(alias = "wrap-policy")]
    pub fn connect_wrap_policy_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_wrap_policy_trampoline<F: Fn(&WrapBox) + 'static>(
            this: *mut ffi::AdwWrapBox,
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
                b"notify::wrap-policy\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_wrap_policy_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v1_7")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_7")))]
    #[doc(alias = "wrap-reverse")]
    pub fn connect_wrap_reverse_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_wrap_reverse_trampoline<F: Fn(&WrapBox) + 'static>(
            this: *mut ffi::AdwWrapBox,
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
                b"notify::wrap-reverse\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_wrap_reverse_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

#[cfg(feature = "v1_7")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_7")))]
impl Default for WrapBox {
    fn default() -> Self {
        Self::new()
    }
}

// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`WrapBox`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct WrapBoxBuilder {
    builder: glib::object::ObjectBuilder<'static, WrapBox>,
}

impl WrapBoxBuilder {
    fn new() -> Self {
        Self {
            builder: glib::object::Object::builder(),
        }
    }

    #[cfg(feature = "v1_7")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_7")))]
    pub fn align(self, align: f32) -> Self {
        Self {
            builder: self.builder.property("align", align),
        }
    }

    #[cfg(feature = "v1_7")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_7")))]
    pub fn child_spacing(self, child_spacing: i32) -> Self {
        Self {
            builder: self.builder.property("child-spacing", child_spacing),
        }
    }

    #[cfg(feature = "v1_7")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_7")))]
    pub fn child_spacing_unit(self, child_spacing_unit: LengthUnit) -> Self {
        Self {
            builder: self
                .builder
                .property("child-spacing-unit", child_spacing_unit),
        }
    }

    #[cfg(feature = "v1_7")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_7")))]
    pub fn justify(self, justify: JustifyMode) -> Self {
        Self {
            builder: self.builder.property("justify", justify),
        }
    }

    #[cfg(feature = "v1_7")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_7")))]
    pub fn justify_last_line(self, justify_last_line: bool) -> Self {
        Self {
            builder: self
                .builder
                .property("justify-last-line", justify_last_line),
        }
    }

    #[cfg(feature = "v1_7")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_7")))]
    pub fn line_homogeneous(self, line_homogeneous: bool) -> Self {
        Self {
            builder: self.builder.property("line-homogeneous", line_homogeneous),
        }
    }

    #[cfg(feature = "v1_7")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_7")))]
    pub fn line_spacing(self, line_spacing: i32) -> Self {
        Self {
            builder: self.builder.property("line-spacing", line_spacing),
        }
    }

    #[cfg(feature = "v1_7")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_7")))]
    pub fn line_spacing_unit(self, line_spacing_unit: LengthUnit) -> Self {
        Self {
            builder: self
                .builder
                .property("line-spacing-unit", line_spacing_unit),
        }
    }

    #[cfg(feature = "v1_7")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_7")))]
    pub fn natural_line_length(self, natural_line_length: i32) -> Self {
        Self {
            builder: self
                .builder
                .property("natural-line-length", natural_line_length),
        }
    }

    #[cfg(feature = "v1_7")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_7")))]
    pub fn natural_line_length_unit(self, natural_line_length_unit: LengthUnit) -> Self {
        Self {
            builder: self
                .builder
                .property("natural-line-length-unit", natural_line_length_unit),
        }
    }

    #[cfg(feature = "v1_7")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_7")))]
    pub fn pack_direction(self, pack_direction: PackDirection) -> Self {
        Self {
            builder: self.builder.property("pack-direction", pack_direction),
        }
    }

    #[cfg(feature = "v1_7")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_7")))]
    pub fn wrap_policy(self, wrap_policy: WrapPolicy) -> Self {
        Self {
            builder: self.builder.property("wrap-policy", wrap_policy),
        }
    }

    #[cfg(feature = "v1_7")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_7")))]
    pub fn wrap_reverse(self, wrap_reverse: bool) -> Self {
        Self {
            builder: self.builder.property("wrap-reverse", wrap_reverse),
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

    pub fn orientation(self, orientation: gtk::Orientation) -> Self {
        Self {
            builder: self.builder.property("orientation", orientation),
        }
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`WrapBox`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> WrapBox {
        self.builder.build()
    }
}
