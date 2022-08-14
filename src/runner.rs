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
