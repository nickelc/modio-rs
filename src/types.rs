use std::collections::HashMap;

use serde::de::{Deserialize, Deserializer};
use url::Url;
use url_serde;

/// See the [Message Object](https://docs.mod.io/#message-object) docs for more informations.
#[derive(Debug, Deserialize)]
pub struct ModioMessage {
    code: u16,
    message: String,
}

/// See the [Multiple Item Response](https://docs.mod.io/#response-formats) docs for more
/// informations.
#[derive(Debug, Deserialize)]
pub struct ModioListResponse<T> {
    data: Vec<T>,
    #[serde(rename = "result_count")]
    count: u32,
    #[serde(rename = "result_limit")]
    limit: u32,
    #[serde(rename = "result_offset")]
    offset: u32,
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
    code: u16,
    message: String,
    errors: Option<HashMap<String, String>>,
}

/// See the [User Object](https://docs.mod.io/#user-object) docs for more informations.
#[derive(Debug, Deserialize)]
pub struct User {
    id: u32,
    name_id: String,
    username: String,
    date_online: u32,
    #[serde(deserialize_with = "deserialize_avatar")]
    avatar: Option<Avatar>,
    timezone: String,
    language: String,
    #[serde(with = "url_serde")]
    profile_url: Url,
}

/// See the [Avatar Object](https://docs.mod.io/#avatar-object) docs for more informations.
#[derive(Debug, Deserialize)]
pub struct Avatar {
    filename: String,
    #[serde(with = "url_serde")]
    original: Url,
    #[serde(with = "url_serde")]
    thumb_50x50: Url,
    #[serde(with = "url_serde")]
    thumb_100x100: Url,
}

/// See the [Logo Object](https://docs.mod.io/#logo-object) docs for more informations.
#[derive(Debug, Deserialize)]
pub struct Logo {
    filename: String,
    #[serde(with = "url_serde")]
    original: Url,
    #[serde(with = "url_serde")]
    thumb_320x180: Url,
    #[serde(with = "url_serde")]
    thumb_640x360: Url,
    #[serde(with = "url_serde")]
    thumb_1280x720: Url,
}

