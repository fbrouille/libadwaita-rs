// Generated by gir (https://github.com/gtk-rs/gir @ 33cf4d29f97b)
// from
// from gir-files (https://github.com/gtk-rs/gir-files.git @ 21b29d0e0c1a)
// DO NOT EDIT

#![cfg(unix)]

use libadwaita_sys::*;
use std::env;
use std::error::Error;
use std::ffi::OsString;
use std::mem::{align_of, size_of};
use std::path::Path;
use std::process::{Command, Stdio};
use std::str;
use tempfile::Builder;

static PACKAGES: &[&str] = &["libadwaita-1"];

#[derive(Clone, Debug)]
struct Compiler {
    pub args: Vec<String>,
}

impl Compiler {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let mut args = get_var("CC", "cc")?;
        args.push("-Wno-deprecated-declarations".to_owned());
        // For _Generic
        args.push("-std=c11".to_owned());
        // For %z support in printf when using MinGW.
        args.push("-D__USE_MINGW_ANSI_STDIO".to_owned());
        args.extend(get_var("CFLAGS", "")?);
        args.extend(get_var("CPPFLAGS", "")?);
        args.extend(pkg_config_cflags(PACKAGES)?);
        Ok(Self { args })
    }

    pub fn compile(&self, src: &Path, out: &Path) -> Result<(), Box<dyn Error>> {
        let mut cmd = self.to_command();
        cmd.arg(src);
        cmd.arg("-o");
        cmd.arg(out);
        let status = cmd.spawn()?.wait()?;
        if !status.success() {
            return Err(format!("compilation command {cmd:?} failed, {status}").into());
        }
        Ok(())
    }

    fn to_command(&self) -> Command {
        let mut cmd = Command::new(&self.args[0]);
        cmd.args(&self.args[1..]);
        cmd
    }
}

fn get_var(name: &str, default: &str) -> Result<Vec<String>, Box<dyn Error>> {
    match env::var(name) {
        Ok(value) => Ok(shell_words::split(&value)?),
        Err(env::VarError::NotPresent) => Ok(shell_words::split(default)?),
        Err(err) => Err(format!("{name} {err}").into()),
    }
}

