use std::collections::HashMap;
use std::fmt;
use std::ops::Index;

use serde::de::{Deserialize, Deserializer};
use url::Url;
use url_serde;

/// See the [Message Object](https://docs.mod.io/#message-object) docs for more informations.
#[derive(Debug, Deserialize)]
pub struct ModioMessage {
    pub code: u16,
    pub message: String,
}

/// See the [Multiple Item Response](https://docs.mod.io/#response-formats) docs for more
/// informations.
#[derive(Debug, Deserialize)]
pub struct ModioListResponse<T> {
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

impl<T> Index<usize> for ModioListResponse<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<T> IntoIterator for ModioListResponse<T> {
    type Item = T;
    type IntoIter = ::std::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

impl<'a, T> IntoIterator for &'a ModioListResponse<T> {
    type Item = &'a T;
    type IntoIter = ::std::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.iter()
    }
}

/// See the [Error Object](https://docs.mod.io/#error-object) docs for more informations.
#[derive(Debug, Deserialize)]
pub struct ModioErrorResponse {
    #[serde(rename = "error")]
    pub error: ClientError,
}

/// See the [Error Object](https://docs.mod.io/#error-object) docs for more informations.
#[derive(Debug, Deserialize)]
pub struct ClientError {
    pub code: u16,
    pub message: String,
    pub errors: Option<HashMap<String, String>>,
}

/// See the [User Object](https://docs.mod.io/#user-object) docs for more informations.
#[derive(Debug, Deserialize)]
pub struct User {
    pub id: u32,
    pub name_id: String,
    pub username: String,
    pub date_online: u32,
    #[serde(deserialize_with = "deserialize_avatar")]
    pub avatar: Option<Avatar>,
    pub timezone: String,
    pub language: String,
    #[serde(with = "url_serde")]
    pub profile_url: Url,
}

/// See the [Avatar Object](https://docs.mod.io/#avatar-object) docs for more informations.
#[derive(Debug, Deserialize)]
pub struct Avatar {
    pub filename: String,
    #[serde(with = "url_serde")]
    pub original: Url,
    #[serde(with = "url_serde")]
    pub thumb_50x50: Url,
    #[serde(with = "url_serde")]
    pub thumb_100x100: Url,
}

/// See the [Logo Object](https://docs.mod.io/#logo-object) docs for more informations.
#[derive(Debug, Deserialize)]
pub struct Logo {
    pub filename: String,
    #[serde(with = "url_serde")]
    pub original: Url,
    #[serde(with = "url_serde")]
    pub thumb_320x180: Url,
    #[serde(with = "url_serde")]
    pub thumb_640x360: Url,
    #[serde(with = "url_serde")]
    pub thumb_1280x720: Url,
}

/// See the [Event Object](https://docs.mod.io/#event-object) docs for more informations.
#[derive(Debug, Deserialize)]
pub struct Event {
    pub id: u32,
    pub mod_id: u32,
    pub user_id: u32,
    pub date_added: u64,
    pub event_type: EventType,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
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
    /// User has joined a team.
    UserTeamJoin,
    /// User has left a team.
    UserTeamLeave,
    /// User has subscribed to a mod.
    UserSubscribe,
    /// User has unsubscribed to a mod.
    UserUnsubscribe,
}

impl fmt::Display for EventType {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            EventType::ModfileChanged => "MODFILE_CHANGED",
            EventType::ModAvailable => "MOD_AVAILABLE",
            EventType::ModUnavailable => "MOD_UNAVAILABLE",
            EventType::ModEdited => "MOD_EDITED",
            EventType::ModDeleted => "MOD_DELETED",
            EventType::ModTeamChanged => "MOD_TEAM_CHANGED",
            EventType::UserTeamJoin => "USER_TEAM_JOIN",
            EventType::UserTeamLeave => "USER_TEAM_LEAVE",
            EventType::UserSubscribe => "USER_SUBSCRIBE",
            EventType::UserUnsubscribe => "USER_UNSUBSCRIBE",
        }.fmt(fmt)
    }
}

/// Deserialize empty objects for the `avatar` property of the User object as `None`.
///
/// The mod.io api returns `{"avatar": {}}` for users without avatars instead of returning
/// `{"avatar": null}`.
fn deserialize_avatar<'de, D>(deserializer: D) -> Result<Option<Avatar>, D::Error>
where
    D: Deserializer<'de>,
{
    match Avatar::deserialize(deserializer) {
        Ok(avatar) => Ok(Some(avatar)),
        Err(err) => {
            let err_s = err.to_string();
            if err_s.starts_with("missing field `filename`")
                || err_s.starts_with("invalid type: null")
            {
                Ok(None)
            } else {
                Err(err)
            }
        }
    }
}

pub mod game {
    use super::*;
    use std::fmt;

