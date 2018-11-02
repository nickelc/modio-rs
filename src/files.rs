//! Modfile interface

use std::path::Path;

use hyper::client::connect::Connect;
use hyper::Body;
use url::form_urlencoded;

use mime::APPLICATION_OCTET_STREAM;
use tokio_io::AsyncRead;

use multipart::{FileSource, FileStream, MultipartForm};
use Future;
use Modio;
use ModioListResponse;
use QueryParams;

pub use types::mods::{Download, File, FileHash};

/// Interface for the modfiles the authenticated user uploaded.
pub struct MyFiles<C>
where
    C: Clone + Connect + 'static,
{
    modio: Modio<C>,
}

impl<C: Clone + Connect + 'static> MyFiles<C> {
    pub(crate) fn new(modio: Modio<C>) -> Self {
        Self { modio }
    }

    /// Return all modfiles the authenticated user uploaded.
    pub fn list(&self, options: &FileListOptions) -> Future<ModioListResponse<File>> {
        let mut uri = vec!["/me/files".to_owned()];
        let query = options.to_query_params();
        if !query.is_empty() {
            uri.push(query);
        }
        self.modio.get(&uri.join("?"))
    }
}

/// Interface for the modfiles of a mod.
pub struct Files<C>
where
    C: Clone + Connect + 'static,
{
    modio: Modio<C>,
    game: u32,
    mod_id: u32,
}

impl<C: Clone + Connect + 'static> Files<C> {
    pub(crate) fn new(modio: Modio<C>, game: u32, mod_id: u32) -> Self {
        Self {
            modio,
            game,
            mod_id,
        }
    }

    fn path(&self, more: &str) -> String {
        format!("/games/{}/mods/{}/files{}", self.game, self.mod_id, more)
    }

    /// Return all files that are published for a mod this `Files` refers to.
    pub fn list(&self, options: &FileListOptions) -> Future<ModioListResponse<File>> {
        let mut uri = vec![self.path("")];
        let query = options.to_query_params();
        if !query.is_empty() {
            uri.push(query);
        }
        self.modio.get(&uri.join("?"))
    }

    /// Return a reference to a file.
    pub fn get(&self, id: u32) -> FileRef<C> {
        FileRef::new(self.modio.clone(), self.game, self.mod_id, id)
    }

    /// Add a file for a mod that this `Files` refers to.
    pub fn add(&self, options: AddFileOptions) -> Future<File> {
        self.modio.post_form(&self.path(""), options)
    }
}

/// Reference interface of a modfile.
pub struct FileRef<C>
where
    C: Clone + Connect + 'static,
{
    modio: Modio<C>,
    game: u32,
    mod_id: u32,
    id: u32,
}

impl<C: Clone + Connect + 'static> FileRef<C> {
    pub(crate) fn new(modio: Modio<C>, game: u32, mod_id: u32, id: u32) -> Self {
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

    /// Get a reference to the Modio modfile object that this `FileRef` refers to.
    pub fn get(&self) -> Future<File> {
        self.modio.get(&self.path())
    }

    /// Edit details of a modfile.
    pub fn edit(&self, options: &EditFileOptions) -> Future<File> {
        let params = options.to_query_params();
        self.modio.put(&self.path(), params)
    }

    /// Delete a modfile.
    pub fn delete(&self) -> Future<()> {
        self.modio.delete(&self.path(), Body::empty())
    }
}

filter_options!{
    /// Options used to filter modfile listings
    ///
    /// # Filter parameters
    /// - _q
    /// - id
    /// - mod_id
    /// - date_added
    /// - date_scanned
    /// - virus_status
    /// - virus_positive
    /// - filesize
    /// - filehash
    /// - filename
    /// - version
    /// - changelog
    ///
    /// # Sorting
    /// - id
    /// - mod_id
    /// - date_added
    /// - version
    ///
    /// See [modio docs](https://docs.mod.io/#get-all-modfiles) for more informations.
    ///
    /// By default this returns up to `100` items. You can limit the result using `limit` and
    /// `offset`.
    /// # Example
    /// ```
    /// use modio::filter::{Order, Operator};
    /// use modio::files::FileListOptions;
    ///
    /// let mut opts = FileListOptions::new();
    /// opts.id(Operator::In, vec![1, 2]);
    /// opts.sort_by(FileListOptions::ID, Order::Desc);
    /// ```
    #[derive(Debug)]
    pub struct FileListOptions {
        Filters
        - id = "id";
        - mod_id = "mod_id";
        - date_added = "date_added";
        - date_scanned = "date_scanned";
        - virus_status = "virus_status";
        - virus_positive = "virus_positive";
        - filesize = "filesize";
        - filehash = "filehash";
        - filename = "filename";
        - version = "version";
        - changelog = "changelog";

        Sort
        - ID = "id";
        - MOD_ID = "mod_id";
        - DATE_ADDED = "date_added";
        - VERSION = "version";
    }
}