fn pkg_config_cflags(packages: &[&str]) -> Result<Vec<String>, Box<dyn Error>> {
    if packages.is_empty() {
        return Ok(Vec::new());
    }
    let pkg_config = env::var_os("PKG_CONFIG").unwrap_or_else(|| OsString::from("pkg-config"));
    let mut cmd = Command::new(pkg_config);
    cmd.arg("--cflags");
    cmd.args(packages);
    cmd.stderr(Stdio::inherit());
    let out = cmd.output()?;
    if !out.status.success() {
        let (status, stdout) = (out.status, String::from_utf8_lossy(&out.stdout));
        return Err(format!("command {cmd:?} failed, {status:?}\nstdout: {stdout}").into());
    }
    let stdout = str::from_utf8(&out.stdout)?;
    Ok(shell_words::split(stdout.trim())?)
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct Layout {
    size: usize,
    alignment: usize,
}

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
struct Results {
    /// Number of successfully completed tests.
    passed: usize,
    /// Total number of failed tests (including those that failed to compile).
    failed: usize,
}

impl Results {
    fn record_passed(&mut self) {
        self.passed += 1;
    }
    fn record_failed(&mut self) {
        self.failed += 1;
    }
    fn summary(&self) -> String {
        format!("{} passed; {} failed", self.passed, self.failed)
    }
    fn expect_total_success(&self) {
        if self.failed == 0 {
            println!("OK: {}", self.summary());
        } else {
            panic!("FAILED: {}", self.summary());
        };
    }
}

#[test]
fn cross_validate_constants_with_c() {
    let mut c_constants: Vec<(String, String)> = Vec::new();

    for l in get_c_output("constant").unwrap().lines() {
        let (name, value) = l.split_once(';').expect("Missing ';' separator");
        c_constants.push((name.to_owned(), value.to_owned()));
    }

    let mut results = Results::default();

    for ((rust_name, rust_value), (c_name, c_value)) in
        RUST_CONSTANTS.iter().zip(c_constants.iter())
    {
        if rust_name != c_name {
            results.record_failed();
            eprintln!("Name mismatch:\nRust: {rust_name:?}\nC:    {c_name:?}");
            continue;
        }

        if rust_value != c_value {
            results.record_failed();
            eprintln!(
                "Constant value mismatch for {rust_name}\nRust: {rust_value:?}\nC:    {c_value:?}",
            );
            continue;
        }

        results.record_passed();
    }

    results.expect_total_success();
}

#[test]
fn cross_validate_layout_with_c() {
    let mut c_layouts = Vec::new();

    for l in get_c_output("layout").unwrap().lines() {
        let (name, value) = l.split_once(';').expect("Missing first ';' separator");
        let (size, alignment) = value.split_once(';').expect("Missing second ';' separator");
        let size = size.parse().expect("Failed to parse size");
        let alignment = alignment.parse().expect("Failed to parse alignment");
        c_layouts.push((name.to_owned(), Layout { size, alignment }));
    }

    let mut results = Results::default();

    for ((rust_name, rust_layout), (c_name, c_layout)) in RUST_LAYOUTS.iter().zip(c_layouts.iter())
    {
        if rust_name != c_name {
            results.record_failed();
            eprintln!("Name mismatch:\nRust: {rust_name:?}\nC:    {c_name:?}");
            continue;
        }

        if rust_layout != c_layout {
            results.record_failed();
            eprintln!("Layout mismatch for {rust_name}\nRust: {rust_layout:?}\nC:    {c_layout:?}",);
            continue;
        }

        results.record_passed();
    }

    results.expect_total_success();
}

fn get_c_output(name: &str) -> Result<String, Box<dyn Error>> {
    let tmpdir = Builder::new().prefix("abi").tempdir()?;
    let exe = tmpdir.path().join(name);
    let c_file = Path::new("tests").join(name).with_extension("c");

    let cc = Compiler::new().expect("configured compiler");
    cc.compile(&c_file, &exe)?;

    let mut cmd = Command::new(exe);
    cmd.stderr(Stdio::inherit());
    let out = cmd.output()?;
    if !out.status.success() {
        let (status, stdout) = (out.status, String::from_utf8_lossy(&out.stdout));
        return Err(format!("command {cmd:?} failed, {status:?}\nstdout: {stdout}").into());
    }

    Ok(String::from_utf8(out.stdout)?)
}

const RUST_LAYOUTS: &[(&str, Layout)] = &[
    (
        "AdwAboutDialogClass",
        Layout {
            size: size_of::<AdwAboutDialogClass>(),
            alignment: align_of::<AdwAboutDialogClass>(),
        },
    ),
    (
        "AdwAboutWindowClass",
        Layout {
            size: size_of::<AdwAboutWindowClass>(),
            alignment: align_of::<AdwAboutWindowClass>(),
        },
    ),
    (
        "AdwActionRow",
        Layout {
            size: size_of::<AdwActionRow>(),
            alignment: align_of::<AdwActionRow>(),
        },
    ),
    (
        "AdwActionRowClass",
        Layout {
            size: size_of::<AdwActionRowClass>(),
            alignment: align_of::<AdwActionRowClass>(),
        },
    ),
    (
        "AdwAlertDialog",
        Layout {
            size: size_of::<AdwAlertDialog>(),
            alignment: align_of::<AdwAlertDialog>(),
        },
    ),
    (
        "AdwAlertDialogClass",
        Layout {
            size: size_of::<AdwAlertDialogClass>(),
            alignment: align_of::<AdwAlertDialogClass>(),
        },
    ),
    (
        "AdwAnimation",
        Layout {
            size: size_of::<AdwAnimation>(),
            alignment: align_of::<AdwAnimation>(),
        },
    ),
    (
        "AdwAnimationState",
        Layout {
            size: size_of::<AdwAnimationState>(),
            alignment: align_of::<AdwAnimationState>(),
        },
    ),
    (
        "AdwApplication",
        Layout {
            size: size_of::<AdwApplication>(),
            alignment: align_of::<AdwApplication>(),
        },
    ),
    (
        "AdwApplicationClass",
        Layout {
            size: size_of::<AdwApplicationClass>(),
            alignment: align_of::<AdwApplicationClass>(),
        },
    ),
    (
        "AdwApplicationWindow",
        Layout {
            size: size_of::<AdwApplicationWindow>(),
            alignment: align_of::<AdwApplicationWindow>(),
        },
    ),
    (
        "AdwApplicationWindowClass",
        Layout {
            size: size_of::<AdwApplicationWindowClass>(),
            alignment: align_of::<AdwApplicationWindowClass>(),
        },
    ),
    (
        "AdwAvatarClass",
        Layout {
            size: size_of::<AdwAvatarClass>(),
            alignment: align_of::<AdwAvatarClass>(),
        },
    ),
    (
        "AdwBannerClass",
        Layout {
            size: size_of::<AdwBannerClass>(),
            alignment: align_of::<AdwBannerClass>(),
        },
    ),
    (
        "AdwBin",
        Layout {
            size: size_of::<AdwBin>(),
            alignment: align_of::<AdwBin>(),
        },
    ),
    (
        "AdwBinClass",
        Layout {
            size: size_of::<AdwBinClass>(),
            alignment: align_of::<AdwBinClass>(),
        },
    ),
    (
        "AdwBreakpointBin",
        Layout {
            size: size_of::<AdwBreakpointBin>(),
            alignment: align_of::<AdwBreakpointBin>(),
        },
    ),
    (
        "AdwBreakpointBinClass",
        Layout {
            size: size_of::<AdwBreakpointBinClass>(),
            alignment: align_of::<AdwBreakpointBinClass>(),
        },
    ),
    (
        "AdwBreakpointClass",
        Layout {
            size: size_of::<AdwBreakpointClass>(),
            alignment: align_of::<AdwBreakpointClass>(),
        },
    ),
    (
        "AdwBreakpointConditionLengthType",
        Layout {
            size: size_of::<AdwBreakpointConditionLengthType>(),
            alignment: align_of::<AdwBreakpointConditionLengthType>(),
        },
    ),
    (
        "AdwBreakpointConditionRatioType",
        Layout {
            size: size_of::<AdwBreakpointConditionRatioType>(),
            alignment: align_of::<AdwBreakpointConditionRatioType>(),
        },
    ),
    (
        "AdwButtonContentClass",
        Layout {
            size: size_of::<AdwButtonContentClass>(),
            alignment: align_of::<AdwButtonContentClass>(),
        },
    ),
    (
        "AdwCarouselClass",
        Layout {
            size: size_of::<AdwCarouselClass>(),
            alignment: align_of::<AdwCarouselClass>(),
        },
    ),
    (
        "AdwCarouselIndicatorDotsClass",
        Layout {
            size: size_of::<AdwCarouselIndicatorDotsClass>(),
            alignment: align_of::<AdwCarouselIndicatorDotsClass>(),
        },
    ),
    (
        "AdwCarouselIndicatorLinesClass",
        Layout {
            size: size_of::<AdwCarouselIndicatorLinesClass>(),
            alignment: align_of::<AdwCarouselIndicatorLinesClass>(),
        },
    ),
    (
        "AdwCenteringPolicy",
        Layout {
            size: size_of::<AdwCenteringPolicy>(),
            alignment: align_of::<AdwCenteringPolicy>(),
        },
    ),
    (
        "AdwClampClass",
        Layout {
            size: size_of::<AdwClampClass>(),
            alignment: align_of::<AdwClampClass>(),
        },
    ),
    (
        "AdwClampLayoutClass",
        Layout {
            size: size_of::<AdwClampLayoutClass>(),
            alignment: align_of::<AdwClampLayoutClass>(),
        },
    ),
    (
        "AdwClampScrollableClass",
        Layout {
            size: size_of::<AdwClampScrollableClass>(),
            alignment: align_of::<AdwClampScrollableClass>(),
        },
    ),
    (
        "AdwColorScheme",
        Layout {
            size: size_of::<AdwColorScheme>(),
            alignment: align_of::<AdwColorScheme>(),
        },
    ),
    (
        "AdwComboRow",
        Layout {
            size: size_of::<AdwComboRow>(),
            alignment: align_of::<AdwComboRow>(),
        },
    ),
    (
        "AdwComboRowClass",
        Layout {
            size: size_of::<AdwComboRowClass>(),
            alignment: align_of::<AdwComboRowClass>(),
        },
    ),
    (
        "AdwDialog",
        Layout {
            size: size_of::<AdwDialog>(),
            alignment: align_of::<AdwDialog>(),
        },
    ),
    (
        "AdwDialogClass",
        Layout {
            size: size_of::<AdwDialogClass>(),
            alignment: align_of::<AdwDialogClass>(),
        },
    ),
    (
        "AdwDialogPresentationMode",
        Layout {
            size: size_of::<AdwDialogPresentationMode>(),
            alignment: align_of::<AdwDialogPresentationMode>(),
        },
    ),
    (
        "AdwEasing",
        Layout {
            size: size_of::<AdwEasing>(),
            alignment: align_of::<AdwEasing>(),
        },
    ),
    (
        "AdwEntryRow",
        Layout {
            size: size_of::<AdwEntryRow>(),
            alignment: align_of::<AdwEntryRow>(),
        },
    ),
    (
        "AdwEntryRowClass",
        Layout {
            size: size_of::<AdwEntryRowClass>(),
            alignment: align_of::<AdwEntryRowClass>(),
        },
    ),
    (
        "AdwEnumListItemClass",
        Layout {
            size: size_of::<AdwEnumListItemClass>(),
            alignment: align_of::<AdwEnumListItemClass>(),
        },
    ),
    (
        "AdwEnumListModelClass",
        Layout {
            size: size_of::<AdwEnumListModelClass>(),
            alignment: align_of::<AdwEnumListModelClass>(),
        },
    ),
    (
        "AdwExpanderRow",
        Layout {
            size: size_of::<AdwExpanderRow>(),
            alignment: align_of::<AdwExpanderRow>(),
        },
    ),
    (
        "AdwExpanderRowClass",
        Layout {
            size: size_of::<AdwExpanderRowClass>(),
            alignment: align_of::<AdwExpanderRowClass>(),
        },
    ),
    (
        "AdwFlapClass",
        Layout {
            size: size_of::<AdwFlapClass>(),
            alignment: align_of::<AdwFlapClass>(),
        },
    ),
    (
        "AdwFlapFoldPolicy",
        Layout {
            size: size_of::<AdwFlapFoldPolicy>(),
            alignment: align_of::<AdwFlapFoldPolicy>(),
        },
    ),
    (
        "AdwFlapTransitionType",
        Layout {
            size: size_of::<AdwFlapTransitionType>(),
            alignment: align_of::<AdwFlapTransitionType>(),
        },
    ),
    (
        "AdwFoldThresholdPolicy",
        Layout {
            size: size_of::<AdwFoldThresholdPolicy>(),
            alignment: align_of::<AdwFoldThresholdPolicy>(),
        },
    ),
    (
        "AdwHeaderBarClass",
        Layout {
            size: size_of::<AdwHeaderBarClass>(),
            alignment: align_of::<AdwHeaderBarClass>(),
        },
    ),
    (
        "AdwLeafletClass",
        Layout {
            size: size_of::<AdwLeafletClass>(),
            alignment: align_of::<AdwLeafletClass>(),
        },
    ),
    (
        "AdwLeafletPageClass",
        Layout {
            size: size_of::<AdwLeafletPageClass>(),
            alignment: align_of::<AdwLeafletPageClass>(),
        },
    ),
    (
        "AdwLeafletTransitionType",
        Layout {
            size: size_of::<AdwLeafletTransitionType>(),
            alignment: align_of::<AdwLeafletTransitionType>(),
        },
    ),
    (
        "AdwLengthUnit",
        Layout {
            size: size_of::<AdwLengthUnit>(),
            alignment: align_of::<AdwLengthUnit>(),
        },
    ),
    (
        "AdwMessageDialog",
        Layout {
            size: size_of::<AdwMessageDialog>(),
            alignment: align_of::<AdwMessageDialog>(),
        },
    ),
    (
        "AdwMessageDialogClass",
        Layout {
            size: size_of::<AdwMessageDialogClass>(),
            alignment: align_of::<AdwMessageDialogClass>(),
        },
    ),
    (
        "AdwNavigationDirection",
        Layout {
            size: size_of::<AdwNavigationDirection>(),
            alignment: align_of::<AdwNavigationDirection>(),
        },
    ),
    (
        "AdwNavigationPage",
        Layout {
            size: size_of::<AdwNavigationPage>(),
            alignment: align_of::<AdwNavigationPage>(),
        },
    ),
    (
        "AdwNavigationPageClass",
        Layout {
            size: size_of::<AdwNavigationPageClass>(),
            alignment: align_of::<AdwNavigationPageClass>(),
        },
    ),
    (
        "AdwNavigationSplitViewClass",
        Layout {
            size: size_of::<AdwNavigationSplitViewClass>(),
            alignment: align_of::<AdwNavigationSplitViewClass>(),
        },
    ),
    (
        "AdwNavigationViewClass",
        Layout {
            size: size_of::<AdwNavigationViewClass>(),
            alignment: align_of::<AdwNavigationViewClass>(),
        },
    ),
    (
        "AdwOverlaySplitViewClass",
        Layout {
            size: size_of::<AdwOverlaySplitViewClass>(),
            alignment: align_of::<AdwOverlaySplitViewClass>(),
        },
    ),
    (
        "AdwPasswordEntryRowClass",
        Layout {
            size: size_of::<AdwPasswordEntryRowClass>(),
            alignment: align_of::<AdwPasswordEntryRowClass>(),
        },
    ),
    (
        "AdwPreferencesDialog",
        Layout {
            size: size_of::<AdwPreferencesDialog>(),
            alignment: align_of::<AdwPreferencesDialog>(),
        },
    ),
    (
        "AdwPreferencesDialogClass",
        Layout {
            size: size_of::<AdwPreferencesDialogClass>(),
            alignment: align_of::<AdwPreferencesDialogClass>(),
        },
    ),
    (
        "AdwPreferencesGroup",
        Layout {
            size: size_of::<AdwPreferencesGroup>(),
            alignment: align_of::<AdwPreferencesGroup>(),
        },
    ),
    (
        "AdwPreferencesGroupClass",
        Layout {
            size: size_of::<AdwPreferencesGroupClass>(),
            alignment: align_of::<AdwPreferencesGroupClass>(),
        },
    ),
    (
        "AdwPreferencesPage",
        Layout {
            size: size_of::<AdwPreferencesPage>(),
            alignment: align_of::<AdwPreferencesPage>(),
        },
    ),
    (
        "AdwPreferencesPageClass",
        Layout {
            size: size_of::<AdwPreferencesPageClass>(),
            alignment: align_of::<AdwPreferencesPageClass>(),
        },
    ),
    (
        "AdwPreferencesRow",
        Layout {
            size: size_of::<AdwPreferencesRow>(),
            alignment: align_of::<AdwPreferencesRow>(),
        },
    ),
    (
        "AdwPreferencesRowClass",
        Layout {
            size: size_of::<AdwPreferencesRowClass>(),
            alignment: align_of::<AdwPreferencesRowClass>(),
        },
    ),
    (
        "AdwPreferencesWindow",
        Layout {
            size: size_of::<AdwPreferencesWindow>(),
            alignment: align_of::<AdwPreferencesWindow>(),
        },
    ),
    (
        "AdwPreferencesWindowClass",
        Layout {
            size: size_of::<AdwPreferencesWindowClass>(),
            alignment: align_of::<AdwPreferencesWindowClass>(),
        },
    ),
    (
        "AdwResponseAppearance",
        Layout {
            size: size_of::<AdwResponseAppearance>(),
            alignment: align_of::<AdwResponseAppearance>(),
        },
    ),
    (
        "AdwSpinRowClass",
        Layout {
            size: size_of::<AdwSpinRowClass>(),
            alignment: align_of::<AdwSpinRowClass>(),
        },
    ),
    (
        "AdwSplitButtonClass",
        Layout {
            size: size_of::<AdwSplitButtonClass>(),
            alignment: align_of::<AdwSplitButtonClass>(),
        },
    ),
    (
        "AdwSqueezerClass",
        Layout {
            size: size_of::<AdwSqueezerClass>(),
            alignment: align_of::<AdwSqueezerClass>(),
        },
    ),
    (
        "AdwSqueezerPageClass",
        Layout {
            size: size_of::<AdwSqueezerPageClass>(),
            alignment: align_of::<AdwSqueezerPageClass>(),
        },
    ),
    (
        "AdwSqueezerTransitionType",
        Layout {
            size: size_of::<AdwSqueezerTransitionType>(),
            alignment: align_of::<AdwSqueezerTransitionType>(),
        },
    ),
    (
        "AdwStatusPageClass",
        Layout {
            size: size_of::<AdwStatusPageClass>(),
            alignment: align_of::<AdwStatusPageClass>(),
        },
    ),
    (
        "AdwStyleManagerClass",
        Layout {
            size: size_of::<AdwStyleManagerClass>(),
            alignment: align_of::<AdwStyleManagerClass>(),
        },
    ),
    (
        "AdwSwipeTrackerClass",
        Layout {
            size: size_of::<AdwSwipeTrackerClass>(),
            alignment: align_of::<AdwSwipeTrackerClass>(),
        },
    ),
    (
        "AdwSwipeableInterface",
        Layout {
            size: size_of::<AdwSwipeableInterface>(),
            alignment: align_of::<AdwSwipeableInterface>(),
        },
    ),
    (
        "AdwSwitchRowClass",
        Layout {
            size: size_of::<AdwSwitchRowClass>(),
            alignment: align_of::<AdwSwitchRowClass>(),
        },
    ),
    (
        "AdwTabBarClass",
        Layout {
            size: size_of::<AdwTabBarClass>(),
            alignment: align_of::<AdwTabBarClass>(),
        },
    ),
    (
        "AdwTabButtonClass",
        Layout {
            size: size_of::<AdwTabButtonClass>(),
            alignment: align_of::<AdwTabButtonClass>(),
        },
    ),
    (
        "AdwTabOverviewClass",
        Layout {
            size: size_of::<AdwTabOverviewClass>(),
            alignment: align_of::<AdwTabOverviewClass>(),
        },
    ),
    (
        "AdwTabPageClass",
        Layout {
            size: size_of::<AdwTabPageClass>(),
            alignment: align_of::<AdwTabPageClass>(),
        },
    ),
    (
        "AdwTabViewClass",
        Layout {
            size: size_of::<AdwTabViewClass>(),
            alignment: align_of::<AdwTabViewClass>(),
        },
    ),
    (
        "AdwTabViewShortcuts",
        Layout {
            size: size_of::<AdwTabViewShortcuts>(),
            alignment: align_of::<AdwTabViewShortcuts>(),
        },
    ),
    (
        "AdwToastClass",
        Layout {
            size: size_of::<AdwToastClass>(),
            alignment: align_of::<AdwToastClass>(),
        },
    ),
    (
        "AdwToastOverlayClass",
        Layout {
            size: size_of::<AdwToastOverlayClass>(),
            alignment: align_of::<AdwToastOverlayClass>(),
        },
    ),
    (
        "AdwToastPriority",
        Layout {
            size: size_of::<AdwToastPriority>(),
            alignment: align_of::<AdwToastPriority>(),
        },
    ),
    (
        "AdwToolbarStyle",
        Layout {
            size: size_of::<AdwToolbarStyle>(),
            alignment: align_of::<AdwToolbarStyle>(),
        },
    ),
    (
        "AdwToolbarViewClass",
        Layout {
            size: size_of::<AdwToolbarViewClass>(),
            alignment: align_of::<AdwToolbarViewClass>(),
        },
    ),
    (
        "AdwViewStackClass",
        Layout {
            size: size_of::<AdwViewStackClass>(),
            alignment: align_of::<AdwViewStackClass>(),
        },
    ),
    (
        "AdwViewStackPageClass",
        Layout {
            size: size_of::<AdwViewStackPageClass>(),
            alignment: align_of::<AdwViewStackPageClass>(),
        },
    ),
    (
        "AdwViewStackPagesClass",
        Layout {
            size: size_of::<AdwViewStackPagesClass>(),
            alignment: align_of::<AdwViewStackPagesClass>(),
        },
    ),
    (
        "AdwViewSwitcherBarClass",
        Layout {
            size: size_of::<AdwViewSwitcherBarClass>(),
            alignment: align_of::<AdwViewSwitcherBarClass>(),
        },
    ),
    (
        "AdwViewSwitcherClass",
        Layout {
            size: size_of::<AdwViewSwitcherClass>(),
            alignment: align_of::<AdwViewSwitcherClass>(),
        },
    ),
    (
        "AdwViewSwitcherPolicy",
        Layout {
            size: size_of::<AdwViewSwitcherPolicy>(),
            alignment: align_of::<AdwViewSwitcherPolicy>(),
        },
    ),
    (
        "AdwViewSwitcherTitleClass",
        Layout {
            size: size_of::<AdwViewSwitcherTitleClass>(),
            alignment: align_of::<AdwViewSwitcherTitleClass>(),
        },
    ),
    (
        "AdwWindow",
        Layout {
            size: size_of::<AdwWindow>(),
            alignment: align_of::<AdwWindow>(),
        },
    ),
    (
        "AdwWindowClass",
        Layout {
            size: size_of::<AdwWindowClass>(),
            alignment: align_of::<AdwWindowClass>(),
        },
    ),
    (
        "AdwWindowTitleClass",
        Layout {
            size: size_of::<AdwWindowTitleClass>(),
            alignment: align_of::<AdwWindowTitleClass>(),
        },
    ),
];

const RUST_CONSTANTS: &[(&str, &str)] = &[
    ("(gint) ADW_ANIMATION_FINISHED", "3"),
    ("(gint) ADW_ANIMATION_IDLE", "0"),
    ("(gint) ADW_ANIMATION_PAUSED", "1"),
    ("(gint) ADW_ANIMATION_PLAYING", "2"),
    ("(gint) ADW_BREAKPOINT_CONDITION_MAX_ASPECT_RATIO", "1"),
    ("(gint) ADW_BREAKPOINT_CONDITION_MAX_HEIGHT", "3"),
    ("(gint) ADW_BREAKPOINT_CONDITION_MAX_WIDTH", "1"),
    ("(gint) ADW_BREAKPOINT_CONDITION_MIN_ASPECT_RATIO", "0"),
    ("(gint) ADW_BREAKPOINT_CONDITION_MIN_HEIGHT", "2"),
    ("(gint) ADW_BREAKPOINT_CONDITION_MIN_WIDTH", "0"),
    ("(gint) ADW_CENTERING_POLICY_LOOSE", "0"),
    ("(gint) ADW_CENTERING_POLICY_STRICT", "1"),
    ("(gint) ADW_COLOR_SCHEME_DEFAULT", "0"),
    ("(gint) ADW_COLOR_SCHEME_FORCE_DARK", "4"),
    ("(gint) ADW_COLOR_SCHEME_FORCE_LIGHT", "1"),
    ("(gint) ADW_COLOR_SCHEME_PREFER_DARK", "3"),
    ("(gint) ADW_COLOR_SCHEME_PREFER_LIGHT", "2"),
    ("(gint) ADW_DIALOG_AUTO", "0"),
    ("(gint) ADW_DIALOG_BOTTOM_SHEET", "2"),
    ("(gint) ADW_DIALOG_FLOATING", "1"),
    ("ADW_DURATION_INFINITE", "4294967295"),
    ("(gint) ADW_EASE_IN_BACK", "25"),
    ("(gint) ADW_EASE_IN_BOUNCE", "28"),
    ("(gint) ADW_EASE_IN_CIRC", "19"),
    ("(gint) ADW_EASE_IN_CUBIC", "4"),
    ("(gint) ADW_EASE_IN_ELASTIC", "22"),
    ("(gint) ADW_EASE_IN_EXPO", "16"),
    ("(gint) ADW_EASE_IN_OUT_BACK", "27"),
    ("(gint) ADW_EASE_IN_OUT_BOUNCE", "30"),
    ("(gint) ADW_EASE_IN_OUT_CIRC", "21"),
    ("(gint) ADW_EASE_IN_OUT_CUBIC", "6"),
    ("(gint) ADW_EASE_IN_OUT_ELASTIC", "24"),
    ("(gint) ADW_EASE_IN_OUT_EXPO", "18"),
    ("(gint) ADW_EASE_IN_OUT_QUAD", "3"),
    ("(gint) ADW_EASE_IN_OUT_QUART", "9"),
    ("(gint) ADW_EASE_IN_OUT_QUINT", "12"),
    ("(gint) ADW_EASE_IN_OUT_SINE", "15"),
    ("(gint) ADW_EASE_IN_QUAD", "1"),
    ("(gint) ADW_EASE_IN_QUART", "7"),
    ("(gint) ADW_EASE_IN_QUINT", "10"),
    ("(gint) ADW_EASE_IN_SINE", "13"),
    ("(gint) ADW_EASE_OUT_BACK", "26"),
    ("(gint) ADW_EASE_OUT_BOUNCE", "29"),
    ("(gint) ADW_EASE_OUT_CIRC", "20"),
    ("(gint) ADW_EASE_OUT_CUBIC", "5"),
    ("(gint) ADW_EASE_OUT_ELASTIC", "23"),
    ("(gint) ADW_EASE_OUT_EXPO", "17"),
    ("(gint) ADW_EASE_OUT_QUAD", "2"),
    ("(gint) ADW_EASE_OUT_QUART", "8"),
    ("(gint) ADW_EASE_OUT_QUINT", "11"),
    ("(gint) ADW_EASE_OUT_SINE", "14"),
    ("(gint) ADW_FLAP_FOLD_POLICY_ALWAYS", "1"),
    ("(gint) ADW_FLAP_FOLD_POLICY_AUTO", "2"),
    ("(gint) ADW_FLAP_FOLD_POLICY_NEVER", "0"),
    ("(gint) ADW_FLAP_TRANSITION_TYPE_OVER", "0"),
    ("(gint) ADW_FLAP_TRANSITION_TYPE_SLIDE", "2"),
    ("(gint) ADW_FLAP_TRANSITION_TYPE_UNDER", "1"),
    ("(gint) ADW_FOLD_THRESHOLD_POLICY_MINIMUM", "0"),
    ("(gint) ADW_FOLD_THRESHOLD_POLICY_NATURAL", "1"),
    ("(gint) ADW_LEAFLET_TRANSITION_TYPE_OVER", "0"),
    ("(gint) ADW_LEAFLET_TRANSITION_TYPE_SLIDE", "2"),
    ("(gint) ADW_LEAFLET_TRANSITION_TYPE_UNDER", "1"),
    ("(gint) ADW_LENGTH_UNIT_PT", "1"),
    ("(gint) ADW_LENGTH_UNIT_PX", "0"),
    ("(gint) ADW_LENGTH_UNIT_SP", "2"),
    ("(gint) ADW_LINEAR", "0"),
    ("(gint) ADW_NAVIGATION_DIRECTION_BACK", "0"),
    ("(gint) ADW_NAVIGATION_DIRECTION_FORWARD", "1"),
    ("(gint) ADW_RESPONSE_DEFAULT", "0"),
    ("(gint) ADW_RESPONSE_DESTRUCTIVE", "2"),
    ("(gint) ADW_RESPONSE_SUGGESTED", "1"),
    ("(gint) ADW_SQUEEZER_TRANSITION_TYPE_CROSSFADE", "1"),
    ("(gint) ADW_SQUEEZER_TRANSITION_TYPE_NONE", "0"),
    ("(guint) ADW_TAB_VIEW_SHORTCUT_ALL_SHORTCUTS", "4095"),
    ("(guint) ADW_TAB_VIEW_SHORTCUT_ALT_DIGITS", "1024"),
    ("(guint) ADW_TAB_VIEW_SHORTCUT_ALT_ZERO", "2048"),
    ("(guint) ADW_TAB_VIEW_SHORTCUT_CONTROL_END", "32"),
    ("(guint) ADW_TAB_VIEW_SHORTCUT_CONTROL_HOME", "16"),
    ("(guint) ADW_TAB_VIEW_SHORTCUT_CONTROL_PAGE_DOWN", "8"),
    ("(guint) ADW_TAB_VIEW_SHORTCUT_CONTROL_PAGE_UP", "4"),
    ("(guint) ADW_TAB_VIEW_SHORTCUT_CONTROL_SHIFT_END", "512"),
    ("(guint) ADW_TAB_VIEW_SHORTCUT_CONTROL_SHIFT_HOME", "256"),
    (
        "(guint) ADW_TAB_VIEW_SHORTCUT_CONTROL_SHIFT_PAGE_DOWN",
        "128",
    ),
    ("(guint) ADW_TAB_VIEW_SHORTCUT_CONTROL_SHIFT_PAGE_UP", "64"),
    ("(guint) ADW_TAB_VIEW_SHORTCUT_CONTROL_SHIFT_TAB", "2"),
    ("(guint) ADW_TAB_VIEW_SHORTCUT_CONTROL_TAB", "1"),
    ("(guint) ADW_TAB_VIEW_SHORTCUT_NONE", "0"),
    ("(gint) ADW_TOAST_PRIORITY_HIGH", "1"),
    ("(gint) ADW_TOAST_PRIORITY_NORMAL", "0"),
    ("(gint) ADW_TOOLBAR_FLAT", "0"),
    ("(gint) ADW_TOOLBAR_RAISED", "1"),
    ("(gint) ADW_TOOLBAR_RAISED_BORDER", "2"),
    ("(gint) ADW_VIEW_SWITCHER_POLICY_NARROW", "0"),
    ("(gint) ADW_VIEW_SWITCHER_POLICY_WIDE", "1"),
];
