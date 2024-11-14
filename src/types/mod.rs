//! Model types defining the mod.io API.

use std::fmt;

use serde::de::{Deserialize, Deserializer};
use serde_derive::{Deserialize, Serialize};
use url::Url;

#[macro_use]
mod macros;
mod utils;

pub mod auth;
pub mod files;
pub mod games;
pub mod id;
pub mod mods;

use utils::{DeserializeField, MissingField};

use self::id::{EventId, GameId, ModId, UserId};

/// See the [Message Object](https://docs.mod.io/#message-object) docs for more information.
#[derive(Debug, Deserialize)]
#[non_exhaustive]
pub struct Message {
    pub code: u16,
    pub message: String,
}

/// Result type for editing games, mods and files.
#[derive(Debug, Deserialize)]
#[serde(untagged, expecting = "edited object or 'no new data' message")]
#[non_exhaustive]
pub enum Editing<T> {
    Entity(T),
    /// The request was successful however no new data was submitted.
    #[serde(deserialize_with = "deserialize_message")]
    NoChanges,
}

/// Result type for deleting game tag options, mod media, mod tags and mod dependencies.
#[derive(Debug, Deserialize)]
#[serde(untagged, expecting = "no content or 'no new data' message")]
#[non_exhaustive]
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
#[derive(Debug, PartialEq, Deserialize)]
#[non_exhaustive]
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
#[non_exhaustive]
pub struct ErrorResponse {
    pub error: Error,
}

/// See the [Error Object](https://docs.mod.io/#error-object) docs for more information.
#[derive(Debug, PartialEq, Deserialize)]
#[non_exhaustive]
pub struct Error {
    pub code: u16,
    pub error_ref: u16,
    pub message: String,
    #[serde(default, deserialize_with = "deserialize_errors")]
    pub errors: Vec<(String, String)>,
}

fn deserialize_errors<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<Vec<(String, String)>, D::Error> {
    use serde::de::{MapAccess, Visitor};

    struct MapVisitor;
    impl<'de> Visitor<'de> for MapVisitor {
        type Value = Vec<(String, String)>;

        fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            formatter.write_str("errors map")
        }

        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            let mut errors = map.size_hint().map_or_else(Vec::new, Vec::with_capacity);
            while let Some(entry) = map.next_entry()? {
                errors.push(entry);
            }
            Ok(errors)
        }
    }

    deserializer.deserialize_map(MapVisitor)
}

/// See the [User Object](https://docs.mod.io/#user-object) docs for more information.
#[derive(Deserialize)]
#[non_exhaustive]
pub struct User {
    pub id: UserId,
    pub name_id: String,
    pub username: String,
    pub date_online: Timestamp,
    #[serde(default, deserialize_with = "deserialize_empty_object")]
    pub avatar: Option<Avatar>,
    pub profile_url: Url,
}

impl fmt::Debug for User {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("User")
            .field("id", &self.id)
            .field("name_id", &self.name_id)
            .field("username", &self.username)
            .field("date_online", &self.date_online)
            .field("avatar", &self.avatar)
            .field("profile_url", &self.profile_url.as_str())
            .finish()
    }
}

/// See the [Avatar Object](https://docs.mod.io/#avatar-object) docs for more information.
#[derive(Deserialize)]
#[non_exhaustive]
pub struct Avatar {
    pub filename: String,
    pub original: Url,
    pub thumb_50x50: Url,
    pub thumb_100x100: Url,
}

impl fmt::Debug for Avatar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Avatar")
            .field("filename", &self.filename)
            .field("thumb_50x50", &self.thumb_50x50.as_str())
            .field("thumb_100x100", &self.thumb_100x100.as_str())
            .finish()
    }
}

/// See the [Logo Object](https://docs.mod.io/#logo-object) docs for more information.
#[derive(Deserialize)]
#[non_exhaustive]
pub struct Logo {
    pub filename: String,
    pub original: Url,
    pub thumb_320x180: Url,
    pub thumb_640x360: Url,
    pub thumb_1280x720: Url,
}

impl fmt::Debug for Logo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Logo")
            .field("filename", &self.filename)
            .field("original", &self.original.as_str())
            .field("thumb_320x180", &self.thumb_320x180.as_str())
            .field("thumb_640x360", &self.thumb_640x360.as_str())
            .field("thumb_1280x720", &self.thumb_1280x720.as_str())
            .finish()
    }
}

