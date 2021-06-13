// This file was generated by gir (https://github.com/gtk-rs/gir)
// from
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

mod action_row;
pub use self::action_row::ActionRowBuilder;
pub use self::action_row::{ActionRow, NONE_ACTION_ROW};

mod application_window;
pub use self::application_window::ApplicationWindowBuilder;
pub use self::application_window::{ApplicationWindow, NONE_APPLICATION_WINDOW};

mod avatar;
pub use self::avatar::Avatar;
pub use self::avatar::AvatarBuilder;

mod bin;
pub use self::bin::BinBuilder;
pub use self::bin::{Bin, NONE_BIN};

mod carousel;
pub use self::carousel::Carousel;
pub use self::carousel::CarouselBuilder;

mod carousel_indicator_dots;
pub use self::carousel_indicator_dots::CarouselIndicatorDots;
pub use self::carousel_indicator_dots::CarouselIndicatorDotsBuilder;

mod carousel_indicator_lines;
pub use self::carousel_indicator_lines::CarouselIndicatorLines;
pub use self::carousel_indicator_lines::CarouselIndicatorLinesBuilder;

mod clamp;
pub use self::clamp::Clamp;
pub use self::clamp::ClampBuilder;

mod clamp_layout;
pub use self::clamp_layout::ClampLayout;
pub use self::clamp_layout::ClampLayoutBuilder;

mod clamp_scrollable;
pub use self::clamp_scrollable::ClampScrollable;
pub use self::clamp_scrollable::ClampScrollableBuilder;

mod combo_row;
pub use self::combo_row::{ComboRow, NONE_COMBO_ROW};

mod enum_list_model;
pub use self::enum_list_model::EnumListModel;

mod enum_value_object;
pub use self::enum_value_object::EnumValueObject;

mod expander_row;
pub use self::expander_row::ExpanderRowBuilder;
pub use self::expander_row::{ExpanderRow, NONE_EXPANDER_ROW};

mod flap;
pub use self::flap::Flap;
pub use self::flap::FlapBuilder;

mod header_bar;
pub use self::header_bar::HeaderBar;
pub use self::header_bar::HeaderBarBuilder;

mod leaflet;
pub use self::leaflet::Leaflet;
pub use self::leaflet::LeafletBuilder;

mod leaflet_page;
pub use self::leaflet_page::LeafletPage;

mod preferences_group;
pub use self::preferences_group::PreferencesGroupBuilder;
pub use self::preferences_group::{PreferencesGroup, NONE_PREFERENCES_GROUP};

mod preferences_page;
pub use self::preferences_page::PreferencesPageBuilder;
pub use self::preferences_page::{PreferencesPage, NONE_PREFERENCES_PAGE};

mod preferences_row;
pub use self::preferences_row::PreferencesRowBuilder;
pub use self::preferences_row::{PreferencesRow, NONE_PREFERENCES_ROW};

mod preferences_window;
pub use self::preferences_window::PreferencesWindowBuilder;
pub use self::preferences_window::{PreferencesWindow, NONE_PREFERENCES_WINDOW};

mod squeezer;
pub use self::squeezer::Squeezer;
pub use self::squeezer::SqueezerBuilder;

mod squeezer_page;
pub use self::squeezer_page::SqueezerPage;

mod status_page;
pub use self::status_page::StatusPage;
pub use self::status_page::StatusPageBuilder;

mod swipe_tracker;
pub use self::swipe_tracker::SwipeTracker;
pub use self::swipe_tracker::SwipeTrackerBuilder;

mod swipeable;
pub use self::swipeable::{Swipeable, NONE_SWIPEABLE};

mod tab_bar;
pub use self::tab_bar::TabBar;
pub use self::tab_bar::TabBarBuilder;

mod tab_page;
pub use self::tab_page::TabPage;

mod tab_view;
pub use self::tab_view::TabView;
pub use self::tab_view::TabViewBuilder;

mod value_object;
pub use self::value_object::ValueObject;

mod view_switcher;
pub use self::view_switcher::ViewSwitcher;
pub use self::view_switcher::ViewSwitcherBuilder;

mod view_switcher_bar;
pub use self::view_switcher_bar::ViewSwitcherBar;
pub use self::view_switcher_bar::ViewSwitcherBarBuilder;

mod view_switcher_title;
pub use self::view_switcher_title::ViewSwitcherTitle;
pub use self::view_switcher_title::ViewSwitcherTitleBuilder;

mod window;
pub use self::window::WindowBuilder;
pub use self::window::{Window, NONE_WINDOW};

mod window_title;
pub use self::window_title::WindowTitle;
pub use self::window_title::WindowTitleBuilder;

mod enums;
pub use self::enums::CenteringPolicy;
pub use self::enums::FlapFoldPolicy;
pub use self::enums::FlapTransitionType;
pub use self::enums::LeafletTransitionType;
pub use self::enums::NavigationDirection;
pub use self::enums::SqueezerTransitionType;
pub use self::enums::ViewSwitcherPolicy;

pub mod functions;

#[doc(hidden)]
pub mod traits {
    pub use super::action_row::ActionRowExt;
    pub use super::application_window::ApplicationWindowExt;
    pub use super::bin::BinExt;
    pub use super::combo_row::ComboRowExt;
    pub use super::expander_row::ExpanderRowExt;
    pub use super::preferences_group::PreferencesGroupExt;
    pub use super::preferences_page::PreferencesPageExt;
    pub use super::preferences_row::PreferencesRowExt;
    pub use super::preferences_window::PreferencesWindowExt;
    pub use super::swipeable::SwipeableExt;
    pub use super::window::WindowExt;
}
