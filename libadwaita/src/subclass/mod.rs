pub mod action_row;
pub mod application;
pub mod application_window;
pub mod bin;
#[cfg(feature = "v1_4")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_4")))]
pub mod breakpoint_bin;
pub mod combo_row;
#[cfg(feature = "v1_2")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_2")))]
pub mod entry_row;
pub mod expander_row;
#[cfg(feature = "v1_2")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_2")))]
mod message_dialog;
#[cfg(feature = "v1_4")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_4")))]
pub mod navigation_page;
pub mod preferences_group;
pub mod preferences_page;
pub mod preferences_row;
pub mod preferences_window;
#[cfg(feature = "v1_4")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_4")))]
pub mod spin_row;
pub mod swipeable;
pub mod window;

pub mod prelude {
    pub use super::action_row::ActionRowImpl;
    pub use super::application::AdwApplicationImpl;
    pub use super::application_window::AdwApplicationWindowImpl;
    pub use super::bin::BinImpl;
    #[cfg(feature = "v1_4")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_4")))]
    pub use super::breakpoint_bin::BreakpointBinImpl;
    pub use super::combo_row::ComboRowImpl;
    #[cfg(feature = "v1_2")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_2")))]
    pub use super::entry_row::EntryRowImpl;
    pub use super::expander_row::ExpanderRowImpl;
    #[cfg(feature = "v1_2")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_2")))]
    pub use super::message_dialog::{MessageDialogImpl, MessageDialogImplExt};
    #[cfg(feature = "v1_4")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_4")))]
    pub use super::navigation_page::NavigationPageImpl;
    pub use super::preferences_group::PreferencesGroupImpl;
    pub use super::preferences_page::PreferencesPageImpl;
    pub use super::preferences_row::PreferencesRowImpl;
    pub use super::preferences_window::PreferencesWindowImpl;
    #[cfg(feature = "v1_4")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_4")))]
    pub use super::spin_row::SpinRowImpl;
    pub use super::swipeable::SwipeableImpl;
    pub use super::window::AdwWindowImpl;
    pub use gtk::subclass::prelude::*;
}