    /// See the [Game Object](https://docs.mod.io/#game-object) docs for more informations.
    #[derive(Debug, Deserialize)]
    pub struct Game {
        pub id: u32,
        pub status: u8,
        pub submitted_by: User,
        pub date_added: u64,
        pub date_updated: u64,
        pub date_live: u64,
        pub presentation_option: u8,
        pub submission_option: u8,
        pub curation_option: u8,
        pub community_options: u8,
        pub revenue_options: u16,
        pub api_access_options: u8,
        pub maturity_options: u8,
        pub ugc_name: String,
        pub icon: Icon,
        pub logo: Logo,
        pub header: HeaderImage,
        pub name: String,
        pub name_id: String,
        pub summary: String,
        pub instructions: Option<String>,
        #[serde(with = "url_serde")]
        pub instructions_url: Option<Url>,
        #[serde(with = "url_serde")]
        pub profile_url: Url,
        pub tag_options: Vec<TagOption>,
    }

    /// See the [Icon Object](https://docs.mod.io/#icon-object) docs for more informations.
    #[derive(Debug, Deserialize)]
    pub struct Icon {
        pub filename: String,
        #[serde(with = "url_serde")]
        pub original: Url,
        #[serde(with = "url_serde")]
        pub thumb_64x64: Url,
        #[serde(with = "url_serde")]
        pub thumb_128x128: Url,
        #[serde(with = "url_serde")]
        pub thumb_256x256: Url,
    }

    /// See the [Header Image Object](https://docs.mod.io/#header-image-object) docs for more
    /// informations.
    #[derive(Debug, Deserialize)]
    pub struct HeaderImage {
        pub filename: String,
        #[serde(with = "url_serde")]
        pub original: Url,
    }

    /// See the [Game Tag Option Object](https://docs.mod.io/#game-tag-option-object) docs for more
    /// informations.
    #[derive(Debug, Deserialize)]
    pub struct TagOption {
        pub name: String,
        #[serde(rename = "type")]
        pub kind: TagType,
        pub hidden: bool,
        pub tags: Vec<String>,
    }

    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "lowercase")]
    pub enum TagType {
        Checkboxes,
        Dropdown,
    }

    impl fmt::Display for TagType {
        fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
            match *self {
                TagType::Checkboxes => write!(fmt, "checkboxes"),
                TagType::Dropdown => write!(fmt, "dropdown"),
            }
        }
    }
}

pub mod mods {
    use super::*;
    use serde::de::{Deserialize, Deserializer};

