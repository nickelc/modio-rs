use std::collections::HashMap;
use std::fmt;

use serde::{Deserialize, Deserializer};
use url::Url;

#[macro_use]
mod macros;

pub mod auth;
pub mod files;
pub mod games;
pub mod mods;

/// Used by `EventType` enums to catch unsupported event type variants
#[derive(Deserialize)]
#[serde(untagged, expecting = "enum type or unknown string")]
enum NonExhaustive<T> {
    Known(T),
    Unknown(String),
}

/// See the [Access Token Object](https://docs.mod.io/#access-token-object) docs for more
/// information.
#[derive(Deserialize)]
pub struct AccessToken {
    #[serde(rename = "access_token")]
    pub value: String,
    #[serde(rename = "date_expires")]
    pub expired_at: Option<u64>,
}

/// See the [Message Object](https://docs.mod.io/#message-object) docs for more information.
#[derive(Debug, Deserialize)]
pub struct Message {
    pub code: u16,
    pub message: String,
}

/// Result type for editing games, mods and files.
#[derive(Debug, Deserialize)]
#[serde(untagged, expecting = "edited object or 'no new data' message")]
pub enum Editing<T> {
    Entity(T),
    /// The request was successful however no new data was submitted.
    #[serde(deserialize_with = "deserialize_message")]
    NoChanges,
}

/// Result type for deleting game tag options, mod media, mod tags and mod dependencies.
#[derive(Debug, Deserialize)]
#[serde(untagged, expecting = "no content or 'no new data' message")]
pub enum Deletion {
    Success,
    /// The request was successful however no new data was submitted.
    #[serde(deserialize_with = "deserialize_message")]
    NoChanges,
}

fn deserialize_message<'de, D>(deserializer: D) -> Result<(), D::Error>
where
    D: serde::Deserializer<'de>,
{
    Message::deserialize(deserializer).map(|_| ())
}

/// See the [Multiple Item Response](https://docs.mod.io/#response-formats) docs for more
/// information.
#[derive(Debug, Deserialize)]
pub struct List<T> {
    pub data: Vec<T>,
    #[serde(rename = "result_count")]
    pub count: u32,
    #[serde(rename = "result_total")]
    pub total: u32,
    #[serde(rename = "result_limit")]
    pub limit: u32,
    #[serde(rename = "result_offset")]
    pub offset: u32,
}

/// See the [Error Object](https://docs.mod.io/#error-object) docs for more information.
#[derive(Debug, Deserialize)]
pub struct ErrorResponse {
    pub error: Error,
}

/// See the [Error Object](https://docs.mod.io/#error-object) docs for more information.
#[derive(Debug, Deserialize)]
pub struct Error {
    pub code: u16,
    pub error_ref: u16,
    pub message: String,
    pub errors: Option<HashMap<String, String>>,
}

/// See the [User Object](https://docs.mod.io/#user-object) docs for more information.
#[derive(Debug, Deserialize)]
pub struct User {
    pub id: u32,
    pub name_id: String,
    pub username: String,
    pub date_online: u32,
    #[serde(default, deserialize_with = "deserialize_empty_object")]
    pub avatar: Option<Avatar>,
    pub profile_url: Url,
}

/// See the [Avatar Object](https://docs.mod.io/#avatar-object) docs for more information.
#[derive(Debug, Deserialize)]
pub struct Avatar {
    pub filename: String,
    pub original: Url,
    pub thumb_50x50: Url,
    pub thumb_100x100: Url,
}

/// See the [Logo Object](https://docs.mod.io/#logo-object) docs for more information.
#[derive(Debug, Deserialize)]
pub struct Logo {
    pub filename: String,
    pub original: Url,
    pub thumb_320x180: Url,
    pub thumb_640x360: Url,
    pub thumb_1280x720: Url,
}

enum_number! {
    /// See [Status & Visibility](https://docs.mod.io/#status-amp-visibility) docs for more information.
    #[derive(Debug)]
    pub enum Status {
        NotAccepted = 0,
        Accepted = 1,
        Deleted = 3,
    }
}

/// See the [User Event Object](https://docs.mod.io/#user-event-object) docs for more information.
#[derive(Debug, Deserialize)]
pub struct Event {
    pub id: u32,
    pub game_id: u32,
    pub mod_id: u32,
    pub user_id: u32,
    pub date_added: u64,
    pub event_type: EventType,
}

/// Type of user event that was triggered.
#[derive(Debug, PartialEq, Eq, Hash, Deserialize)]
#[serde(from = "NonExhaustive<KnownEventType>")]
pub enum EventType {
    /// User has joined a team.
    UserTeamJoin,
    /// User has left a team.
    UserTeamLeave,
    /// User has subscribed to a mod.
    UserSubscribe,
    /// User has unsubscribed to a mod.
    UserUnsubscribe,
    /// New event types which are not supported yet.
    Other(String),
}

