use std::collections::HashMap;
use std::fmt;

use serde::de::{Deserializer, Visitor};
use serde::Deserialize;
use url::Url;

use super::files::File;
use super::{deserialize_empty_object, TargetPlatform};
use super::{Logo, Status, User};

/// See the [Mod Object](https://docs.mod.io/#mod-object) docs for more information.
#[derive(Debug, Deserialize)]
pub struct Mod {
    pub id: u32,
    pub game_id: u32,
    pub status: Status,
    pub visible: Visibility,
    pub submitted_by: User,
    pub date_added: u64,
    pub date_updated: u64,
    pub date_live: u64,
    pub maturity_option: MaturityOption,
    pub logo: Logo,
    pub homepage_url: Option<Url>,
    pub name: String,
    pub name_id: String,
    pub summary: String,
    pub description: Option<String>,
    pub description_plaintext: Option<String>,
    pub metadata_blob: Option<String>,
    pub profile_url: Url,
    #[serde(default, deserialize_with = "deserialize_empty_object")]
    pub modfile: Option<File>,
    pub media: Media,
    #[serde(rename = "metadata_kvp", deserialize_with = "deserialize_kvp")]
    pub metadata: MetadataMap,
    pub tags: Vec<Tag>,
    pub stats: Statistics,
    pub platforms: Vec<Platform>,
}

enum_number! {
    /// See [Status & Visibility](https://docs.mod.io/#status-amp-visibility) docs for more information.
    #[derive(Clone, Copy, Debug, Deserialize)]
    #[serde(from = "u8")]
    pub enum Visibility {
        Hidden = 0,
        Public = 1,
        _ => Unknown(u8),
    }
}

bitflags! {
    /// Maturity options a mod can be flagged.
    ///
    /// This is only relevant if the parent game allows mods to be labelled as mature.
    pub struct MaturityOption: u8 {
        const ALCOHOL   = 0b0001;
        const DRUGS     = 0b0010;
        const VIOLENCE  = 0b0100;
        const EXPLICIT  = 0b1000;
        const ALL = Self::ALCOHOL.bits | Self::DRUGS.bits | Self::VIOLENCE.bits | Self::EXPLICIT.bits;
    }
}

/// See the [Mod Event Object](https://docs.mod.io/#mod-event-object) docs for more information.
#[derive(Debug, Deserialize)]
pub struct Event {
    pub id: u32,
    pub mod_id: u32,
    pub user_id: u32,
    pub date_added: u64,
    pub event_type: EventType,
}

/// Type of mod event that was triggered.
#[derive(Debug, PartialEq, Eq, Hash)]
pub enum EventType {
    /// Primary file changed, the mod should be updated.
    ModfileChanged,
    /// Mod is marked as accepted and public.
    ModAvailable,
    /// Mod is marked as not accepted, deleted or hidden.
    ModUnavailable,
    /// Mod has been updated.
    ModEdited,
    /// Mod has been permanently deleted.
    ModDeleted,
    /// User has joined or left the mod team.
    ModTeamChanged,
    /// A comment has been published for a mod.
    ModCommentAdded,
    /// A comment has been deleted from a mod.
    ModCommentDeleted,
    /// New event types which are not supported yet.
    Unknown(String),
}

impl<'de> Deserialize<'de> for EventType {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct EventTypeVisitor;

        impl<'de> Visitor<'de> for EventTypeVisitor {
            type Value = EventType;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("mod event type string")
            }

            fn visit_str<E: serde::de::Error>(self, value: &str) -> Result<Self::Value, E> {
                match value {
                    "MODFILE_CHANGED" => Ok(Self::Value::ModfileChanged),
                    "MOD_AVAILABLE" => Ok(Self::Value::ModAvailable),
                    "MOD_UNAVAILABLE" => Ok(Self::Value::ModUnavailable),
                    "MOD_EDITED" => Ok(Self::Value::ModEdited),
                    "MOD_DELETED" => Ok(Self::Value::ModDeleted),
                    "MOD_TEAM_CHANGED" => Ok(Self::Value::ModTeamChanged),
                    "MOD_COMMENT_ADDED" => Ok(Self::Value::ModCommentAdded),
                    "MOD_COMMENT_DELETED" => Ok(Self::Value::ModCommentDeleted),
                    _ => Ok(Self::Value::Unknown(value.to_owned())),
                }
            }
        }

        deserializer.deserialize_str(EventTypeVisitor)
    }
}

