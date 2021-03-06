use std::collections::HashMap;
use std::fmt;

use serde::{Deserialize, Deserializer};
use url::Url;

// macro: enum_number {{{
macro_rules! enum_number {
    (
        $(#[$outer:meta])*
        pub enum $name:ident {
            $(
                $(#[$inner:ident $($args:tt)*])*
                $variant:ident = $value:expr,
            )*
        }
    ) => {
        $(#[$outer])*
        #[derive(Clone, Copy)]
        pub enum $name {
            $(
                $(#[$inner $($args)*])*
                $variant = $value,
            )*
        }

        impl ::std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                (*self as u8).fmt(f)
            }
        }

        impl ::serde::Serialize for $name {
            fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
            where
                S: ::serde::Serializer,
            {
                // Serialize the enum as a u64.
                serializer.serialize_u64(*self as u64)
            }
        }

        impl<'de> ::serde::Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
            where
                D: ::serde::Deserializer<'de>,
            {
                struct Visitor;

                impl<'de> ::serde::de::Visitor<'de> for Visitor {
                    type Value = $name;

                    fn expecting(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        fmt.write_str("positive integer")
                    }

                    fn visit_u64<E>(self, value: u64) -> ::std::result::Result<$name, E>
                    where
                        E: ::serde::de::Error,
                    {
                        match value {
                            $( $value => Ok($name::$variant), )*
                            _ => Err(E::custom(format!(
                                "unknown {} value {}",
                                stringify!($name),
                                value
                            ))),
                        }
                    }
                }

                deserializer.deserialize_u64(Visitor)
            }
        }
    };
}
// }}}

// macro: bitflags_serde {{{
macro_rules! bitflags_serde {
    ($name:ident, $type:ty) => {
        impl<'de> ::serde::Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
            where
                D: ::serde::Deserializer<'de>,
            {
                struct Visitor;

                impl<'de> ::serde::de::Visitor<'de> for Visitor {
                    type Value = $name;

                    fn expecting(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        fmt.write_str("positive integer")
                    }

                    fn visit_u64<E>(self, value: u64) -> ::std::result::Result<$name, E>
                    where
                        E: ::serde::de::Error,
                    {
                        $name::from_bits(value as $type).ok_or_else(|| {
                            E::custom(format!("invalid {} value: {}", stringify!($name), value))
                        })
                    }
                }

                deserializer.deserialize_u64(Visitor)
            }
        }

        impl ::std::fmt::Display for $name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                self.bits.fmt(f)
            }
        }
    };
}
// }}}

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
            EventType::UserTeamJoin => "USER_TEAM_JOIN".fmt(f),
            EventType::UserTeamLeave => "USER_TEAM_LEAVE".fmt(f),
            EventType::UserSubscribe => "USER_SUBSCRIBE".fmt(f),
            EventType::UserUnsubscribe => "USER_UNSUBSCRIBE".fmt(f),
            EventType::Other(s) => s.fmt(f),
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

pub mod auth {
    use serde::Deserialize;
    use url::Url;

    /// See the [Terms Object](https://docs.mod.io/#terms-object) docs for more information.
    #[derive(Debug, Deserialize)]
    pub struct Terms {
        pub plaintext: String,
        pub html: String,
        pub links: Links,
    }

    /// Part of [`Terms`]
    ///
    /// See the [Terms Object](https://docs.mod.io/#terms-object) docs for more information.
    #[derive(Debug, Deserialize)]
    pub struct Links {
        pub website: Link,
        pub terms: Link,
        pub privacy: Link,
        pub manage: Link,
    }

    /// Part of [`Terms`]
    ///
    /// See the [Terms Object](https://docs.mod.io/#terms-object) docs for more information.
    #[derive(Debug, Deserialize)]
    pub struct Link {
        pub text: String,
        pub url: Url,
        pub required: bool,
    }
}

pub mod game {
    use std::collections::HashMap;
    use std::fmt;

    use bitflags::bitflags;
    use serde::Deserialize;
    use url::Url;

    use super::deserialize_empty_object;
    use super::{Logo, Status, User};

    /// See the [Game Object](https://docs.mod.io/#game-object) docs for more information.
    #[derive(Debug, Deserialize)]
    pub struct Game {
        pub id: u32,
        pub status: Status,
        pub submitted_by: User,
        pub date_added: u64,
        pub date_updated: u64,
        pub date_live: u64,
        pub presentation_option: PresentationOption,
        pub submission_option: SubmissionOption,
        pub curation_option: CurationOption,
        pub community_options: CommunityOptions,
        pub revenue_options: RevenueOptions,
        pub api_access_options: ApiAccessOptions,
        pub maturity_options: MaturityOptions,
        pub ugc_name: String,
        pub icon: Icon,
        pub logo: Logo,
        #[serde(default, deserialize_with = "deserialize_empty_object")]
        pub header: Option<HeaderImage>,
        pub name: String,
        pub name_id: String,
        pub summary: String,
        pub instructions: Option<String>,
        pub instructions_url: Option<Url>,
        pub profile_url: Url,
        pub tag_options: Vec<TagOption>,
    }