newtype_enum! {
    /// See [Status & Visibility](https://docs.mod.io/#status-amp-visibility) docs for more information.
    pub struct Status: u8 {
        const NOT_ACCEPTED = 0;
        const ACCEPTED     = 1;
        const DELETED      = 3;
    }

    /// See the [mod.io docs](https://docs.mod.io/#targeting-a-platform) for more information.
    #[derive(Deserialize, Serialize)]
    pub struct TargetPlatform<16> {
        const ANDROID       = b"android";
        const IOS           = b"ios";
        const LINUX         = b"linux";
        const MAC           = b"mac";
        const WINDOWS       = b"windows";
        const PS4           = b"ps4";
        const PS5           = b"ps5";
        const SOURCE        = b"source";
        const SWITCH        = b"switch";
        const XBOX_ONE      = b"xboxone";
        const XBOX_SERIES_X = b"xboxseriesx";
        const OCULUS        = b"oculus";
    }

    /// See the [mod.io docs](https://docs.mod.io/#targeting-a-portal) for more information.
    pub struct TargetPortal<12> {
        const STEAM     = b"steam";
        const GOG       = b"gog";
        const EGS       = b"egs";
        const ITCHIO    = b"itchio";
        const NINTENDO  = b"nintendo";
        const PSN       = b"psn";
        const XBOX_LIVE = b"xboxlive";
        const APPLE     = b"apple";
        const GOOGLE    = b"google";
        const FACEBOOK  = b"facebook";
        const DISCORD   = b"discord";
    }
}

impl TargetPlatform {
    pub fn display_name(&self) -> &str {
        match *self {
            Self::ANDROID => "Android",
            Self::IOS => "iOS",
            Self::LINUX => "Linux",
            Self::MAC => "Mac",
            Self::WINDOWS => "Windows",
            Self::PS4 => "PlayStation 4",
            Self::PS5 => "PlayStation 5",
            Self::SOURCE => "Source",
            Self::SWITCH => "Nintendo Switch",
            Self::XBOX_ONE => "Xbox One",
            Self::XBOX_SERIES_X => "Xbox Series X/S",
            Self::OCULUS => "Oculus",
            _ => self.0.as_str(),
        }
    }
}

impl fmt::Display for TargetPlatform {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.display_name())
    }
}

/// See the [User Event Object](https://docs.mod.io/#user-event-object) docs for more information.
#[derive(Debug, Deserialize)]
#[non_exhaustive]
pub struct Event {
    pub id: EventId,
    pub game_id: GameId,
    pub mod_id: ModId,
    pub user_id: UserId,
    pub date_added: Timestamp,
    pub event_type: EventType,
}

newtype_enum! {
    /// Type of user event that was triggered.
    #[derive(Deserialize)]
    #[serde(transparent)]
    pub struct EventType<24> {
        /// User has joined a team.
        const USER_TEAM_JOIN   = b"USER_TEAM_JOIN";
        /// User has left a team.
        const USER_TEAM_LEAVE  = b"USER_TEAM_LEAVE";
        /// User has subscribed to a mod.
        const USER_SUBSCRIBE   = b"USER_SUBSCRIBE";
        /// User has unsubscribed to a mod.
        const USER_UNSUBSCRIBE = b"USER_UNSUBSCRIBE";
    }
}

impl fmt::Display for EventType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

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
fn deserialize_empty_object<'de, D, T>(deserializer: D) -> Result<Option<T>, D::Error>
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

/// Repesentation of a Unix timestamp.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct Timestamp(i64);

