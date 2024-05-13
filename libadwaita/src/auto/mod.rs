// This file was generated by gir (https://github.com/gtk-rs/gir)
// from
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

#[cfg(feature = "v1_5")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_5")))]
mod about_dialog;
#[cfg(feature = "v1_5")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_5")))]
pub use self::about_dialog::AboutDialog;

#[cfg(feature = "v1_2")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_2")))]
mod about_window;
#[cfg(feature = "v1_2")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_2")))]
pub use self::about_window::AboutWindow;

mod action_row;
pub use self::action_row::ActionRow;

#[cfg(feature = "v1_5")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_5")))]
mod alert_dialog;
#[cfg(feature = "v1_5")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_5")))]
pub use self::alert_dialog::AlertDialog;

mod animation;
pub use self::animation::Animation;

mod animation_target;
pub use self::animation_target::AnimationTarget;

mod application;
pub use self::application::Application;

mod application_window;
pub use self::application_window::ApplicationWindow;

mod avatar;
pub use self::avatar::Avatar;

#[cfg(feature = "v1_3")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_3")))]
mod banner;
#[cfg(feature = "v1_3")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_3")))]
pub use self::banner::Banner;

mod bin;
pub use self::bin::Bin;

#[cfg(feature = "v1_4")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_4")))]
mod breakpoint;
#[cfg(feature = "v1_4")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_4")))]
pub use self::breakpoint::Breakpoint;

#[cfg(feature = "v1_4")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_4")))]
mod breakpoint_bin;
#[cfg(feature = "v1_4")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_4")))]
pub use self::breakpoint_bin::BreakpointBin;

mod button_content;
pub use self::button_content::ButtonContent;

#[cfg(feature = "v1_6")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_6")))]
mod button_row;
#[cfg(feature = "v1_6")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_6")))]
pub use self::button_row::ButtonRow;

mod callback_animation_target;
pub use self::callback_animation_target::CallbackAnimationTarget;

mod carousel;
pub use self::carousel::Carousel;

mod carousel_indicator_dots;
pub use self::carousel_indicator_dots::CarouselIndicatorDots;

mod carousel_indicator_lines;
pub use self::carousel_indicator_lines::CarouselIndicatorLines;

mod clamp;
pub use self::clamp::Clamp;

mod clamp_layout;
pub use self::clamp_layout::ClampLayout;

mod clamp_scrollable;
pub use self::clamp_scrollable::ClampScrollable;

mod combo_row;
pub use self::combo_row::ComboRow;

#[cfg(feature = "v1_5")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_5")))]
mod dialog;
#[cfg(feature = "v1_5")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_5")))]
pub use self::dialog::Dialog;

#[cfg(feature = "v1_2")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_2")))]
mod entry_row;
#[cfg(feature = "v1_2")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_2")))]
pub use self::entry_row::EntryRow;

mod enum_list_item;
pub use self::enum_list_item::EnumListItem;

mod enum_list_model;
pub use self::enum_list_model::EnumListModel;

mod expander_row;
pub use self::expander_row::ExpanderRow;

#[cfg_attr(feature = "v1_4", deprecated = "Since 1.4")]
mod flap;
#[cfg_attr(feature = "v1_4", deprecated = "Since 1.4")]
pub use self::flap::Flap;

mod header_bar;
pub use self::header_bar::HeaderBar;

#[cfg_attr(feature = "v1_4", deprecated = "Since 1.4")]
mod leaflet;
#[cfg_attr(feature = "v1_4", deprecated = "Since 1.4")]
pub use self::leaflet::Leaflet;

#[cfg_attr(feature = "v1_4", deprecated = "Since 1.4")]
mod leaflet_page;
#[cfg_attr(feature = "v1_4", deprecated = "Since 1.4")]
pub use self::leaflet_page::LeafletPage;

#[cfg(feature = "v1_2")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_2")))]
mod message_dialog;
#[cfg(feature = "v1_2")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_2")))]
pub use self::message_dialog::MessageDialog;

#[cfg(feature = "v1_4")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_4")))]
mod navigation_page;
#[cfg(feature = "v1_4")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_4")))]
pub use self::navigation_page::NavigationPage;