    enum_number! {
        /// Presentation style used on the mod.io website.
        #[derive(Debug)]
        pub enum PresentationOption {
            /// Displays mods in a grid.
            GridView = 0,
            /// Displays mods in a table.
            TableView = 1,
        }
    }

    enum_number! {
        /// Submission process modders must follow.
        #[derive(Debug)]
        pub enum SubmissionOption {
            /// Mod uploads must occur via the API using a tool by the game developers.
            ApiOnly = 0,
            /// Mod uploads can occur from anywhere, include the website and API.
            Anywhere = 1,
        }
    }

    enum_number! {
        /// Curation process used to approve mods.
        #[derive(Debug)]
        pub enum CurationOption {
            /// No curation: Mods are immediately available to play.
            No = 0,
            /// Paid curation: Mods are immediately to play unless they choose to receive
            /// donations. These mods must be accepted to be listed.
            Paid = 1,
            /// Full curation: All mods must be accepted by someone to be listed.
            Full = 2,
        }
    }

    enum_number! {
        /// Option to allow developers to select if they flag their mods containing mature content.
        #[derive(Debug)]
        pub enum MaturityOptions {
            NotAllowed = 0,
            /// Allow flagging mods as mature.
            Allowed = 1,
        }
    }

    bitflags! {
        /// Community features enabled on the mod.io website.
        pub struct CommunityOptions: u8 {
            /// Discussion board enabled.
            const DISCUSSIONS       = 0b0001;
            /// Guides & News enabled.
            const GUIDES_NEWS       = 0b0010;
            /// "Subscribe to install" button disabled.
            const DISABLE_SUBSCRIBE = 0b0100;
            const ALL = Self::DISCUSSIONS.bits | Self::GUIDES_NEWS.bits | Self::DISABLE_SUBSCRIBE.bits;
        }
    }
    bitflags_serde!(CommunityOptions, u8);

    bitflags! {
        /// Revenue capabilities mods can enable.
        pub struct RevenueOptions: u8 {
            /// Allow mods to be sold.
            const SELL      = 0b0001;
            /// Allow mods to receive donations.
            const DONATIONS = 0b0010;
            /// Allow mods to be traded.
            const TRADE     = 0b0100;
            /// Allow mods to control supply and scarcity.
            const SCARCITY  = 0b1000;
            const ALL = Self::SELL.bits | Self::DONATIONS.bits | Self::TRADE.bits | Self::SCARCITY.bits;
        }
    }
    bitflags_serde!(RevenueOptions, u8);

    bitflags! {
        /// Level of API access allowed by a game.
        pub struct ApiAccessOptions: u8 {
            /// Allow third parties to access a game's API endpoints.
            const ALLOW_THIRD_PARTY     = 0b0001;
            /// Allow mods to be downloaded directly.
            const ALLOW_DIRECT_DOWNLOAD = 0b0010;
            const ALL = Self::ALLOW_THIRD_PARTY.bits | Self::ALLOW_DIRECT_DOWNLOAD.bits;
        }
    }
    bitflags_serde!(ApiAccessOptions, u8);

    /// See the [Icon Object](https://docs.mod.io/#icon-object) docs for more information.
    #[derive(Debug, Deserialize)]
    pub struct Icon {
        pub filename: String,
        pub original: Url,
        pub thumb_64x64: Url,
        pub thumb_128x128: Url,
        pub thumb_256x256: Url,
    }

    /// See the [Header Image Object](https://docs.mod.io/#header-image-object) docs for more
    /// information.
    #[derive(Debug, Deserialize)]
    pub struct HeaderImage {
        pub filename: String,
        pub original: Url,
    }

    /// See the [Game Statistics Object](https://docs.mod.io/#game-stats-object) docs for more
    /// information.
    #[derive(Debug, Deserialize)]
    pub struct Statistics {
        pub game_id: u32,
        #[serde(rename = "mods_count_total")]
        pub mods_total: u32,
        #[serde(rename = "mods_subscribers_total")]
        pub subscribers_total: u32,
        #[serde(flatten)]
        pub downloads: Downloads,
        #[serde(rename = "date_expires")]
        pub expired_at: u64,
    }

    /// Part of [`Statistics`]
    #[derive(Debug, Deserialize)]
    pub struct Downloads {
        #[serde(rename = "mods_downloads_total")]
        pub total: u32,
        #[serde(rename = "mods_downloads_today")]
        pub today: u32,
        #[serde(rename = "mods_downloads_daily_average")]
        pub daily_average: u32,
    }

    /// See the [Game Tag Option Object](https://docs.mod.io/#game-tag-option-object) docs for more
    /// information.
    #[derive(Debug, Deserialize)]
    pub struct TagOption {
        pub name: String,
        #[serde(rename = "type")]
        pub kind: TagType,
        #[serde(rename = "tag_count_map")]
        pub tag_count: HashMap<String, u32>,
        pub hidden: bool,
        pub tags: Vec<String>,
    }

