//! Types related to the property inspector

use super::{GlobalSettingsPayload, KeyPayload, LogMessagePayload, UrlPayload};

use serde_derive::{Deserialize, Serialize};

/// A message received from the Stream Deck software.
///
/// - `G` represents the global settings that are persisted within the Stream Deck software.
/// - `S` represents the settings that are persisted within the Stream Deck software.
/// - `M` represents the messages that are received from the property inspector.
///
/// [Official Documentation](https://developer.elgato.com/documentation/stream-deck/sdk/events-received/)
#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "event", rename_all = "camelCase")]
pub enum Message<G, S, M> {
    /// The application has sent settings for an action.
    ///
    /// This message is sent in response to GetSettings, but also after the
    /// plugin changes the settings.
    ///
    /// [Official Documentation](https://developer.elgato.com/documentation/stream-deck/sdk/events-received/#didreceivesettings)
    #[serde(rename_all = "camelCase")]
    DidReceiveSettings {
        /// The uuid of the action.
        action: String,
        /// Value received during registration
        context: String,
        /// The device where the action exists.
        device: String,
        /// The current settings for the action.
        payload: KeyPayload<S>,
    },
    /// The application has sent settings for an action.
    ///
    /// This message is sent in response to GetGlobalSettings, but also after the
    /// plugin changes the settings.
    ///
    /// [Official Documentation](https://developer.elgato.com/documentation/stream-deck/sdk/events-received/#didreceiveglobalsettings)
    #[serde(rename_all = "camelCase")]
    DidReceiveGlobalSettings {
        /// The current settings for the action.
        payload: GlobalSettingsPayload<G>,
    },
    /// The plugin has sent some data
    ///
    /// [Official Documentation](https://developer.elgato.com/documentation/stream-deck/sdk/events-received/#sendtopropertyinspector)
    #[serde(rename_all = "camelCase")]
    SendToPropertyInspector {
        /// The uuid of the action
        action: String,
        /// Value received during registration
        context: String,
        /// Message sent by the plugin
        payload: M,
    },
}

/// A message to be sent to the Stream Deck software.
///
/// - `G` represents the global settings that are persisted within the Stream Deck software.
/// - `S` represents the action settings that are persisted within the Stream Deck software.
/// - `M` represents the messages that are sent to the plugin.
///
/// [Official Documentation](https://developer.elgato.com/documentation/stream-deck/sdk/events-sent/)
#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "event", rename_all = "camelCase")]
pub enum MessageOut<G, S, M> {
    #[serde(rename_all = "camelCase")]
    GetSettings {
        /// Value received during registration
        context: String,
    },
    /// Store settings for an instance of an action.
    ///
    /// [Official Documentation](https://developer.elgato.com/documentation/stream-deck/sdk/events-sent/#setsettings)
    #[serde(rename_all = "camelCase")]
    SetSettings {
        /// Value received during registration
        context: String,
        /// The settings to save.
        payload: S,
    },
    /// Open a URL in the default browser.
    ///
    /// [Official Documentation](https://developer.elgato.com/documentation/stream-deck/sdk/events-sent/#openurl)
    #[serde(rename_all = "camelCase")]
    OpenUrl {
        /// The url to open.
        payload: UrlPayload,
    },
    /// Retrieve plugin settings for via DidReceiveGlobalSettings.
    ///
    /// [Official Documentation](https://developer.elgato.com/documentation/stream-deck/sdk/events-sent/#getglobalsettings)
    #[serde(rename_all = "camelCase")]
    GetGlobalSettings {
        /// Value received during registration
        context: String,
    },
    /// Store plugin settings.
    ///
    /// [Official Documentation](https://developer.elgato.com/documentation/stream-deck/sdk/events-sent/#setglobalsettings)
    #[serde(rename_all = "camelCase")]
    SetGlobalSettings {
        /// Value received during registration
        context: String,
        /// The settings to save.
        payload: G,
    },
    /// Write to the log.
    ///
    /// [Official Documentation](https://developer.elgato.com/documentation/stream-deck/sdk/events-sent/#logmessage)
    #[serde(rename_all = "camelCase")]
    LogMessage {
        /// The message to log.
        payload: LogMessagePayload,
    },
    /// Send data to the plugin
    ///
    /// [Official Documentation](https://developer.elgato.com/documentation/stream-deck/sdk/events-sent/#sendtoplugin)
    #[serde(rename_all = "camelCase")]
    SendToPlugin {
        /// The uuid of the action
        action: String,
        /// Value received during registration
        context: String,
        /// Data to send
        payload: M,
    },
}
