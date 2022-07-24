use std::fmt;

use serde::Deserialize;
use url::Url;

use crate::TargetPlatform;

/// See the [Modfile Object](https://docs.mod.io/#modfile-object) docs for more information.
#[derive(Debug, Deserialize)]
#[non_exhaustive]
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
    pub platforms: Vec<Platform>,
}

/// See the [Modfile Object](https://docs.mod.io/#modfile-object) docs for more information.
#[derive(Debug, Deserialize)]
#[non_exhaustive]
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
#[non_exhaustive]
pub struct FileHash {
    pub md5: String,
}

/// See the [Download Object](https://docs.mod.io/#download-object) docs for more information.
#[derive(Deserialize)]
#[non_exhaustive]
pub struct Download {
    pub binary_url: Url,
    pub date_expires: u64,
}

impl fmt::Debug for Download {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Download")
            .field("binary_url", &self.binary_url.as_str())
            .field("date_expires", &self.date_expires)
            .finish()
    }
}

/// See the [Modfile Platform Object](https://docs.mod.io/#modfile-platform-object) docs for more
/// information.
#[derive(Debug, Deserialize)]
#[non_exhaustive]
pub struct Platform {
    #[serde(rename = "platform")]
    pub target: TargetPlatform,
    pub status: PlatformStatus,
}

enum_number! {
    /// See the [Modfile Platform Object](https://docs.mod.io/#modfile-platform-object) docs for
    /// more information.
    #[derive(Clone, Copy, Debug, Deserialize)]
    #[serde(from = "u8")]
    #[non_exhaustive]
    pub enum PlatformStatus {
        Pending = 0,
        Approved = 1,
        Denied = 2,
        _ => Unknown(u8),
    }
}
