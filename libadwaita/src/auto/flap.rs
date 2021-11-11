// This file was generated by gir (https://github.com/gtk-rs/gir)
// from
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::FlapFoldPolicy;
use crate::FlapTransitionType;
use crate::FoldThresholdPolicy;
use crate::Swipeable;
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
    #[doc(alias = "AdwFlap")]
    pub struct Flap(Object<ffi::AdwFlap, ffi::AdwFlapClass>) @extends gtk::Widget, @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, Swipeable, gtk::Orientable;

    match fn {
        type_ => || ffi::adw_flap_get_type(),
    }
}

impl Flap {
    #[doc(alias = "adw_flap_new")]
    pub fn new() -> Flap {
        assert_initialized_main_thread!();
        unsafe { gtk::Widget::from_glib_none(ffi::adw_flap_new()).unsafe_cast() }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`Flap`] objects.
    ///
    /// This method returns an instance of [`FlapBuilder`] which can be used to create [`Flap`] objects.
    pub fn builder() -> FlapBuilder {
        FlapBuilder::default()
    }

    #[doc(alias = "adw_flap_get_content")]
    #[doc(alias = "get_content")]
    pub fn content(&self) -> Option<gtk::Widget> {
        unsafe { from_glib_none(ffi::adw_flap_get_content(self.to_glib_none().0)) }
    }

    #[doc(alias = "adw_flap_get_flap")]
    #[doc(alias = "get_flap")]
    pub fn flap(&self) -> Option<gtk::Widget> {
        unsafe { from_glib_none(ffi::adw_flap_get_flap(self.to_glib_none().0)) }
    }

    #[doc(alias = "adw_flap_get_flap_position")]
    #[doc(alias = "get_flap_position")]
    pub fn flap_position(&self) -> gtk::PackType {
        unsafe { from_glib(ffi::adw_flap_get_flap_position(self.to_glib_none().0)) }
    }

    #[doc(alias = "adw_flap_get_fold_duration")]
    #[doc(alias = "get_fold_duration")]
    pub fn fold_duration(&self) -> u32 {
        unsafe { ffi::adw_flap_get_fold_duration(self.to_glib_none().0) }
    }

    #[doc(alias = "adw_flap_get_fold_policy")]
    #[doc(alias = "get_fold_policy")]
    pub fn fold_policy(&self) -> FlapFoldPolicy {
        unsafe { from_glib(ffi::adw_flap_get_fold_policy(self.to_glib_none().0)) }
    }

