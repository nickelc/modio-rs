use std::fmt;

use serde::Deserialize;
use url::Url;

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
#[derive(Deserialize)]
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
