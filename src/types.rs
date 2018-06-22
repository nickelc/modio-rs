use std::collections::HashMap;

use serde::de::{Deserialize, Deserializer};
use url::Url;
use url_serde;

#[derive(Debug, Deserialize)]
pub struct ModioMessage {
    code: u16,
    message: String,
}

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

#[derive(Debug, Deserialize)]
pub struct ModioErrorResponse {
    #[serde(rename = "error")]
    pub error: ClientError,
}

#[derive(Debug, Deserialize)]
pub struct ClientError {
    code: u16,
    message: String,
    errors: Option<HashMap<String, String>>,
}

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

    #[derive(Debug, Deserialize)]
    pub struct HeaderImage {
        filename: String,
        #[serde(with = "url_serde")]
        original: Url,
    }

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
        // metadata_kvp: MetadataKVP,
        tags: Vec<Tag>,
    }

    #[derive(Debug, Deserialize)]
    pub struct Dependency {
        mod_id: u32,
        date_added: u64,
    }

    #[derive(Debug, Deserialize)]
    pub struct Media {
        #[serde(default = "Vec::new")]
        youtube: Vec<String>,
        #[serde(default = "Vec::new")]
        sketchfab: Vec<String>,
        #[serde(default = "Vec::new")]
        images: Vec<Image>,
    }

    #[derive(Debug, Deserialize)]
    pub struct Image {
        filename: String,
        #[serde(with = "url_serde")]
        original: Url,
        #[serde(with = "url_serde")]
        thumb_320x180: Url,
    }

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

    #[derive(Debug, Deserialize)]
    pub struct Tag {
        name: String,
        date_added: u64,
    }

    #[derive(Debug, Deserialize)]
    pub struct Comment {
        id: u32,
        mod_id: u32,
        #[serde(rename = "user")]
        submitted_by: User,
        date_added: u64,
        reply_id: u32,
        thread_position: String,
        karma: u32,
        karma_guest: u32,
        content: String,
    }

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
        version: String,
        changelog: Option<String>,
        metadata_blob: Option<String>,
        download: Download,
    }

    #[derive(Debug, Deserialize)]
    pub struct FileHash {
        md5: String,
    }

    #[derive(Debug, Deserialize)]
    pub struct Download {
        #[serde(with = "url_serde")]
        binary_url: Url,
        date_expires: u64,
    }

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