#[cfg(feature = "v1_4")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_4")))]
mod navigation_split_view;
#[cfg(feature = "v1_4")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_4")))]
pub use self::navigation_split_view::NavigationSplitView;

#[cfg(feature = "v1_4")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_4")))]
mod navigation_view;
#[cfg(feature = "v1_4")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_4")))]
pub use self::navigation_view::NavigationView;

#[cfg(feature = "v1_4")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_4")))]
mod overlay_split_view;
#[cfg(feature = "v1_4")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_4")))]
pub use self::overlay_split_view::OverlaySplitView;

#[cfg(feature = "v1_2")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_2")))]
mod password_entry_row;
#[cfg(feature = "v1_2")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_2")))]
pub use self::password_entry_row::PasswordEntryRow;

#[cfg(feature = "v1_5")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_5")))]
mod preferences_dialog;
#[cfg(feature = "v1_5")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_5")))]
pub use self::preferences_dialog::PreferencesDialog;

mod preferences_group;
pub use self::preferences_group::PreferencesGroup;

mod preferences_page;
pub use self::preferences_page::PreferencesPage;

mod preferences_row;
pub use self::preferences_row::PreferencesRow;

mod preferences_window;
pub use self::preferences_window::PreferencesWindow;

#[cfg(feature = "v1_2")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_2")))]
mod property_animation_target;
#[cfg(feature = "v1_2")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_2")))]
pub use self::property_animation_target::PropertyAnimationTarget;

#[cfg(feature = "v1_4")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_4")))]
mod spin_row;
#[cfg(feature = "v1_4")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_4")))]
pub use self::spin_row::SpinRow;

mod split_button;
pub use self::split_button::SplitButton;

mod spring_animation;
pub use self::spring_animation::SpringAnimation;

#[cfg_attr(feature = "v1_4", deprecated = "Since 1.4")]
mod squeezer;
#[cfg_attr(feature = "v1_4", deprecated = "Since 1.4")]
pub use self::squeezer::Squeezer;

#[cfg_attr(feature = "v1_4", deprecated = "Since 1.4")]
mod squeezer_page;
#[cfg_attr(feature = "v1_4", deprecated = "Since 1.4")]
pub use self::squeezer_page::SqueezerPage;

mod status_page;
pub use self::status_page::StatusPage;

mod style_manager;
pub use self::style_manager::StyleManager;

mod swipe_tracker;
pub use self::swipe_tracker::SwipeTracker;

mod swipeable;
pub use self::swipeable::Swipeable;

#[cfg(feature = "v1_4")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_4")))]
mod switch_row;
#[cfg(feature = "v1_4")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_4")))]
pub use self::switch_row::SwitchRow;

mod tab_bar;
pub use self::tab_bar::TabBar;

#[cfg(feature = "v1_3")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_3")))]
mod tab_button;
#[cfg(feature = "v1_3")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_3")))]
pub use self::tab_button::TabButton;

#[cfg(feature = "v1_3")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_3")))]
mod tab_overview;
#[cfg(feature = "v1_3")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_3")))]
pub use self::tab_overview::TabOverview;

mod tab_page;
pub use self::tab_page::TabPage;

mod tab_view;
pub use self::tab_view::TabView;

mod timed_animation;
pub use self::timed_animation::TimedAnimation;

mod toast;
pub use self::toast::Toast;

mod toast_overlay;
pub use self::toast_overlay::ToastOverlay;

#[cfg(feature = "v1_4")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_4")))]
mod toolbar_view;
#[cfg(feature = "v1_4")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_4")))]
pub use self::toolbar_view::ToolbarView;

mod view_stack;
pub use self::view_stack::ViewStack;

mod view_stack_page;
pub use self::view_stack_page::ViewStackPage;

#[cfg(feature = "v1_4")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_4")))]
mod view_stack_pages;
#[cfg(feature = "v1_4")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_4")))]
pub use self::view_stack_pages::ViewStackPages;

mod view_switcher;
pub use self::view_switcher::ViewSwitcher;

mod view_switcher_bar;
pub use self::view_switcher_bar::ViewSwitcherBar;