impl fmt::Display for EventType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ModfileChanged => f.write_str("MODFILE_CHANGED"),
            Self::ModAvailable => f.write_str("MOD_AVAILABLE"),
            Self::ModUnavailable => f.write_str("MOD_UNAVAILABLE"),
            Self::ModEdited => f.write_str("MOD_EDITED"),
            Self::ModDeleted => f.write_str("MOD_DELETED"),
            Self::ModTeamChanged => f.write_str("MOD_TEAM_CHANGED"),
            Self::ModCommentAdded => f.write_str("MOD_COMMENT_ADDED"),
            Self::ModCommentDeleted => f.write_str("MOD_COMMENT_DELETED"),
            Self::Unknown(s) => f.write_str(s),
        }
    }
}

/// See the [Mod Dependency Object](https://docs.mod.io/#mod-dependencies-object) docs for more
/// information.
#[derive(Debug, Deserialize)]
pub struct Dependency {
    pub mod_id: u32,
    pub date_added: u64,
}

/// See the [Mod Media Object](https://docs.mod.io/#mod-media-object) docs for more
/// information.
#[derive(Debug, Deserialize)]
pub struct Media {
    #[serde(default = "Vec::new")]
    pub youtube: Vec<String>,
    #[serde(default = "Vec::new")]
    pub sketchfab: Vec<String>,
    #[serde(default = "Vec::new")]
    pub images: Vec<Image>,
}

/// See the [Image Object](https://docs.mod.io/#image-object) docs for more information.
#[derive(Deserialize)]
pub struct Image {
    pub filename: String,
    pub original: Url,
    pub thumb_320x180: Url,
}

impl fmt::Debug for Image {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Image")
            .field("filename", &self.filename)
            .field("original", &self.original.as_str())
            .field("thumb_320x180", &self.thumb_320x180.as_str())
            .finish()
    }
}

/// See the [Statistics Object](https://docs.mod.io/#mod-stats-object) docs for more
/// information.
#[derive(Debug, Deserialize)]
pub struct Statistics {
    pub mod_id: u32,
    pub downloads_total: u32,
    pub subscribers_total: u32,
    #[serde(flatten)]
    pub popularity: Popularity,
    #[serde(flatten)]
    pub ratings: Ratings,
    pub date_expires: u64,
}

/// Part of [`Statistics`]
#[derive(Debug, Deserialize)]
pub struct Popularity {
    #[serde(rename = "popularity_rank_position")]
    pub rank_position: u32,
    #[serde(rename = "popularity_rank_total_mods")]
    pub rank_total: u32,
}

/// Part of [`Statistics`]
#[derive(Debug, Deserialize)]
pub struct Ratings {
    #[serde(rename = "ratings_total")]
    pub total: u32,
    #[serde(rename = "ratings_positive")]
    pub positive: u32,
    #[serde(rename = "ratings_negative")]
    pub negative: u32,
    #[serde(rename = "ratings_percentage_positive")]
    pub percentage_positive: u32,
    #[serde(rename = "ratings_weighted_aggregate")]
    pub weighted_aggregate: f32,
    #[serde(rename = "ratings_display_text")]
    pub display_text: String,
}

/// See the [Rating Object](https://docs.mod.io/#rating-object) docs for more information.
#[derive(Debug)]
pub enum Rating {
    Positive {
        game_id: u32,
        mod_id: u32,
        date_added: u64,
    },
    Negative {
        game_id: u32,
        mod_id: u32,
        date_added: u64,
    },
}

impl<'de> Deserialize<'de> for Rating {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::Error;

        #[derive(Deserialize)]
        struct R {
            game_id: u32,
            mod_id: u32,
            rating: i8,
            date_added: u64,
        }

        match R::deserialize(deserializer) {
            Ok(R {
                game_id,
                mod_id,
                rating: 1,
                date_added,
            }) => Ok(Self::Positive {
                game_id,
                mod_id,
                date_added,
            }),
            Ok(R {
                game_id,
                mod_id,
                rating: -1,
                date_added,
            }) => Ok(Self::Negative {
                game_id,
                mod_id,
                date_added,
            }),
            Ok(R { rating, .. }) => Err(D::Error::custom(format!(
                "invalid rating value: {}",
                rating,
            ))),
            Err(e) => Err(e),
        }
    }
}

/// See the [Mod Platforms Object](https://docs.mod.io/#mod-platforms-object) docs for more information.
#[derive(Debug, Deserialize)]
pub struct Platform {
    #[serde(rename = "platform")]
    pub target: TargetPlatform,
    /// The unique id of the modfile that is currently live on the platform specified in the
    /// `target` field.
    #[serde(rename = "modfile_live")]
    pub modfile_id: u32,
}

