use std::collections::HashMap;
use std::path::{Path, PathBuf};

use futures::future;
use hyper::client::Connect;
use hyper_multipart::client::multipart;
use serde_urlencoded;
use url::form_urlencoded;

use errors::Error;
use types::mods::File;
use types::ModioListResponse;
use Future;
use Modio;
use MultipartForm;

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

    pub fn get(&self, id: u32) -> FileRef<C> {
        FileRef::new(self.modio.clone(), self.game, self.mod_id, id)
    }

    pub fn add(&self, options: AddFileOptions) -> Future<File> {
        self.modio.post_form(&self.path(""), options)
    }
}

pub struct FileRef<C>
where
    C: Clone + Connect,
{
    modio: Modio<C>,
    game: u32,
    mod_id: u32,
    id: u32,
}

impl<C: Clone + Connect> FileRef<C> {
    pub fn new(modio: Modio<C>, game: u32, mod_id: u32, id: u32) -> Self {
        Self {
            modio,
            game,
            mod_id,
            id,
        }
    }

    fn path(&self) -> String {
        format!(
            "/games/{}/mods/{}/files/{}",
            self.game, self.mod_id, self.id
        )
    }

    pub fn get(&self) -> Future<File> {
        self.modio.get(&self.path())
    }

    pub fn edit(&self, options: &EditFileOptions) -> Future<File> {
        let msg = match serde_urlencoded::to_string(&options) {
            Ok(data) => data,
            Err(err) => return Box::new(future::err(err.into())),
        };
        self.modio.put(&self.path(), msg)
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

#[derive(Clone, Debug, Default)]
pub struct AddFileOptions {
    filedata: PathBuf,
    version: Option<String>,
    changelog: Option<String>,
    active: Option<bool>,
    filehash: Option<String>,
    metadata_blob: Option<String>,
}

impl AddFileOptions {
    pub fn builder<P: AsRef<Path>>(filedata: P) -> AddFileOptionsBuilder {
        AddFileOptionsBuilder::new(filedata)
    }
}

impl MultipartForm for AddFileOptions {
    fn to_form(&self) -> Result<multipart::Form, Error> {
        let mut form = multipart::Form::default();

        match form.add_file("filedata", self.filedata.clone()) {
            Ok(_) => {}
            Err(err) => return Err(err.into()),
        };
        if let Some(ref version) = self.version {
            form.add_text("version", version.clone());
        }
        if let Some(ref changelog) = self.changelog {
            form.add_text("changelog", changelog.clone());
        }
        if let Some(active) = self.active {
            form.add_text("active", active.to_string());
        }
        if let Some(ref filehash) = self.filehash {
            form.add_text("filehash", filehash.clone());
        }
        if let Some(ref metadata_blob) = self.metadata_blob {
            form.add_text("metadata_blob", metadata_blob.clone());
        }
        Ok(form)
    }
}

pub struct AddFileOptionsBuilder(AddFileOptions);

impl AddFileOptionsBuilder {
    pub fn new<P: AsRef<Path>>(filedata: P) -> Self {
        AddFileOptionsBuilder(AddFileOptions {
            filedata: filedata.as_ref().to_path_buf(),
            ..Default::default()
        })
    }

    pub fn version<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.0.version = Some(value.into());
        self
    }

    pub fn changelog<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.0.changelog = Some(value.into());
        self
    }

    pub fn active(&mut self, value: bool) -> &mut Self {
        self.0.active = Some(value);
        self
    }

    pub fn filehash<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.0.filehash = Some(value.into());
        self
    }

    pub fn metadata_blob<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.0.metadata_blob = Some(value.into());
        self
    }

    pub fn build(&self) -> AddFileOptions {
        AddFileOptions {
            filedata: self.0.filedata.clone(),
            version: self.0.version.clone(),
            changelog: self.0.changelog.clone(),
            active: self.0.active,
            filehash: self.0.filehash.clone(),
            metadata_blob: self.0.metadata_blob.clone(),
        }
    }
}

#[derive(Default, Serialize)]
pub struct EditFileOptions {
    version: Option<String>,
    changelog: Option<String>,
    active: Option<bool>,
    metadata_blob: Option<String>,
}

impl EditFileOptions {
    pub fn builder() -> EditFileOptionsBuilder {
        EditFileOptionsBuilder::new()
    }
}

pub struct EditFileOptionsBuilder(EditFileOptions);

impl EditFileOptionsBuilder {
    pub fn new() -> Self {
        EditFileOptionsBuilder(EditFileOptions {
            ..Default::default()
        })
    }

    pub fn version<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.0.version = Some(value.into());
        self
    }

    pub fn changelog<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.0.changelog = Some(value.into());
        self
    }

    pub fn active(&mut self, value: bool) -> &mut Self {
        self.0.active = Some(value);
        self
    }

    pub fn metadata_blob<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.0.metadata_blob = Some(value.into());
        self
    }

    pub fn build(&self) -> EditFileOptions {
        EditFileOptions {
            version: self.0.version.clone(),
            changelog: self.0.changelog.clone(),
            active: self.0.active.clone(),
            metadata_blob: self.0.metadata_blob.clone(),
        }
    }
}