#[cfg_attr(feature = "v1_4", deprecated = "Since 1.4")]
mod view_switcher_title;
#[cfg_attr(feature = "v1_4", deprecated = "Since 1.4")]
pub use self::view_switcher_title::ViewSwitcherTitle;

mod window;
pub use self::window::Window;

mod window_title;
pub use self::window_title::WindowTitle;

#[cfg(feature = "v1_4")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_4")))]
mod breakpoint_condition;
#[cfg(feature = "v1_4")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_4")))]
pub use self::breakpoint_condition::BreakpointCondition;

mod spring_params;
pub use self::spring_params::SpringParams;

mod enums;
pub use self::enums::AnimationState;
#[cfg(feature = "v1_4")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_4")))]
pub use self::enums::BreakpointConditionLengthType;
#[cfg(feature = "v1_4")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_4")))]
pub use self::enums::BreakpointConditionRatioType;
pub use self::enums::CenteringPolicy;
pub use self::enums::ColorScheme;
#[cfg(feature = "v1_5")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_5")))]
pub use self::enums::DialogPresentationMode;
pub use self::enums::Easing;
#[allow(deprecated)]
pub use self::enums::FlapFoldPolicy;
#[allow(deprecated)]
pub use self::enums::FlapTransitionType;
#[allow(deprecated)]
pub use self::enums::FoldThresholdPolicy;
#[allow(deprecated)]
pub use self::enums::LeafletTransitionType;
#[cfg(feature = "v1_4")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_4")))]
pub use self::enums::LengthUnit;
pub use self::enums::NavigationDirection;
#[cfg(feature = "v1_2")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_2")))]
pub use self::enums::ResponseAppearance;
#[allow(deprecated)]
pub use self::enums::SqueezerTransitionType;
pub use self::enums::ToastPriority;
#[cfg(feature = "v1_4")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_4")))]
pub use self::enums::ToolbarStyle;
pub use self::enums::ViewSwitcherPolicy;

mod flags;
#[cfg(feature = "v1_2")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_2")))]
pub use self::flags::TabViewShortcuts;

pub(crate) mod functions;

