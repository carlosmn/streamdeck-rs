//! Types for the case where you want to execute a plugin rather than write a plugin.

use serde_derive::{Deserialize, Serialize};
use serde_json::Value;

/// A message sent to the plugin
///
/// Contrary to the top-level [`Message`] we can't specify the types for the
/// settings or message from the property inspector as those are different on
/// each plugin. The best we can do is leave them as JSON values.
///
/// [Official Documentation](https://developer.elgato.com/documentation/stream-deck/sdk/events-received/)
pub type Message = crate::Message<Value, Value, Value>;

/// A message received from the plugin
///
/// Contrary to the top-level [`MessageOut`] we can't specify the types for the
/// settings or message from the property inspector as those are different on
/// each plugin. The best we can do is leave them as JSON values.
///
/// [Official Documentation](https://developer.elgato.com/documentation/stream-deck/sdk/events-sent/)
pub type MessageIn = crate::MessageOut<Value, Value, Value>;

/// The message sent from the plugin during registration
#[derive(Debug, Deserialize, Serialize)]
pub struct Registration {
    /// Event as was passed in to the plugin during startup
    pub event: String,
    /// UUID as passed in to the plugin during startup
    pub uuid: String,
}

/// A message sent to the property inspector
///
/// Contrary to the "regular" [`crate::property_inspector::Message`] we can't
/// specify the types for the settings or message from the property inspector as
/// those are different on each plugin. The best we can do is leave them as JSON
/// values.
///
/// [Official Documentation](https://developer.elgato.com/documentation/stream-deck/sdk/events-received/)
pub type PropertyInspectorMessage = crate::property_inspector::Message<Value, Value, Value>;

/// A message received from the property inspector
///
/// Contrary to the "regular" [`crate::property_inspector::Message`] we can't
/// specify the types for the settings or message from the property inspector as
/// those are different on each plugin. The best we can do is leave them as JSON
/// values.
///
/// [Official Documentation](https://developer.elgato.com/documentation/stream-deck/sdk/events-sent/)
pub type PropertyInspectorMessageIn = crate::property_inspector::MessageOut<Value, Value, Value>;