    /// See the [Mod Object](https://docs.mod.io/#mod-object) docs for more informations.
    #[derive(Debug, Deserialize)]
    pub struct Mod {
        pub id: u32,
        pub game_id: u32,
        pub status: u32,
        pub visible: u32,
        pub submitted_by: User,
        pub date_added: u64,
        pub date_updated: u64,
        pub date_live: u64,
        pub maturity_option: u8,
        pub logo: Logo,
        #[serde(with = "url_serde")]
        pub homepage_url: Option<Url>,
        pub name: String,
        pub name_id: String,
        pub summary: String,
        pub description: Option<String>,
        pub description_plaintext: Option<String>,
        pub metadata_blob: Option<String>,
        #[serde(with = "url_serde")]
        pub profile_url: Url,
        #[serde(deserialize_with = "deserialize_modfile")]
        pub modfile: Option<File>,
        pub media: Media,
        #[serde(
            rename = "metadata_kvp",
            deserialize_with = "deserialize_kvp"
        )]
        pub metadata: MetadataMap,
        pub tags: Vec<Tag>,
        pub stats: Statistics,
    }

    /// See the [Mod Dependency Object](https://docs.mod.io/#mod-dependencies-object) docs for more
    /// informations.
    #[derive(Debug, Deserialize)]
    pub struct Dependency {
        pub mod_id: u32,
        pub date_added: u64,
    }

    /// See the [Mod Media Object](https://docs.mod.io/#mod-media-object) docs for more
    /// informations.
    #[derive(Debug, Deserialize)]
    pub struct Media {
        #[serde(default = "Vec::new")]
        pub youtube: Vec<String>,
        #[serde(default = "Vec::new")]
        pub sketchfab: Vec<String>,
        #[serde(default = "Vec::new")]
        pub images: Vec<Image>,
    }

    /// See the [Image Object](https://docs.mod.io/#image-object) docs for more informations.
    #[derive(Debug, Deserialize)]
    pub struct Image {
        pub filename: String,
        #[serde(with = "url_serde")]
        pub original: Url,
        #[serde(with = "url_serde")]
        pub thumb_320x180: Url,
    }

    /// See the [Statistics Object](https://docs.mod.io/#stats-object) docs for more
    /// informations.
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

    #[derive(Debug, Deserialize)]
    pub struct Popularity {
        #[serde(rename = "popularity_rank_position")]
        pub rank_position: u32,
        #[serde(rename = "popularity_rank_total_mods")]
        pub rank_total: u32,
    }

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

    /// See the [Rating Object](https://docs.mod.io/#rating-object) docs for more informations.
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
                }) => Ok(Rating::Positive {
                    game_id,
                    mod_id,
                    date_added,
                }),
                Ok(R {
                    game_id,
                    mod_id,
                    rating: -1,
                    date_added,
                }) => Ok(Rating::Negative {
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

    /// See the [Mod Tag Object](https://docs.mod.io/#mod-tag-object) docs for more informations.
    #[derive(Debug, Deserialize)]
    pub struct Tag {
        pub name: String,
        pub date_added: u64,
    }

    /// See the [Metadata KVP Object](https://docs.mod.io/#metadata-kvp-object) docs for more
    /// informations.
    pub type MetadataMap = HashMap<String, Vec<String>>;

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
        use serde::de::{SeqAccess, Visitor};
        use std::fmt;

        struct MetadataVisitor;

        impl<'de> Visitor<'de> for MetadataVisitor {
            type Value = MetadataMap;

            fn expecting(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
                fmt.write_str("metadata kvp")
            }

            fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
                #[derive(Deserialize)]
                struct KV {
                    metakey: String,
                    metavalue: String,
                }

                let mut map = MetadataMap::new();
                while let Ok(Some(elem)) = seq.next_element::<KV>() {
                    map.entry(elem.metakey)
                        .or_insert_with(Vec::new)
                        .push(elem.metavalue);
                }
                Ok(map)
            }
        }
        deserializer.deserialize_seq(MetadataVisitor)
    }

    /// See the [Comment Object](https://docs.mod.io/#comment-object) docs for more informations.
    #[derive(Debug, Deserialize)]
    pub struct Comment {
        pub id: u32,
        pub mod_id: u32,
        pub user: User,
        pub date_added: u64,
        pub reply_id: u32,
        pub thread_position: String,
        pub karma: u32,
        pub karma_guest: u32,
        pub content: String,
    }

    /// See the [Modfile Object](https://docs.mod.io/#modfile-object) docs for more informations.
    #[derive(Debug, Deserialize)]
    pub struct File {
        pub id: u32,
        pub mod_id: u32,
        pub date_added: u64,
        pub date_scanned: u64,
        pub virus_status: u32,
        pub virus_positive: u32,
        pub virustotal_hash: Option<String>,
        pub filesize: u64,
        pub filehash: FileHash,
        pub filename: String,
        pub version: Option<String>,
        pub changelog: Option<String>,
        pub metadata_blob: Option<String>,
        pub download: Download,
    }

    /// See the [Filehash Object](https://docs.mod.io/#filehash-object) docs for more informations.
    #[derive(Debug, Deserialize)]
    pub struct FileHash {
        pub md5: String,
    }

    /// See the [Download Object](https://docs.mod.io/#download-object) docs for more informations.
    #[derive(Debug, Deserialize)]
    pub struct Download {
        #[serde(with = "url_serde")]
        pub binary_url: Url,
        pub date_expires: u64,
    }

    /// See the [Team Member Object](https://docs.mod.io/#team-member-object) docs for more
    /// informations.
    #[derive(Debug, Deserialize)]
    pub struct TeamMember {
        pub id: u32,
        pub user: User,
        pub level: TeamLevel,
        pub date_added: u64,
        pub position: String,
    }

    #[derive(Clone, Copy, Debug)]
    pub enum TeamLevel {
        Moderator = 1,
        Creator = 4,
        Admin = 8,
    }

    impl TeamLevel {
        pub fn value(self) -> u64 {
            self as u64
        }
    }

    // impl Serialize, Deserialize for TeamLevel {{{
    impl ::serde::Serialize for TeamLevel {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::Serializer,
        {
            serializer.serialize_u64(*self as u64)
        }
    }

    impl<'de> ::serde::Deserialize<'de> for TeamLevel {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = TeamLevel;

                fn expecting(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    fmt.write_str("positive integer")
                }

                fn visit_u64<E>(self, value: u64) -> Result<TeamLevel, E>
                where
                    E: ::serde::de::Error,
                {
                    match value {
                        1 => Ok(TeamLevel::Moderator),
                        4 => Ok(TeamLevel::Creator),
                        8 => Ok(TeamLevel::Admin),
                        _ => Err(E::custom(format!(
                            "unknown {} value {}",
                            stringify!(TeamLevel),
                            value
                        ))),
                    }
                }
            }

            deserializer.deserialize_u64(Visitor)
        }
    }
    // }}}

    /// Deserialize empty objects for the `modfile` property of the Mod object as `None`.
    ///
    /// The mod.io api returns `{"modfile": {}}` for mods without files instead of returning
    /// `{"modfile": null}`.
    fn deserialize_modfile<'de, D>(deserializer: D) -> Result<Option<File>, D::Error>
    where
        D: Deserializer<'de>,
    {
        match File::deserialize(deserializer) {
            Ok(file) => Ok(Some(file)),
            Err(err) => {
                let err_s = err.to_string();
                if err_s.starts_with("missing field `id`")
                    || err_s.starts_with("invalid type: null")
                {
                    Ok(None)
                } else {
                    Err(err)
                }
            }
        }
    }
}

// vim: fdm=marker