pub(crate) mod traits {
    pub use super::action_row::ActionRowExt;
    #[cfg(feature = "v1_5")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_5")))]
    pub use super::alert_dialog::AlertDialogExt;
    pub use super::animation::AnimationExt;
    pub use super::application::AdwApplicationExt;
    pub use super::application_window::AdwApplicationWindowExt;
    pub use super::bin::BinExt;
    #[cfg(feature = "v1_4")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_4")))]
    pub use super::breakpoint_bin::BreakpointBinExt;
    pub use super::combo_row::ComboRowExt;
    #[cfg(feature = "v1_5")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_5")))]
    pub use super::dialog::AdwDialogExt;
    #[cfg(feature = "v1_2")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_2")))]
    pub use super::entry_row::EntryRowExt;
    pub use super::expander_row::ExpanderRowExt;
    #[cfg(feature = "v1_2")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_2")))]
    pub use super::message_dialog::MessageDialogExt;
    #[cfg(feature = "v1_4")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_4")))]
    pub use super::navigation_page::NavigationPageExt;
    #[cfg(feature = "v1_5")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_5")))]
    pub use super::preferences_dialog::PreferencesDialogExt;
    pub use super::preferences_group::PreferencesGroupExt;
    pub use super::preferences_page::PreferencesPageExt;
    pub use super::preferences_row::PreferencesRowExt;
    pub use super::preferences_window::PreferencesWindowExt;
    pub use super::swipeable::SwipeableExt;
    pub use super::window::AdwWindowExt;
}
pub(crate) mod builders {
    #[cfg(feature = "v1_5")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_5")))]
    pub use super::about_dialog::AboutDialogBuilder;
    #[cfg(feature = "v1_2")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_2")))]
    pub use super::about_window::AboutWindowBuilder;
    pub use super::action_row::ActionRowBuilder;
    #[cfg(feature = "v1_5")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_5")))]
    pub use super::alert_dialog::AlertDialogBuilder;
    pub use super::application::ApplicationBuilder;
    pub use super::application_window::ApplicationWindowBuilder;
    pub use super::avatar::AvatarBuilder;
    #[cfg(feature = "v1_3")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_3")))]
    pub use super::banner::BannerBuilder;
    pub use super::bin::BinBuilder;
    #[cfg(feature = "v1_4")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_4")))]
    pub use super::breakpoint_bin::BreakpointBinBuilder;
    pub use super::button_content::ButtonContentBuilder;
    #[cfg(feature = "v1_6")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_6")))]
    pub use super::button_row::ButtonRowBuilder;
    pub use super::carousel::CarouselBuilder;
    pub use super::carousel_indicator_dots::CarouselIndicatorDotsBuilder;
    pub use super::carousel_indicator_lines::CarouselIndicatorLinesBuilder;
    pub use super::clamp::ClampBuilder;
    pub use super::clamp_layout::ClampLayoutBuilder;
    pub use super::clamp_scrollable::ClampScrollableBuilder;
    pub use super::combo_row::ComboRowBuilder;
    #[cfg(feature = "v1_5")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_5")))]
    pub use super::dialog::DialogBuilder;
    #[cfg(feature = "v1_2")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_2")))]
    pub use super::entry_row::EntryRowBuilder;
    pub use super::expander_row::ExpanderRowBuilder;
    #[cfg_attr(feature = "v1_4", deprecated = "Since 1.4")]
    pub use super::flap::FlapBuilder;
    pub use super::header_bar::HeaderBarBuilder;
    #[cfg_attr(feature = "v1_4", deprecated = "Since 1.4")]
    pub use super::leaflet::LeafletBuilder;
    #[cfg(feature = "v1_2")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_2")))]
    pub use super::message_dialog::MessageDialogBuilder;
    #[cfg(feature = "v1_4")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_4")))]
    pub use super::navigation_page::NavigationPageBuilder;
    #[cfg(feature = "v1_4")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_4")))]
    pub use super::navigation_split_view::NavigationSplitViewBuilder;
    #[cfg(feature = "v1_4")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_4")))]
    pub use super::navigation_view::NavigationViewBuilder;
    #[cfg(feature = "v1_4")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_4")))]
    pub use super::overlay_split_view::OverlaySplitViewBuilder;
    #[cfg(feature = "v1_2")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_2")))]
    pub use super::password_entry_row::PasswordEntryRowBuilder;
    #[cfg(feature = "v1_5")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_5")))]
    pub use super::preferences_dialog::PreferencesDialogBuilder;
    pub use super::preferences_group::PreferencesGroupBuilder;
    pub use super::preferences_page::PreferencesPageBuilder;
    pub use super::preferences_row::PreferencesRowBuilder;
    pub use super::preferences_window::PreferencesWindowBuilder;
    #[cfg(feature = "v1_4")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_4")))]
    pub use super::spin_row::SpinRowBuilder;
    pub use super::split_button::SplitButtonBuilder;
    pub use super::spring_animation::SpringAnimationBuilder;
    #[cfg_attr(feature = "v1_4", deprecated = "Since 1.4")]
    pub use super::squeezer::SqueezerBuilder;
    pub use super::status_page::StatusPageBuilder;
    pub use super::swipe_tracker::SwipeTrackerBuilder;
    #[cfg(feature = "v1_4")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_4")))]
    pub use super::switch_row::SwitchRowBuilder;
    pub use super::tab_bar::TabBarBuilder;
    #[cfg(feature = "v1_3")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_3")))]
    pub use super::tab_button::TabButtonBuilder;
    #[cfg(feature = "v1_3")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_3")))]
    pub use super::tab_overview::TabOverviewBuilder;
    pub use super::tab_view::TabViewBuilder;
    pub use super::timed_animation::TimedAnimationBuilder;
    pub use super::toast::ToastBuilder;
    #[cfg(feature = "v1_4")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_4")))]
    pub use super::toolbar_view::ToolbarViewBuilder;
    pub use super::view_stack::ViewStackBuilder;
    pub use super::view_switcher::ViewSwitcherBuilder;
    pub use super::view_switcher_bar::ViewSwitcherBarBuilder;
    #[cfg_attr(feature = "v1_4", deprecated = "Since 1.4")]
    pub use super::view_switcher_title::ViewSwitcherTitleBuilder;
    pub use super::window::WindowBuilder;
    pub use super::window_title::WindowTitleBuilder;
}
