//! Modfile interface
use std::path::Path;

use mime::APPLICATION_OCTET_STREAM;
use tokio_io::AsyncRead;
use url::form_urlencoded;

use crate::multipart::{FileSource, FileStream};
use crate::prelude::*;

pub use crate::types::mods::{Download, File, FileHash};

/// Interface for the modfiles the authenticated user uploaded.
pub struct MyFiles {
    modio: Modio,
}

impl MyFiles {
    pub(crate) fn new(modio: Modio) -> Self {
        Self { modio }
    }

    /// Return all modfiles the authenticated user uploaded. [required: token]
    ///
    /// See [Filters and sorting](filters/index.html).
    pub fn list(&self, filter: &Filter) -> Future<List<File>> {
        token_required!(self.modio);
        let mut uri = vec!["/me/files".to_owned()];
        let query = filter.to_query_params();
        if !query.is_empty() {
            uri.push(query);
        }
        self.modio.get(&uri.join("?"))
    }

    /// Provides a stream over all modfiles the authenticated user uploaded. [required: token]
    ///
    /// See [Filters and sorting](filters/index.html).
    pub fn iter(&self, filter: &Filter) -> Stream<File> {
        token_required!(s self.modio);
        let mut uri = vec!["/me/files".to_owned()];
        let query = filter.to_query_params();
        if !query.is_empty() {
            uri.push(query);
        }
        self.modio.stream(&uri.join("?"))
    }
}

/// Interface for the modfiles of a mod.
pub struct Files {
    modio: Modio,
    game: u32,
    mod_id: u32,
}

impl Files {
    pub(crate) fn new(modio: Modio, game: u32, mod_id: u32) -> Self {
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
    ///
    /// See [Filters and sorting](filters/index.html).
    pub fn list(&self, filter: &Filter) -> Future<List<File>> {
        let mut uri = vec![self.path("")];
        let query = filter.to_query_params();
        if !query.is_empty() {
            uri.push(query);
        }
        self.modio.get(&uri.join("?"))
    }

    /// Provides a stream over all files that are published for a mod this `Files` refers to.
    ///
    /// See [Filters and sorting](filters/index.html).
    pub fn iter(&self, filter: &Filter) -> Stream<File> {
        let mut uri = vec![self.path("")];
        let query = filter.to_query_params();
        if !query.is_empty() {
            uri.push(query);
        }
        self.modio.stream(&uri.join("?"))
    }

    /// Return a reference to a file.
    pub fn get(&self, id: u32) -> FileRef {
        FileRef::new(self.modio.clone(), self.game, self.mod_id, id)
    }

    /// Add a file for a mod that this `Files` refers to. [required: token]
    pub fn add(&self, options: AddFileOptions) -> Future<File> {
        token_required!(self.modio);
        self.modio.post_form(&self.path(""), options)
    }
}

/// Reference interface of a modfile.
pub struct FileRef {
    modio: Modio,
    game: u32,
    mod_id: u32,
    id: u32,
}

impl FileRef {
    pub(crate) fn new(modio: Modio, game: u32, mod_id: u32, id: u32) -> Self {
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

    /// Edit details of a modfile. [required: token]
    pub fn edit(&self, options: &EditFileOptions) -> Future<File> {
        token_required!(self.modio);
        let params = options.to_query_params();
        self.modio.put(&self.path(), params)
    }

    /// Delete a modfile. [required: token]
    pub fn delete(&self) -> Future<()> {
        token_required!(self.modio);
        self.modio.delete(&self.path(), RequestBody::Empty)
    }
}

/// Modfile filters and sorting.
///
/// # Filters
/// - Fulltext
/// - Id
/// - ModId
/// - DateAdded
/// - DateScanned
/// - VirusStatus
/// - VirusPositive
/// - Filesize
/// - Filehash
/// - Filename
/// - Version
/// - Changelog
///
/// # Sorting
/// - Id
/// - ModId
/// - DateAdded
/// - Version
///
/// See [modio docs](https://docs.mod.io/#get-all-modfiles) for more information.
///
/// By default this returns up to `100` items. You can limit the result by using `limit` and
/// `offset`.
///
/// # Example
/// ```
/// use modio::filter::prelude::*;
/// use modio::files::filters::Id;
///
/// let filter = Id::_in(vec![1, 2]).order_by(Id::desc());
/// ```
#[rustfmt::skip]
pub mod filters {
    #[doc(inline)]
    pub use crate::filter::prelude::Fulltext;
    #[doc(inline)]
    pub use crate::filter::prelude::Id;
    #[doc(inline)]
    pub use crate::filter::prelude::ModId;
    #[doc(inline)]
    pub use crate::filter::prelude::DateAdded;

    filter!(DateScanned, DATE_SCANNED, "date_scanned", Eq, NotEq, In, Cmp);
    filter!(VirusStatus, VIRUS_STATUS, "virus_status", Eq, NotEq, In, Cmp);
    filter!(VirusPositive, VIRUS_POSITIVE, "virus_positive", Eq, NotEq, In, Cmp);
    filter!(Filesize, FILESIZE, "filesize", Eq, NotEq, In, Cmp, OrderBy);
    filter!(Filehash, FILEHASH, "filehash", Eq, NotEq, In, Like);
    filter!(Filename, FILENAME, "filename", Eq, NotEq, In, Like);
    filter!(Version, VERSION, "version", Eq, NotEq, In, Like, OrderBy);
    filter!(Changelog, CHANGELOG, "changelog", Eq, NotEq, In, Like);
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
impl From<AddFileOptions> for Form {
    fn from(opts: AddFileOptions) -> Form {
        let mut form = Form::new();
        if let Some(version) = opts.version {
            form = form.text("version", version);
        }
        if let Some(changelog) = opts.changelog {
            form = form.text("changelog", changelog);
        }
        if let Some(active) = opts.active {
            form = form.text("active", active.to_string());
        }
        if let Some(filehash) = opts.filehash {
            form = form.text("filehash", filehash);
        }
        if let Some(metadata_blob) = opts.metadata_blob {
            form = form.text("metadata_blob", metadata_blob);
        }
        form.part("filedata", opts.source.into())
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
