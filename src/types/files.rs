use std::fmt;

use serde::de::{Deserialize, Deserializer, IgnoredAny, MapAccess, Visitor};
use serde_derive::Deserialize;
use url::Url;

use crate::types::{DeserializeField, MissingField, TargetPlatform};

use super::id::{FileId, ModId};
use super::{utils, Timestamp};

/// See the [Modfile Object](https://docs.mod.io/#modfile-object) docs for more information.
#[derive(Debug)]
#[non_exhaustive]
pub struct File {
    pub id: FileId,
    pub mod_id: ModId,
    pub date_added: Timestamp,
    pub virus_scan: VirusScan,
    pub filesize: u64,
    pub filesize_uncompressed: u64,
    pub filehash: FileHash,
    pub filename: String,
    pub version: Option<String>,
    pub changelog: Option<String>,
    pub metadata_blob: Option<String>,
    pub download: Download,
    pub platforms: Vec<Platform>,
}

impl<'de> Deserialize<'de> for File {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[derive(Deserialize)]
        #[serde(field_identifier, rename_all = "snake_case")]
        enum Field {
            Id,
            ModId,
            DateAdded,
            DateScanned,
            VirusStatus,
            VirusPositive,
            Filesize,
            FilesizeUncompressed,
            Filehash,
            Filename,
            Version,
            Changelog,
            MetadataBlob,
            Download,
            Platforms,
            #[allow(dead_code)]
            Other(String),
        }

        struct FileVisitor;

        impl<'de> Visitor<'de> for FileVisitor {
            type Value = File;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("struct File")
            }

            fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut id = None;
                let mut mod_id = None;
                let mut date_added = None;
                let mut date_scanned = None;
                let mut virus_status = None;
                let mut virus_result = None;
                let mut filesize = None;
                let mut filesize_uncompressed = None;
                let mut filehash = None;
                let mut filename = None;
                let mut version = None;
                let mut changelog = None;
                let mut metadata_blob = None;
                let mut download = None;
                let mut platforms = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Id => {
                            id.deserialize_value("id", &mut map)?;
                        }
                        Field::ModId => {
                            mod_id.deserialize_value("mod_id", &mut map)?;
                        }
                        Field::DateAdded => {
                            date_added.deserialize_value("date_added", &mut map)?;
                        }
                        Field::DateScanned => {
                            date_scanned.deserialize_value("date_scanned", &mut map)?;
                        }
                        Field::VirusStatus => {
                            virus_status.deserialize_value("virus_status", &mut map)?;
                        }
                        Field::VirusPositive => {
                            virus_result.deserialize_value("virus_positive", &mut map)?;
                        }
                        Field::Filesize => {
                            filesize.deserialize_value("filesize", &mut map)?;
                        }
                        Field::FilesizeUncompressed => {
                            filesize_uncompressed
                                .deserialize_value("filesize_uncompressed", &mut map)?;
                        }
                        Field::Filehash => {
                            filehash.deserialize_value("filehash", &mut map)?;
                        }
                        Field::Filename => {
                            filename.deserialize_value("filename", &mut map)?;
                        }
                        Field::Version => {
                            version.deserialize_value("version", &mut map)?;
                        }
                        Field::Changelog => {
                            changelog.deserialize_value("changelog", &mut map)?;
                        }
                        Field::MetadataBlob => {
                            metadata_blob.deserialize_value("metadata_blob", &mut map)?;
                        }
                        Field::Download => {
                            download.deserialize_value("download", &mut map)?;
                        }
                        Field::Platforms => {
                            platforms.deserialize_value("platforms", &mut map)?;
                        }
                        Field::Other(_) => {
                            map.next_value::<IgnoredAny>()?;
                        }
                    }
                }

                let id = id.missing_field("id")?;
                let mod_id = mod_id.missing_field("mod_id")?;
                let date_added = date_added.missing_field("date_added")?;
                let date_scanned = date_scanned.missing_field("date_scanned")?;
                let virus_status = virus_status.missing_field("virus_status")?;
                let virus_result = virus_result.missing_field("virus_positive")?;
                let filesize = filesize.missing_field("filesize")?;
                let filesize_uncompressed =
                    filesize_uncompressed.missing_field("filesize_uncompressed")?;
                let filehash = filehash.missing_field("filehash")?;
                let filename = filename.missing_field("filename")?;
                let version = version.missing_field("version")?;
                let changelog = changelog.missing_field("changelog")?;
                let metadata_blob = metadata_blob.missing_field("metadata_blob")?;
                let download = download.missing_field("download")?;
                let platforms = platforms.missing_field("platforms")?;

                Ok(File {
                    id,
                    mod_id,
                    date_added,
                    virus_scan: VirusScan {
                        date_scanned,
                        status: virus_status,
                        result: virus_result,
                    },
                    filesize,
                    filesize_uncompressed,
                    filehash,
                    filename,
                    version,
                    changelog,
                    metadata_blob,
                    download,
                    platforms,
                })
            }
        }

        deserializer.deserialize_map(FileVisitor)
    }
}

/// See the [Modfile Object](https://docs.mod.io/#modfile-object) docs for more information.
#[derive(Debug)]
#[non_exhaustive]
pub struct VirusScan {
    pub date_scanned: Timestamp,
    pub status: VirusStatus,
    pub result: VirusResult,
}

newtype_enum! {
    /// See the [Modfile Object](https://docs.mod.io/#modfile-object) docs for more information.
    pub struct VirusStatus: u8 {
        const NOT_SCANNED       = 0;
        const SCAN_COMPLETED    = 1;
        const IN_PROGRESS       = 2;
        const TOO_LARGE_TO_SCAN = 3;
        const FILE_NOT_FOUND    = 4;
        const ERROR_SCANNING    = 5;
    }

    /// See the [Modfile Object](https://docs.mod.io/#modfile-object) docs for more information.
    pub struct VirusResult: u8 {
        const NO_THREATS_DETECTED = 0;
        const MALICIOUS           = 1;
        const POTENTIALLY_HARMFUL = 2;
    }
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
    #[serde(with = "utils::url")]
    pub binary_url: Url,
    pub date_expires: Timestamp,
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

newtype_enum! {
    /// See the [Modfile Platform Object](https://docs.mod.io/#modfile-platform-object) docs for
    /// more information.
    pub struct PlatformStatus: u8 {
        const PENDING  = 0;
        const APPROVED = 1;
        const DENIED   = 2;
    }
}