/// See the [Mod Tag Object](https://docs.mod.io/#mod-tag-object) docs for more information.
#[derive(Debug, Deserialize)]
pub struct Tag {
    pub name: String,
    pub date_added: u64,
}

impl fmt::Display for Tag {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.name.fmt(fmt)
    }
}

/// See the [Metadata KVP Object](https://docs.mod.io/#metadata-kvp-object) docs for more
/// information.
#[derive(Debug, Clone, Default)]
pub struct MetadataMap(HashMap<String, Vec<String>>);

impl MetadataMap {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self(HashMap::with_capacity(capacity))
    }
}

impl std::ops::Deref for MetadataMap {
    type Target = HashMap<String, Vec<String>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for MetadataMap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// Deserialize a sequence of key-value objects to a `MetadataMap`.
///
/// Input
/// ```json
/// [
///     {"metakey": "pistol-dmg", "metavalue": "800"},
///     {"metakey": "smg-dmg", "metavalue": "1200"},
///     {"metakey": "pistol-dmg", "metavalue": "850"}
/// ]
/// ```
/// Result
/// ```json
/// {
///     "pistol-dmg": ["800", "850"],
///     "smg-dmg": ["1000"]
/// }
/// ```
fn deserialize_kvp<'de, D>(deserializer: D) -> Result<MetadataMap, D::Error>
where
    D: Deserializer<'de>,
{
    use serde::de::SeqAccess;

    struct MetadataVisitor;

    impl<'de> Visitor<'de> for MetadataVisitor {
        type Value = MetadataMap;

        fn expecting(&self, fmt: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
            fmt.write_str("metadata kvp")
        }

        fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
            #[derive(Deserialize)]
            struct KV {
                metakey: String,
                metavalue: String,
            }

            let mut map = seq
                .size_hint()
                .map_or_else(MetadataMap::new, MetadataMap::with_capacity);

            while let Some(elem) = seq.next_element::<KV>()? {
                map.entry(elem.metakey).or_default().push(elem.metavalue);
            }
            Ok(map)
        }
    }
    deserializer.deserialize_seq(MetadataVisitor)
}

/// See the [Comment Object](https://docs.mod.io/#comment-object) docs for more information.
#[derive(Debug, Deserialize)]
pub struct Comment {
    pub id: u32,
    pub resource_id: u32,
    pub user: User,
    pub date_added: u64,
    pub reply_id: u32,
    pub thread_position: String,
    pub karma: i32,
    pub content: String,
}

/// See the [Team Member Object](https://docs.mod.io/#team-member-object) docs for more
/// information.
#[derive(Debug, Deserialize)]
pub struct TeamMember {
    pub id: u32,
    pub user: User,
    pub level: TeamLevel,
    pub date_added: u64,
    pub position: String,
}

enum_number! {
    /// Defines the role of a team member.
    #[derive(Clone, Copy, Debug, Deserialize)]
    #[serde(from = "u8")]
    pub enum TeamLevel {
        Moderator = 1,
        Creator = 4,
        Admin = 8,
        _ => Unknown(u8),
    }
}

impl TeamLevel {
    pub fn value(self) -> u64 {
        u8::from(self).into()
    }
}

#[cfg(test)]
mod tests {
    use serde_test::{assert_de_tokens, Token};

    use super::EventType;

    #[test]
    fn mod_event_type_serde() {
        assert_de_tokens(&EventType::ModfileChanged, &[Token::Str("MODFILE_CHANGED")]);
        assert_de_tokens(&EventType::ModAvailable, &[Token::Str("MOD_AVAILABLE")]);
        assert_de_tokens(&EventType::ModUnavailable, &[Token::Str("MOD_UNAVAILABLE")]);
        assert_de_tokens(&EventType::ModEdited, &[Token::Str("MOD_EDITED")]);
        assert_de_tokens(&EventType::ModDeleted, &[Token::Str("MOD_DELETED")]);
        assert_de_tokens(
            &EventType::ModTeamChanged,
            &[Token::Str("MOD_TEAM_CHANGED")],
        );
        assert_de_tokens(
            &EventType::ModCommentAdded,
            &[Token::Str("MOD_COMMENT_ADDED")],
        );
        assert_de_tokens(
            &EventType::ModCommentDeleted,
            &[Token::Str("MOD_COMMENT_DELETED")],
        );
        assert_de_tokens(&EventType::Unknown("foo".to_owned()), &[Token::Str("foo")]);
    }
}