/// See the [Event Object](https://docs.mod.io/#event-object) docs for more informations.
#[derive(Debug, Deserialize)]
pub struct Event {
    id: u32,
    mod_id: u32,
    user_id: u32,
    date_added: u64,
    event_type: EventType,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum EventType {
    ModfileChanged,
    ModAvailable,
    ModUnavailable,
    ModEdited,
    ModDeleted,
    ModTeamChanged,
    UserTeamJoin,
    UserTeamLeave,
    UserSubscribe,
    UserUnsubscribe,
}

fn deserialize_avatar<'de, D>(deserializer: D) -> Result<Option<Avatar>, D::Error>
where
    D: Deserializer<'de>,
{
    match Avatar::deserialize(deserializer) {
        Ok(avatar) => Ok(Some(avatar)),
        Err(err) => {
            let err_s = format!("{}", err);
            if err_s.starts_with("missing field `filename`") {
                Ok(None)
            } else if err_s.starts_with("invalid type: null") {
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
        id: u32,
        status: u8,
        submitted_by: User,
        date_added: u64,
        date_updated: u64,
        date_live: u64,
        presentation_option: u8,
        submission_option: u8,
        curation_option: u8,
        community_options: u8,
        revenue_options: u16,
        api_access_options: u8,
        maturity_options: u8,
        ugc_name: String,
        icon: Icon,
        logo: Logo,
        header: HeaderImage,
        name: String,
        name_id: String,
        summary: String,
        instructions: Option<String>,
        #[serde(with = "url_serde")]
        instructions_url: Option<Url>,
        #[serde(with = "url_serde")]
        profile_url: Url,
        tag_options: Vec<TagOption>,
    }

    /// See the [Icon Object](https://docs.mod.io/#icon-object) docs for more informations.
    #[derive(Debug, Deserialize)]
    pub struct Icon {
        filename: String,
        #[serde(with = "url_serde")]
        original: Url,
        #[serde(with = "url_serde")]
        thumb_64x64: Url,
        #[serde(with = "url_serde")]
        thumb_128x128: Url,
        #[serde(with = "url_serde")]
        thumb_256x256: Url,
    }

    /// See the [Header Image Object](https://docs.mod.io/#header-image-object) docs for more
    /// informations.
    #[derive(Debug, Deserialize)]
    pub struct HeaderImage {
        filename: String,
        #[serde(with = "url_serde")]
        original: Url,
    }

    /// See the [Game Tag Option Object](https://docs.mod.io/#game-tag-option-object) docs for more
    /// informations.
    #[derive(Debug, Deserialize)]
    pub struct TagOption {
        name: String,
        #[serde(rename = "type")]
        kind: TagType,
        hidden: bool,
        tags: Vec<String>,
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
        id: u32,
        game_id: u32,
        status: u32,
        visible: u32,
        submitted_by: User,
        date_added: u64,
        date_updated: u64,
        date_live: u64,
        maturity_option: u8,
        logo: Logo,
        #[serde(with = "url_serde")]
        homepage_url: Option<Url>,
        name: String,
        name_id: String,
        summary: String,
        description: Option<String>,
        metadata_blob: Option<String>,
        #[serde(with = "url_serde")]
        profile_url: Url,
        #[serde(deserialize_with = "deserialize_modfile")]
        modfile: Option<File>,
        media: Media,
        #[serde(rename = "rating_summary")]
        ratings: Ratings,
        #[serde(rename = "metadata_kvp", deserialize_with = "deserialize_kvp")]
        metadata: MetadataMap,
        tags: Vec<Tag>,
    }

    /// See the [Mod Dependency Object](https://docs.mod.io/#mod-dependencies-object) docs for more
    /// informations.
    #[derive(Debug, Deserialize)]
    pub struct Dependency {
        mod_id: u32,
        date_added: u64,
    }

    /// See the [Mod Media Object](https://docs.mod.io/#mod-media-object) docs for more
    /// informations.
    #[derive(Debug, Deserialize)]
    pub struct Media {
        #[serde(default = "Vec::new")]
        youtube: Vec<String>,
        #[serde(default = "Vec::new")]
        sketchfab: Vec<String>,
        #[serde(default = "Vec::new")]
        images: Vec<Image>,
    }

    /// See the [Image Object](https://docs.mod.io/#image-object) docs for more informations.
    #[derive(Debug, Deserialize)]
    pub struct Image {
        filename: String,
        #[serde(with = "url_serde")]
        original: Url,
        #[serde(with = "url_serde")]
        thumb_320x180: Url,
    }

    /// See the [Rating Summary Object](https://docs.mod.io/#rating-summary-object) docs for more
    /// informations.
    #[derive(Debug, Deserialize)]
    pub struct Ratings {
        #[serde(rename = "total_ratings")]
        total: u32,
        #[serde(rename = "positive_ratings")]
        positive: u32,
        #[serde(rename = "negative_ratings")]
        negative: u32,
        percentage_positive: u32,
        weighted_aggregate: f32,
        display_text: String,
    }

    /// See the [Mod Tag Object](https://docs.mod.io/#mod-tag-object) docs for more informations.
    #[derive(Debug, Deserialize)]
    pub struct Tag {
        name: String,
        date_added: u64,
    }

    /// See the [Metadata KVP Object](https://docs.mod.io/#metadata-kvp-object) docs for more
    /// informations.
    pub type MetadataMap = HashMap<String, Vec<String>>;

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
        id: u32,
        mod_id: u32,
        user: User,
        date_added: u64,
        reply_id: u32,
        thread_position: String,
        karma: u32,
        karma_guest: u32,
        content: String,
    }

    /// See the [Modfile Object](https://docs.mod.io/#modfile-object) docs for more informations.
    #[derive(Debug, Deserialize)]
    pub struct File {
        id: u32,
        mod_id: u32,
        date_added: u64,
        date_scanned: u64,
        virus_status: u32,
        virus_positive: u32,
        virustotal_hash: Option<String>,
        filesize: u64,
        filehash: FileHash,
        filename: String,
        version: Option<String>,
        changelog: Option<String>,
        metadata_blob: Option<String>,
        download: Download,
    }

    /// See the [Filehash Object](https://docs.mod.io/#filehash-object) docs for more informations.
    #[derive(Debug, Deserialize)]
    pub struct FileHash {
        md5: String,
    }

    /// See the [Download Object](https://docs.mod.io/#download-object) docs for more informations.
    #[derive(Debug, Deserialize)]
    pub struct Download {
        #[serde(with = "url_serde")]
        binary_url: Url,
        date_expires: u64,
    }

    /// See the [Team Member Object](https://docs.mod.io/#team-member-object) docs for more
    /// informations.
    #[derive(Debug, Deserialize)]
    pub struct TeamMember {
        id: u32,
        user: User,
        level: TeamLevel,
        date_added: u64,
        position: String,
    }

    #[derive(Clone, Copy, Debug)]
    pub enum TeamLevel {
        Moderator = 1,
        Creator = 4,
        Admin = 8,
    }

    impl TeamLevel {
        pub fn value(&self) -> u64 {
            (*self as u64)
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

    fn deserialize_modfile<'de, D>(deserializer: D) -> Result<Option<File>, D::Error>
    where
        D: Deserializer<'de>,
    {
        match File::deserialize(deserializer) {
            Ok(file) => Ok(Some(file)),
            Err(err) => {
                let err_s = format!("{}", err);
                if err_s.starts_with("missing field `id`") {
                    Ok(None)
                } else if err_s.starts_with("invalid type: null") {
                    Ok(None)
                } else {
                    Err(err)
                }
            }
        }
    }
}

// vim: fdm=marker