pub struct AddFileOptions {
    source: FileSource,
    version: Option<String>,
    changelog: Option<String>,
    active: Option<bool>,
    filehash: Option<String>,
    metadata_blob: Option<String>,
}

impl AddFileOptions {
    pub fn with_read<R, S>(inner: R, filename: S) -> AddFileOptionsBuilder
    where
        R: AsyncRead + 'static + Send + Sync,
        S: Into<String>,
    {
        AddFileOptionsBuilder::new(FileSource {
            inner: FileStream::new(inner),
            filename: filename.into(),
            mime: APPLICATION_OCTET_STREAM,
        })
    }

    pub fn with_file<P: AsRef<Path>>(file: P) -> AddFileOptionsBuilder {
        let file = file.as_ref();
        let filename = file
            .file_name()
            .and_then(|n| n.to_str())
            .map_or_else(String::new, |n| n.to_string());

        Self::with_file_name(file, filename)
    }

    pub fn with_file_name<P, S>(file: P, filename: S) -> AddFileOptionsBuilder
    where
        P: AsRef<Path>,
        S: Into<String>,
    {
        let file = file.as_ref();

        AddFileOptionsBuilder::new(FileSource {
            inner: FileStream::open(file),
            filename: filename.into(),
            mime: APPLICATION_OCTET_STREAM,
        })
    }
}

#[doc(hidden)]
impl From<AddFileOptions> for MultipartForm {
    fn from(opts: AddFileOptions) -> MultipartForm {
        let mut mpart = MultipartForm::default();
        if let Some(version) = opts.version {
            mpart.add_field("version", &version);
        }
        if let Some(changelog) = opts.changelog {
            mpart.add_field("changelog", &changelog);
        }
        if let Some(active) = opts.active {
            mpart.add_field("active", &active.to_string());
        }
        if let Some(filehash) = opts.filehash {
            mpart.add_field("filehash", &filehash);
        }
        if let Some(metadata_blob) = opts.metadata_blob {
            mpart.add_field("metadata_blob", &metadata_blob);
        }
        mpart.add_stream(
            "filedata",
            &opts.source.filename,
            &opts.source.mime.to_string(),
            opts.source.inner,
        );
        mpart
    }
}

pub struct AddFileOptionsBuilder(AddFileOptions);

impl AddFileOptionsBuilder {
    fn new(source: FileSource) -> Self {
        AddFileOptionsBuilder(AddFileOptions {
            source,
            version: None,
            changelog: None,
            active: None,
            filehash: None,
            metadata_blob: None,
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

    pub fn build(self) -> AddFileOptions {
        AddFileOptions {
            source: self.0.source,
            version: self.0.version,
            changelog: self.0.changelog,
            active: self.0.active,
            filehash: self.0.filehash,
            metadata_blob: self.0.metadata_blob,
        }
    }
}

#[derive(Default)]
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

impl QueryParams for EditFileOptions {
    fn to_query_params(&self) -> String {
        form_urlencoded::Serializer::new(String::new())
            .extend_pairs(self.version.iter().map(|v| ("version", v)))
            .extend_pairs(self.changelog.iter().map(|c| ("changelog", c)))
            .extend_pairs(self.active.iter().map(|a| ("active", a.to_string())))
            .extend_pairs(self.metadata_blob.iter().map(|m| ("metadata_blob", m)))
            .finish()
    }
}

#[derive(Default)]
pub struct EditFileOptionsBuilder(EditFileOptions);

impl EditFileOptionsBuilder {
    pub fn new() -> Self {
        Default::default()
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
            active: self.0.active,
            metadata_blob: self.0.metadata_blob.clone(),
        }
    }
}
