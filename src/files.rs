use std::collections::HashMap;

use hyper::client::Connect;
use url::{form_urlencoded, Url};
use url_serde;

use Future;
use Modio;
use ModioListResponse;

pub struct MyFiles<C>
where
    C: Clone + Connect,
{
    modio: Modio<C>,
}

impl<C: Clone + Connect> MyFiles<C> {
    pub fn new(modio: Modio<C>) -> Self {
        Self { modio }
    }

    pub fn list(&self, options: &FileListOptions) -> Future<ModioListResponse<File>> {
        let mut uri = vec!["/me/files".to_owned()];
        if let Some(query) = options.serialize() {
            uri.push(query);
        }
        self.modio.get(&uri.join("?"))
    }
}

pub struct Files<C>
where
    C: Clone + Connect,
{
    modio: Modio<C>,
    game: u32,
    mod_id: u32,
}

impl<C: Clone + Connect> Files<C> {
    pub fn new(modio: Modio<C>, game: u32, mod_id: u32) -> Self {
        Self {
            modio,
            game,
            mod_id,
        }
    }

    fn path(&self, more: &str) -> String {
        format!("/games/{}/mods/{}/files{}", self.game, self.mod_id, more)
    }

    pub fn list(&self, options: &FileListOptions) -> Future<ModioListResponse<File>> {
        let mut uri = vec![self.path("")];
        if let Some(query) = options.serialize() {
            uri.push(query);
        }
        self.modio.get(&uri.join("?"))
    }

    pub fn get(&self, id: u32) -> Future<File> {
        self.modio.get(&self.path(&format!("/{}", id)))
    }
}

#[derive(Default)]
pub struct FileListOptions {
    params: HashMap<&'static str, String>,
}

impl FileListOptions {
    pub fn serialize(&self) -> Option<String> {
        if self.params.is_empty() {
            None
        } else {
            let encoded = form_urlencoded::Serializer::new(String::new())
                .extend_pairs(&self.params)
                .finish();
            Some(encoded)
        }
    }
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
