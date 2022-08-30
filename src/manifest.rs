pub use std::collections::HashMap;

use serde_derive::{Deserialize, Serialize};

pub use crate::DeviceType;
use crate::registration::Platform;

/// The data in the manifest.json file describing a plugin
///
/// [Official Documentation](https://developer.elgato.com/documentation/stream-deck/sdk/manifest/)
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Manifest {
    /// Array of actions available in the plugin.
    pub actions: Vec<Action>,
    /// The author of the plugin.
    pub author: String,
    /// Category for listing the plugin.
    pub category: Option<String>,
    /// Relative path to a PNG without suffix to use with the actions list.
    pub category_icon: Option<String>,
    /// Relative path to the HTML or binary that makes up the plugin.
    pub code_path: String,
    /// Override for code_path for macOS.
    pub code_path_mac: Option<String>,
    /// Override for code_path for macOS.
    pub code_path_win: Option<String>,
    /// A general description of the plugin.
    pub description: String,
    /// Relative path to a PNG without suffix to be displayed in the Plugin Store.
    pub icon: String,
    /// Name of the plugin displayed in the Stream Deck store.
    pub name: String,
    /// Array of profiles, presented on installation. A way to create fullscreen plugins.
    pub profiles: Option<Vec<Profile>>,
    /// Relative path to the property inspector HTML for custom setting display.
    pub property_inspector_path: Option<String>,
    /// Default window size when a JavaScript plugin or Property Inspector opens a window.
    #[serde(default)]
    pub default_window_size: WindowSize,
    /// A site with more information about the plugin.
    #[serde(rename = "URL")]
    pub url: Option<String>,
    /// The plugin's version.
    pub version: String,
    /// Supported SDK version, current is 2 according to docs.
    #[serde(rename = "SDKVersion")]
    pub sdk_version: u32,
    /// List of operating system and versions supported by the plugin.
    #[serde(rename = "OS")]
    pub os: Vec<OS>,
    /// Version of the Stream Deck application required to install the plugin.
    pub software: Software,
    /// List of application identifiers to monitor (launched or terminated).
    ///
    /// This is a hash map where the key is either "mac" or "windows" and the key is a list of
    /// 1) bundle indifier for mac; or
    /// 2) exe name for windows
    pub applications_to_monitor: Option<HashMap<String, Vec<String>>>,
}

/// Describes an action available in this plugin.
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Action {
    /// Relative path to a PNG without the suffix. Not required for actions not
    /// visible in the actions list.
    pub icon: Option<String>,
    /// Name of the action, visible to the user in the actions list
    pub name: String,
    /// Overrides the PropertyInspectorPath from the plugin.
    pub property_inspector_path: Option<String>,
    /// Array of states. Each action can have one or many.
    pub states: Vec<State>,
    /// If false, prevents the action from being used in a Multi Action. Defaults to true.
    #[serde(default = "return_true")]
    pub supported_in_multi_actions: bool,
    /// String displayed as a tooltip when the user hovers over the actions list.
    pub tooltip: Option<String>,
    /// Unique identifier of the action. it must be a Uniform Type Identifier
    /// (UTI) with only lowercase alphanumeric characters, hyper or period. It
    /// must be in reverse-DNS format.
    #[serde(rename = "UUID")]
    pub uuid: String,
    /// Disable image caching.
    #[serde(default)]
    pub disable_caching: bool,
    /// Set to false it hides the action from the actions list.
    #[serde(default = "return_true")]
    pub visiable_in_actions_list: bool,
}

/// Describes a state that a button might be in.
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct State {
    /// Default image for the state.
    pub image: String,
    /// Provides a different image when used in a multi-action.
    pub multi_action_image: Option<String>,
    /// Displayed in a dropdown menu for a Multi-action.
    pub name: Option<String>,
    /// Default title.
    pub title: Option<String>,
    /// Whether to show the title.
    #[serde(default = "return_true")]
    pub show_title: bool,
    /// Default title color.
    pub title_color: Option<String>,
    /// Default title vertical alignment.
    pub title_alignment: Option<TitleAlignment>,
    /// Default font family for the title.
    pub font_family: Option<String>,
    /// Default font style for the title.
    pub font_style: Option<FontStyle>,
    /// Default font size for the title.
    pub font_size: Option<String>,
    /// True to set an underline under the title. False by default.
    #[serde(default)]
    pubfont_underline: bool,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Profile {
    /// The name of the profile.
    pub name: String,
    /// Type of device.
    pub device_type: DeviceType,
    /// Mark the profile as read-only.
    #[serde(default)]
    pub readonly: bool,
    #[serde(default)]
    /// Prevent Stream Deck from switching to the profile when installed.
    pub dont_auto_switch_when_installed: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct WindowSize(u32, u32);

impl Default for WindowSize {
    fn default() -> Self {
        WindowSize(500, 650)
    }
}

/// Describes an OS supported by the plugin
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct OS {
    /// Name of the platform, mac or windows
    pub platform: Platform,
    /// Minimum version of the operating system supported.
    pub minimum_version: String,
}

/// Describes the Stream Deck software required by the plugin.
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Software {
    /// Minimum version of the Stream Deck software supported by the plugin.
    pub minimum_version: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum TitleAlignment {
    #[serde(rename = "top")]
    Top,
    #[serde(rename = "bottom")]
    Bottom,
    #[serde(rename = "middle")]
    Middle,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum FontStyle {
    Regular,
    Bold,
    Italic,
    BoldItalic,
}

/// Returns false. This is for serde to get a default-true bool
fn return_true() -> bool {
    true
}