impl fmt::Display for EventType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EventType::UserTeamJoin => f.write_str("USER_TEAM_JOIN"),
            EventType::UserTeamLeave => f.write_str("USER_TEAM_LEAVE"),
            EventType::UserSubscribe => f.write_str("USER_SUBSCRIBE"),
            EventType::UserUnsubscribe => f.write_str("USER_UNSUBSCRIBE"),
            EventType::Other(s) => f.write_str(s),
        }
    }
}

// impl #[serde(from = "NonExhaustive<KnowEventType>")] {{{
#[derive(Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[allow(clippy::enum_variant_names)]
enum KnownEventType {
    /// User has joined a team.
    UserTeamJoin,
    /// User has left a team.
    UserTeamLeave,
    /// User has subscribed to a mod.
    UserSubscribe,
    /// User has unsubscribed to a mod.
    UserUnsubscribe,
}

impl From<NonExhaustive<KnownEventType>> for EventType {
    fn from(kind: NonExhaustive<KnownEventType>) -> EventType {
        use KnownEventType::*;
        use NonExhaustive::*;

        match kind {
            Known(UserTeamJoin) => EventType::UserTeamJoin,
            Known(UserTeamLeave) => EventType::UserTeamLeave,
            Known(UserSubscribe) => EventType::UserSubscribe,
            Known(UserUnsubscribe) => EventType::UserUnsubscribe,
            Unknown(other) => EventType::Other(other),
        }
    }
}
// }}}

/// Deserialize empty objects for optional properties as `None`.
///
/// The mod.io api returns `"field": {}` for some optional properties instead of returning
/// `"field": null` or omitting the field.
///
/// This function supports the following JSON examples as `None`.
/// ```json
/// {"id": 1, "field": {}}
/// {"id": 1, "field": null}
///
/// // And missing fields with `#[serde(default)]`
/// {"id": 1}
/// ```
pub fn deserialize_empty_object<'de, D, T>(deserializer: D) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de>,
{
    #[derive(Deserialize)]
    #[serde(
        untagged,
        deny_unknown_fields,
        expecting = "object, empty object or null"
    )]
    enum Helper<T> {
        Data(T),
        Empty {},
        Null,
    }
    match Helper::deserialize(deserializer) {
        Ok(Helper::Data(data)) => Ok(Some(data)),
        Ok(_) => Ok(None),
        Err(e) => Err(e),
    }
}

#[cfg(test)]
mod tests {
    use serde::Deserialize;

    #[test]
    fn deserialize_empty_object() {
        #[derive(Deserialize, Debug, PartialEq)]
        struct Game {
            id: u32,
            #[serde(default, deserialize_with = "super::deserialize_empty_object")]
            header: Option<Header>,
        }

        #[derive(Deserialize, Debug, PartialEq)]
        struct Header {
            filename: String,
        }

        let s = r#"{"id":1,"header":{"filename":"foobar"}}"#;
        let value = serde_json::from_str::<Game>(s).unwrap();
        let expected = Game {
            id: 1,
            header: Some(Header {
                filename: "foobar".to_string(),
            }),
        };
        assert_eq!(value, expected);

        let s = r#"{"id":1,"header":{}}"#;
        let value = serde_json::from_str::<Game>(s).unwrap();
        let expected = Game {
            id: 1,
            header: None,
        };
        assert_eq!(value, expected);

        let s = r#"{"id":1,"header":null}"#;
        let value = serde_json::from_str::<Game>(s).unwrap();
        let expected = Game {
            id: 1,
            header: None,
        };
        assert_eq!(value, expected);

        let s = r#"{"id":1}"#;
        let value = serde_json::from_str::<Game>(s).unwrap();
        let expected = Game {
            id: 1,
            header: None,
        };
        assert_eq!(value, expected);

        let s = r#"{"id":1,"header":{"filename":"foobar","id":1}}"#;
        let value = serde_json::from_str::<Game>(s).unwrap();
        let expected = Game {
            id: 1,
            header: Some(Header {
                filename: "foobar".to_string(),
            }),
        };
        assert_eq!(value, expected);

        let s = r#"{"id":1,"header":{"id":1}}"#;
        let value = serde_json::from_str::<Game>(s).unwrap_err();
        let expected = "object, empty object or null at line 1 column 26";
        assert_eq!(format!("{}", value), expected);
    }

    #[test]
    fn unknown_user_event_type() {
        use super::EventType;

        #[derive(Deserialize)]
        struct Event {
            kind: EventType,
        }
        let s = r#"{"kind": "UNKNOWN"}"#;
        let value = serde_json::from_str::<Event>(s).unwrap();
        let expected = EventType::Other(String::from("UNKNOWN"));
        assert_eq!(value.kind, expected);
    }

    #[test]
    fn unknown_mod_event_type() {
        use super::mods::EventType;

        #[derive(Deserialize)]
        struct Event {
            kind: EventType,
        }
        let s = r#"{"kind": "UNKNOWN"}"#;
        let value = serde_json::from_str::<Event>(s).unwrap();
        let expected = EventType::Other(String::from("UNKNOWN"));
        assert_eq!(value.kind, expected);
    }
}

// vim: fdm=marker