    #[doc(alias = "adw_flap_get_fold_threshold_policy")]
    #[doc(alias = "get_fold_threshold_policy")]
    pub fn fold_threshold_policy(&self) -> FoldThresholdPolicy {
        unsafe {
            from_glib(ffi::adw_flap_get_fold_threshold_policy(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "adw_flap_get_folded")]
    #[doc(alias = "get_folded")]
    pub fn is_folded(&self) -> bool {
        unsafe { from_glib(ffi::adw_flap_get_folded(self.to_glib_none().0)) }
    }

    #[doc(alias = "adw_flap_get_locked")]
    #[doc(alias = "get_locked")]
    pub fn is_locked(&self) -> bool {
        unsafe { from_glib(ffi::adw_flap_get_locked(self.to_glib_none().0)) }
    }

    #[doc(alias = "adw_flap_get_modal")]
    #[doc(alias = "get_modal")]
    pub fn is_modal(&self) -> bool {
        unsafe { from_glib(ffi::adw_flap_get_modal(self.to_glib_none().0)) }
    }

    #[doc(alias = "adw_flap_get_reveal_duration")]
    #[doc(alias = "get_reveal_duration")]
    pub fn reveal_duration(&self) -> u32 {
        unsafe { ffi::adw_flap_get_reveal_duration(self.to_glib_none().0) }
    }

    #[doc(alias = "adw_flap_get_reveal_flap")]
    #[doc(alias = "get_reveal_flap")]
    pub fn reveals_flap(&self) -> bool {
        unsafe { from_glib(ffi::adw_flap_get_reveal_flap(self.to_glib_none().0)) }
    }

    #[doc(alias = "adw_flap_get_reveal_progress")]
    #[doc(alias = "get_reveal_progress")]
    pub fn reveal_progress(&self) -> f64 {
        unsafe { ffi::adw_flap_get_reveal_progress(self.to_glib_none().0) }
    }

    #[doc(alias = "adw_flap_get_separator")]
    #[doc(alias = "get_separator")]
    pub fn separator(&self) -> Option<gtk::Widget> {
        unsafe { from_glib_none(ffi::adw_flap_get_separator(self.to_glib_none().0)) }
    }

    #[doc(alias = "adw_flap_get_swipe_to_close")]
    #[doc(alias = "get_swipe_to_close")]
    pub fn is_swipe_to_close(&self) -> bool {
        unsafe { from_glib(ffi::adw_flap_get_swipe_to_close(self.to_glib_none().0)) }
    }

    #[doc(alias = "adw_flap_get_swipe_to_open")]
    #[doc(alias = "get_swipe_to_open")]
    pub fn is_swipe_to_open(&self) -> bool {
        unsafe { from_glib(ffi::adw_flap_get_swipe_to_open(self.to_glib_none().0)) }
    }

    #[doc(alias = "adw_flap_get_transition_type")]
    #[doc(alias = "get_transition_type")]
    pub fn transition_type(&self) -> FlapTransitionType {
        unsafe { from_glib(ffi::adw_flap_get_transition_type(self.to_glib_none().0)) }
    }

    #[doc(alias = "adw_flap_set_content")]
    pub fn set_content(&self, content: Option<&impl IsA<gtk::Widget>>) {
        unsafe {
            ffi::adw_flap_set_content(
                self.to_glib_none().0,
                content.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    #[doc(alias = "adw_flap_set_flap")]
    pub fn set_flap(&self, flap: Option<&impl IsA<gtk::Widget>>) {
        unsafe {
            ffi::adw_flap_set_flap(
                self.to_glib_none().0,
                flap.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    #[doc(alias = "adw_flap_set_flap_position")]
    pub fn set_flap_position(&self, position: gtk::PackType) {
        unsafe {
            ffi::adw_flap_set_flap_position(self.to_glib_none().0, position.into_glib());
        }
    }

    #[doc(alias = "adw_flap_set_fold_duration")]
    pub fn set_fold_duration(&self, duration: u32) {
        unsafe {
            ffi::adw_flap_set_fold_duration(self.to_glib_none().0, duration);
        }
    }

    #[doc(alias = "adw_flap_set_fold_policy")]
    pub fn set_fold_policy(&self, policy: FlapFoldPolicy) {
        unsafe {
            ffi::adw_flap_set_fold_policy(self.to_glib_none().0, policy.into_glib());
        }
    }

    #[doc(alias = "adw_flap_set_fold_threshold_policy")]
    pub fn set_fold_threshold_policy(&self, policy: FoldThresholdPolicy) {
        unsafe {
            ffi::adw_flap_set_fold_threshold_policy(self.to_glib_none().0, policy.into_glib());
        }
    }

    #[doc(alias = "adw_flap_set_locked")]
    pub fn set_locked(&self, locked: bool) {
        unsafe {
            ffi::adw_flap_set_locked(self.to_glib_none().0, locked.into_glib());
        }
    }

    #[doc(alias = "adw_flap_set_modal")]
    pub fn set_modal(&self, modal: bool) {
        unsafe {
            ffi::adw_flap_set_modal(self.to_glib_none().0, modal.into_glib());
        }
    }

    #[doc(alias = "adw_flap_set_reveal_duration")]
    pub fn set_reveal_duration(&self, duration: u32) {
        unsafe {
            ffi::adw_flap_set_reveal_duration(self.to_glib_none().0, duration);
        }
    }

    #[doc(alias = "adw_flap_set_reveal_flap")]
    pub fn set_reveal_flap(&self, reveal_flap: bool) {
        unsafe {
            ffi::adw_flap_set_reveal_flap(self.to_glib_none().0, reveal_flap.into_glib());
        }
    }

    #[doc(alias = "adw_flap_set_separator")]
    pub fn set_separator(&self, separator: Option<&impl IsA<gtk::Widget>>) {
        unsafe {
            ffi::adw_flap_set_separator(
                self.to_glib_none().0,
                separator.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    #[doc(alias = "adw_flap_set_swipe_to_close")]
    pub fn set_swipe_to_close(&self, swipe_to_close: bool) {
        unsafe {
            ffi::adw_flap_set_swipe_to_close(self.to_glib_none().0, swipe_to_close.into_glib());
        }
    }

    #[doc(alias = "adw_flap_set_swipe_to_open")]
    pub fn set_swipe_to_open(&self, swipe_to_open: bool) {
        unsafe {
            ffi::adw_flap_set_swipe_to_open(self.to_glib_none().0, swipe_to_open.into_glib());
        }
    }

    #[doc(alias = "adw_flap_set_transition_type")]
    pub fn set_transition_type(&self, transition_type: FlapTransitionType) {
        unsafe {
            ffi::adw_flap_set_transition_type(self.to_glib_none().0, transition_type.into_glib());
        }
    }

    #[doc(alias = "content")]
    pub fn connect_content_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_content_trampoline<F: Fn(&Flap) + 'static>(
            this: *mut ffi::AdwFlap,
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
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_content_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "flap")]
    pub fn connect_flap_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_flap_trampoline<F: Fn(&Flap) + 'static>(
            this: *mut ffi::AdwFlap,
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
                b"notify::flap\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_flap_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "flap-position")]
    pub fn connect_flap_position_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_flap_position_trampoline<F: Fn(&Flap) + 'static>(
            this: *mut ffi::AdwFlap,
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
                b"notify::flap-position\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_flap_position_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "fold-duration")]
    pub fn connect_fold_duration_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_fold_duration_trampoline<F: Fn(&Flap) + 'static>(
            this: *mut ffi::AdwFlap,
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
                b"notify::fold-duration\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_fold_duration_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "fold-policy")]
    pub fn connect_fold_policy_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_fold_policy_trampoline<F: Fn(&Flap) + 'static>(
            this: *mut ffi::AdwFlap,
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
                b"notify::fold-policy\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_fold_policy_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "fold-threshold-policy")]
    pub fn connect_fold_threshold_policy_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_fold_threshold_policy_trampoline<F: Fn(&Flap) + 'static>(
            this: *mut ffi::AdwFlap,
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
                b"notify::fold-threshold-policy\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_fold_threshold_policy_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "folded")]
    pub fn connect_folded_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_folded_trampoline<F: Fn(&Flap) + 'static>(
            this: *mut ffi::AdwFlap,
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
                b"notify::folded\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_folded_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "locked")]
    pub fn connect_locked_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_locked_trampoline<F: Fn(&Flap) + 'static>(
            this: *mut ffi::AdwFlap,
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
                b"notify::locked\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_locked_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "modal")]
    pub fn connect_modal_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_modal_trampoline<F: Fn(&Flap) + 'static>(
            this: *mut ffi::AdwFlap,
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
                b"notify::modal\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_modal_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "reveal-duration")]
    pub fn connect_reveal_duration_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_reveal_duration_trampoline<F: Fn(&Flap) + 'static>(
            this: *mut ffi::AdwFlap,
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
                b"notify::reveal-duration\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_reveal_duration_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "reveal-flap")]
    pub fn connect_reveal_flap_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_reveal_flap_trampoline<F: Fn(&Flap) + 'static>(
            this: *mut ffi::AdwFlap,
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
                b"notify::reveal-flap\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_reveal_flap_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "reveal-progress")]
    pub fn connect_reveal_progress_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_reveal_progress_trampoline<F: Fn(&Flap) + 'static>(
            this: *mut ffi::AdwFlap,
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
                b"notify::reveal-progress\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_reveal_progress_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "separator")]
    pub fn connect_separator_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_separator_trampoline<F: Fn(&Flap) + 'static>(
            this: *mut ffi::AdwFlap,
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
                b"notify::separator\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_separator_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "swipe-to-close")]
    pub fn connect_swipe_to_close_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_swipe_to_close_trampoline<F: Fn(&Flap) + 'static>(
            this: *mut ffi::AdwFlap,
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
                b"notify::swipe-to-close\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_swipe_to_close_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "swipe-to-open")]
    pub fn connect_swipe_to_open_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_swipe_to_open_trampoline<F: Fn(&Flap) + 'static>(
            this: *mut ffi::AdwFlap,
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
                b"notify::swipe-to-open\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_swipe_to_open_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "transition-type")]
    pub fn connect_transition_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_transition_type_trampoline<F: Fn(&Flap) + 'static>(
            this: *mut ffi::AdwFlap,
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
                b"notify::transition-type\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_transition_type_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl Default for Flap {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Default)]
// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`Flap`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
pub struct FlapBuilder {
    content: Option<gtk::Widget>,
    flap: Option<gtk::Widget>,
    flap_position: Option<gtk::PackType>,
    fold_duration: Option<u32>,
    fold_policy: Option<FlapFoldPolicy>,
    fold_threshold_policy: Option<FoldThresholdPolicy>,
    locked: Option<bool>,
    modal: Option<bool>,
    reveal_duration: Option<u32>,
    reveal_flap: Option<bool>,
    separator: Option<gtk::Widget>,
    swipe_to_close: Option<bool>,
    swipe_to_open: Option<bool>,
    transition_type: Option<FlapTransitionType>,
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
    orientation: Option<gtk::Orientation>,
}

impl FlapBuilder {
    // rustdoc-stripper-ignore-next
    /// Create a new [`FlapBuilder`].
    pub fn new() -> Self {
        Self::default()
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`Flap`].
    pub fn build(self) -> Flap {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref content) = self.content {
            properties.push(("content", content));
        }
        if let Some(ref flap) = self.flap {
            properties.push(("flap", flap));
        }
        if let Some(ref flap_position) = self.flap_position {
            properties.push(("flap-position", flap_position));
        }
        if let Some(ref fold_duration) = self.fold_duration {
            properties.push(("fold-duration", fold_duration));
        }
        if let Some(ref fold_policy) = self.fold_policy {
            properties.push(("fold-policy", fold_policy));
        }
        if let Some(ref fold_threshold_policy) = self.fold_threshold_policy {
            properties.push(("fold-threshold-policy", fold_threshold_policy));
        }
        if let Some(ref locked) = self.locked {
            properties.push(("locked", locked));
        }
        if let Some(ref modal) = self.modal {
            properties.push(("modal", modal));
        }
        if let Some(ref reveal_duration) = self.reveal_duration {
            properties.push(("reveal-duration", reveal_duration));
        }
        if let Some(ref reveal_flap) = self.reveal_flap {
            properties.push(("reveal-flap", reveal_flap));
        }
        if let Some(ref separator) = self.separator {
            properties.push(("separator", separator));
        }
        if let Some(ref swipe_to_close) = self.swipe_to_close {
            properties.push(("swipe-to-close", swipe_to_close));
        }
        if let Some(ref swipe_to_open) = self.swipe_to_open {
            properties.push(("swipe-to-open", swipe_to_open));
        }
        if let Some(ref transition_type) = self.transition_type {
            properties.push(("transition-type", transition_type));
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
        if let Some(ref orientation) = self.orientation {
            properties.push(("orientation", orientation));
        }
        glib::Object::new::<Flap>(&properties).expect("Failed to create an instance of Flap")
    }

    pub fn content(mut self, content: &impl IsA<gtk::Widget>) -> Self {
        self.content = Some(content.clone().upcast());
        self
    }

    pub fn flap(mut self, flap: &impl IsA<gtk::Widget>) -> Self {
        self.flap = Some(flap.clone().upcast());
        self
    }

    pub fn flap_position(mut self, flap_position: gtk::PackType) -> Self {
        self.flap_position = Some(flap_position);
        self
    }

    pub fn fold_duration(mut self, fold_duration: u32) -> Self {
        self.fold_duration = Some(fold_duration);
        self
    }

    pub fn fold_policy(mut self, fold_policy: FlapFoldPolicy) -> Self {
        self.fold_policy = Some(fold_policy);
        self
    }

    pub fn fold_threshold_policy(mut self, fold_threshold_policy: FoldThresholdPolicy) -> Self {
        self.fold_threshold_policy = Some(fold_threshold_policy);
        self
    }

    pub fn locked(mut self, locked: bool) -> Self {
        self.locked = Some(locked);
        self
    }

    pub fn modal(mut self, modal: bool) -> Self {
        self.modal = Some(modal);
        self
    }

    pub fn reveal_duration(mut self, reveal_duration: u32) -> Self {
        self.reveal_duration = Some(reveal_duration);
        self
    }

    pub fn reveal_flap(mut self, reveal_flap: bool) -> Self {
        self.reveal_flap = Some(reveal_flap);
        self
    }

    pub fn separator(mut self, separator: &impl IsA<gtk::Widget>) -> Self {
        self.separator = Some(separator.clone().upcast());
        self
    }

    pub fn swipe_to_close(mut self, swipe_to_close: bool) -> Self {
        self.swipe_to_close = Some(swipe_to_close);
        self
    }

    pub fn swipe_to_open(mut self, swipe_to_open: bool) -> Self {
        self.swipe_to_open = Some(swipe_to_open);
        self
    }

    pub fn transition_type(mut self, transition_type: FlapTransitionType) -> Self {
        self.transition_type = Some(transition_type);
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

    pub fn orientation(mut self, orientation: gtk::Orientation) -> Self {
        self.orientation = Some(orientation);
        self
    }
}

impl fmt::Display for Flap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Flap")
    }
}