impl Timestamp {
    /// Get the Unix timestamp in seconds.
    pub const fn as_secs(self) -> i64 {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use serde_derive::Deserialize;
    use serde_test::{assert_de_tokens, assert_tokens, Token};

    use super::{deserialize_empty_object, Error, EventType, TargetPlatform};

    #[test]
    fn deserialize_error_no_errors_field() {
        let value = Error {
            code: 404,
            error_ref: 11005,
            message: "foo".to_owned(),
            errors: vec![],
        };

        assert_de_tokens(
            &value,
            &[
                Token::Struct {
                    name: "Error",
                    len: 3,
                },
                Token::Str("code"),
                Token::U16(404),
                Token::Str("error_ref"),
                Token::U16(11005),
                Token::Str("message"),
                Token::Str("foo"),
                Token::StructEnd,
            ],
        );
    }

    #[test]
    fn deserialize_error_empty_errors() {
        let value = Error {
            code: 404,
            error_ref: 11005,
            message: "foo".to_owned(),
            errors: vec![],
        };

        assert_de_tokens(
            &value,
            &[
                Token::Struct {
                    name: "Error",
                    len: 3,
                },
                Token::Str("code"),
                Token::U16(404),
                Token::Str("error_ref"),
                Token::U16(11005),
                Token::Str("message"),
                Token::Str("foo"),
                Token::Str("errors"),
                Token::Map { len: Some(0) },
                Token::MapEnd,
                Token::StructEnd,
            ],
        );
    }

    #[test]
    fn deserialize_error_with_errors() {
        let value = Error {
            code: 404,
            error_ref: 11005,
            message: "foo".to_owned(),
            errors: vec![("foo".to_owned(), "bar".to_owned())],
        };

        assert_de_tokens(
            &value,
            &[
                Token::Struct {
                    name: "Error",
                    len: 3,
                },
                Token::Str("code"),
                Token::U16(404),
                Token::Str("error_ref"),
                Token::U16(11005),
                Token::Str("message"),
                Token::Str("foo"),
                Token::Str("errors"),
                Token::Map { len: Some(1) },
                Token::Str("foo"),
                Token::Str("bar"),
                Token::MapEnd,
                Token::StructEnd,
            ],
        );
    }

    #[derive(Debug, PartialEq, Deserialize)]
    struct Game {
        id: u32,
        #[serde(default, deserialize_with = "deserialize_empty_object")]
        header: Option<Header>,
    }

    #[derive(Debug, PartialEq, Deserialize)]
    struct Header {
        filename: String,
    }

    #[test]
    fn deserialize_empty_object_full() {
        let value = Game {
            id: 1,
            header: Some(Header {
                filename: "foobar".to_string(),
            }),
        };
        assert_de_tokens(
            &value,
            &[
                Token::Struct {
                    name: "Game",
                    len: 2,
                },
                Token::Str("id"),
                Token::U8(1),
                Token::Str("header"),
                Token::Struct {
                    name: "Header",
                    len: 1,
                },
                Token::Str("filename"),
                Token::Str("foobar"),
                Token::StructEnd,
                Token::StructEnd,
            ],
        );
    }

    #[test]
    fn deserialize_empty_object_empty() {
        let value = Game {
            id: 1,
            header: None,
        };
        assert_de_tokens(
            &value,
            &[
                Token::Struct {
                    name: "Game",
                    len: 2,
                },
                Token::Str("id"),
                Token::U8(1),
                Token::Str("header"),
                Token::Struct {
                    name: "Header",
                    len: 0,
                },
                Token::StructEnd,
                Token::StructEnd,
            ],
        );
    }

    #[test]
    fn deserialize_empty_object_null() {
        let value = Game {
            id: 1,
            header: None,
        };
        assert_de_tokens(
            &value,
            &[
                Token::Struct {
                    name: "Game",
                    len: 2,
                },
                Token::Str("id"),
                Token::U8(1),
                Token::Str("header"),
                Token::None,
                Token::StructEnd,
            ],
        );
    }

    #[test]
    fn deserialize_empty_object_absent() {
        let value = Game {
            id: 1,
            header: None,
        };
        assert_de_tokens(
            &value,
            &[
                Token::Struct {
                    name: "Game",
                    len: 1,
                },
                Token::Str("id"),
                Token::U8(1),
                Token::StructEnd,
            ],
        );
    }

    #[test]
    fn deserialize_empty_object_unknown_values() {
        let value = Game {
            id: 1,
            header: Some(Header {
                filename: "foobar".to_string(),
            }),
        };
        assert_de_tokens(
            &value,
            &[
                Token::Struct {
                    name: "Game",
                    len: 2,
                },
                Token::Str("id"),
                Token::U8(1),
                Token::Str("header"),
                Token::Struct {
                    name: "Header",
                    len: 1,
                },
                Token::Str("filename"),
                Token::Str("foobar"),
                Token::Str("id"),
                Token::U8(2),
                Token::StructEnd,
                Token::StructEnd,
            ],
        );
    }

    #[test]
    fn deserialize_empty_object_missing_field() {
        serde_test::assert_de_tokens_error::<Game>(
            &[
                Token::Struct {
                    name: "Game",
                    len: 2,
                },
                Token::Str("id"),
                Token::U8(1),
                Token::Str("header"),
                Token::Struct {
                    name: "Header",
                    len: 1,
                },
                Token::Str("id"),
                Token::U8(2),
                Token::StructEnd,
                Token::StructEnd,
            ],
            "object, empty object or null",
        );
    }

    #[test]
    fn user_event_type_serde() {
        assert_de_tokens(&EventType::USER_TEAM_JOIN, &[Token::Str("USER_TEAM_JOIN")]);
        assert_de_tokens(
            &EventType::USER_TEAM_LEAVE,
            &[Token::Str("USER_TEAM_LEAVE")],
        );
        assert_de_tokens(&EventType::USER_SUBSCRIBE, &[Token::Str("USER_SUBSCRIBE")]);
        assert_de_tokens(
            &EventType::USER_UNSUBSCRIBE,
            &[Token::Str("USER_UNSUBSCRIBE")],
        );
        assert_de_tokens(&EventType::from_bytes(b"foo"), &[Token::Str("foo")]);
    }

    #[test]
    fn target_platform_compare() {
        assert_eq!(TargetPlatform::ANDROID, "ANDROID");
        assert_eq!("android", TargetPlatform::ANDROID);
    }

    #[test]
    fn target_platform_serde() {
        assert_tokens(
            &TargetPlatform::ANDROID,
            &[
                Token::NewtypeStruct {
                    name: "TargetPlatform",
                },
                Token::Str("android"),
            ],
        );
    }
}

// vim: fdm=marker