    /// Defines the type of a tag. See [`TagOption`].
    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "lowercase")]
    pub enum TagType {
        Checkboxes,
        Dropdown,
    }

    impl fmt::Display for TagType {
        fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
            match *self {
                TagType::Checkboxes => write!(fmt, "checkboxes"),
                TagType::Dropdown => write!(fmt, "dropdown"),
            }
        }
    }
}

pub mod mods {
    use std::collections::HashMap;
    use std::fmt;

    use bitflags::bitflags;
    use serde::{Deserialize, Deserializer};
    use url::Url;

    use super::{deserialize_empty_object, NonExhaustive};
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
    }

    enum_number! {
        /// See [Status & Visibility](https://docs.mod.io/#status-amp-visibility) docs for more information.
        #[derive(Debug)]
        pub enum Visibility {
            Hidden = 0,
            Public = 1,
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
    bitflags_serde!(MaturityOption, u8);

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
    #[derive(Debug, PartialEq, Eq, Hash, Deserialize)]
    #[serde(from = "NonExhaustive<KnownEventType>")]
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
        Other(String),
    }

    impl fmt::Display for EventType {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                EventType::ModfileChanged => "MODFILE_CHANGED".fmt(f),
                EventType::ModAvailable => "MOD_AVAILABLE".fmt(f),
                EventType::ModUnavailable => "MOD_UNAVAILABLE".fmt(f),
                EventType::ModEdited => "MOD_EDITED".fmt(f),
                EventType::ModDeleted => "MOD_DELETED".fmt(f),
                EventType::ModTeamChanged => "MOD_TEAM_CHANGED".fmt(f),
                EventType::ModCommentAdded => "MOD_COMMENT_ADDED".fmt(f),
                EventType::ModCommentDeleted => "MOD_COMMENT_DELETED".fmt(f),
                EventType::Other(s) => s.fmt(f),
            }
        }
    }

    // impl #[serde(from = "NonExhaustive<KnowEventType>")] {{{
    #[derive(Deserialize)]
    #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
    #[allow(clippy::enum_variant_names)]
    enum KnownEventType {
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
    }

    impl From<NonExhaustive<KnownEventType>> for EventType {
        fn from(kind: NonExhaustive<KnownEventType>) -> EventType {
            use KnownEventType::*;
            use NonExhaustive::*;

            match kind {
                Known(ModfileChanged) => EventType::ModfileChanged,
                Known(ModAvailable) => EventType::ModAvailable,
                Known(ModUnavailable) => EventType::ModUnavailable,
                Known(ModEdited) => EventType::ModEdited,
                Known(ModDeleted) => EventType::ModDeleted,
                Known(ModTeamChanged) => EventType::ModTeamChanged,
                Known(ModCommentAdded) => EventType::ModCommentAdded,
                Known(ModCommentDeleted) => EventType::ModCommentDeleted,
                Unknown(other) => EventType::Other(other),
            }
        }
    }
    // }}}

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
    #[derive(Debug, Deserialize)]
    pub struct Image {
        pub filename: String,
        pub original: Url,
        pub thumb_320x180: Url,
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
            Default::default()
        }

        pub fn with_capacity(capacity: usize) -> Self {
            MetadataMap(HashMap::with_capacity(capacity))
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
        use serde::de::{SeqAccess, Visitor};

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

                let size = seq.size_hint().unwrap_or_default();
                let mut map = MetadataMap::with_capacity(size);

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
        pub mod_id: u32,
        pub user: User,
        pub date_added: u64,
        pub reply_id: u32,
        pub thread_position: String,
        pub karma: u32,
        pub karma_guest: u32,
        pub content: String,
    }

    /// See the [Modfile Object](https://docs.mod.io/#modfile-object) docs for more information.
    #[derive(Debug, Deserialize)]
    pub struct File {
        pub id: u32,
        pub mod_id: u32,
        pub date_added: u64,
        #[serde(flatten)]
        pub virus_scan: VirusScan,
        pub filesize: u64,
        pub filehash: FileHash,
        pub filename: String,
        pub version: Option<String>,
        pub changelog: Option<String>,
        pub metadata_blob: Option<String>,
        pub download: Download,
    }

    /// See the [Modfile Object](https://docs.mod.io/#modfile-object) docs for more information.
    #[derive(Debug, Deserialize)]
    pub struct VirusScan {
        pub date_scanned: u64,
        #[serde(rename = "virus_status")]
        pub status: u32,
        #[serde(rename = "virus_positive")]
        pub result: u32,
        pub virustotal_hash: Option<String>,
    }

    /// See the [Filehash Object](https://docs.mod.io/#filehash-object) docs for more information.
    #[derive(Debug, Deserialize)]
    pub struct FileHash {
        pub md5: String,
    }

    /// See the [Download Object](https://docs.mod.io/#download-object) docs for more information.
    #[derive(Debug, Deserialize)]
    pub struct Download {
        pub binary_url: Url,
        pub date_expires: u64,
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
        #[derive(Debug)]
        pub enum TeamLevel {
            Moderator = 1,
            Creator = 4,
            Admin = 8,
        }
    }

    impl TeamLevel {
        pub fn value(self) -> u64 {
            self as u64
        }
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
