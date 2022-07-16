use std::collections::HashMap;
use std::fmt;

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
    #[derive(Clone, Copy, Debug, Deserialize)]
    #[serde(from = "u8")]
    pub enum PresentationOption {
        /// Displays mods in a grid.
        GridView = 0,
        /// Displays mods in a table.
        TableView = 1,
        _ => Unknown(u8),
    }
}

enum_number! {
    /// Submission process modders must follow.
    #[derive(Clone, Copy, Debug, Deserialize)]
    #[serde(from = "u8")]
    pub enum SubmissionOption {
        /// Mod uploads must occur via the API using a tool by the game developers.
        ApiOnly = 0,
        /// Mod uploads can occur from anywhere, include the website and API.
        Anywhere = 1,
        _ => Unknown(u8),
    }
}

enum_number! {
    /// Curation process used to approve mods.
    #[derive(Clone, Copy, Debug, Deserialize)]
    #[serde(from = "u8")]
    pub enum CurationOption {
        /// No curation: Mods are immediately available to play.
        No = 0,
        /// Paid curation: Mods are immediately to play unless they choose to receive
        /// donations. These mods must be accepted to be listed.
        Paid = 1,
        /// Full curation: All mods must be accepted by someone to be listed.
        Full = 2,
        _ => Unknown(u8),
    }
}

enum_number! {
    /// Option to allow developers to select if they flag their mods containing mature content.
    #[derive(Clone, Copy, Debug, Deserialize)]
    #[serde(from = "u8")]
    pub enum MaturityOptions {
        NotAllowed = 0,
        /// Allow flagging mods as mature.
        Allowed = 1,
        _ => Unknown(u8),
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

    /// Level of API access allowed by a game.
    pub struct ApiAccessOptions: u8 {
        /// Allow third parties to access a game's API endpoints.
        const ALLOW_THIRD_PARTY     = 0b0001;
        /// Allow mods to be downloaded directly.
        const ALLOW_DIRECT_DOWNLOAD = 0b0010;
        const ALL = Self::ALLOW_THIRD_PARTY.bits | Self::ALLOW_DIRECT_DOWNLOAD.bits;
    }
}

/// See the [Icon Object](https://docs.mod.io/#icon-object) docs for more information.
#[derive(Deserialize)]
pub struct Icon {
    pub filename: String,
    pub original: Url,
    pub thumb_64x64: Url,
    pub thumb_128x128: Url,
    pub thumb_256x256: Url,
}

impl fmt::Debug for Icon {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Icon")
            .field("filename", &self.filename)
            .field("original", &self.original.as_str())
            .field("thumb_64x64", &self.thumb_64x64.as_str())
            .field("thumb_128x128", &self.thumb_128x128.as_str())
            .field("thumb_256x256", &self.thumb_256x256.as_str())
            .finish()
    }
}

/// See the [Header Image Object](https://docs.mod.io/#header-image-object) docs for more
/// information.
#[derive(Deserialize)]
pub struct HeaderImage {
    pub filename: String,
    pub original: Url,
}

impl fmt::Debug for HeaderImage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("HeaderImage")
            .field("filename", &self.filename)
            .field("original", &self.original.as_str())
            .finish()
    }
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
        match self {
            Self::Checkboxes => fmt.write_str("checkboxes"),
            Self::Dropdown => fmt.write_str("dropdown"),
        }
    }
}
