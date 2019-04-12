//! Mods Interface
use std::ffi::OsStr;
use std::path::Path;

use mime::{APPLICATION_OCTET_STREAM, IMAGE_STAR};
use url::{form_urlencoded, Url};

use crate::error::ErrorKind;
use crate::files::{FileRef, Files};
use crate::metadata::Metadata;
use crate::multipart::{FileSource, FileStream};
use crate::prelude::*;
use crate::teams::Members;
use crate::Comments;

pub use crate::types::mods::{
    Dependency, Event, EventType, Image, MaturityOption, Media, MetadataMap, Mod, Popularity,
    Ratings, Statistics, Tag, Visibility,
};
pub use crate::types::Logo;
pub use crate::types::Status;

/// Interface for mods the authenticated user added or is team member of.
pub struct MyMods {
    modio: Modio,
}

impl MyMods {
    pub(crate) fn new(modio: Modio) -> Self {
        Self { modio }
    }

    /// List all mods the authenticated user added or is team member of. [required: token]
    ///
    /// See [Filters and sorting](filters/index.html).
    pub fn list(&self, filter: &Filter) -> Future<List<Mod>> {
        token_required!(self.modio);
        let mut uri = vec!["/me/mods".to_owned()];
        let query = filter.to_query_string();
        if !query.is_empty() {
            uri.push(query);
        }
        self.modio.get(&uri.join("?"))
    }

    /// Provides a stream over mods the authenticated user added or is team member of. [required:
    /// token]
    ///
    /// See [Filters and sorting](filters/index.html).
    pub fn iter(&self, filter: &Filter) -> Stream<Mod> {
        token_required!(s self.modio);
        let mut uri = vec!["/me/mods".to_owned()];
        let query = filter.to_query_string();
        if !query.is_empty() {
            uri.push(query);
        }
        self.modio.stream(&uri.join("?"))
    }
}

/// Interface for mods of a game.
pub struct Mods {
    modio: Modio,
    game: u32,
}

impl Mods where {
    pub(crate) fn new(modio: Modio, game: u32) -> Self {
        Self { modio, game }
    }

    fn path(&self, more: &str) -> String {
        format!("/games/{}/mods{}", self.game, more)
    }

    /// Return a reference to a mod.
    pub fn get(&self, id: u32) -> ModRef {
        ModRef::new(self.modio.clone(), self.game, id)
    }

    /// List all mods.
    ///
    /// See [Filters and sorting](filters/index.html).
    pub fn list(&self, filter: &Filter) -> Future<List<Mod>> {
        let mut uri = vec![self.path("")];
        let query = filter.to_query_string();
        if !query.is_empty() {
            uri.push(query);
        }
        self.modio.get(&uri.join("?"))
    }

    /// Provides a stream over all mods of the game.
    ///
    /// See [Filters and sorting](filters/index.html).
    pub fn iter(&self, filter: &Filter) -> Stream<Mod> {
        let mut uri = vec![self.path("")];
        let query = filter.to_query_string();
        if !query.is_empty() {
            uri.push(query);
        }
        self.modio.stream(&uri.join("?"))
    }

    /// Add a mod and return the newly created Modio mod object. [required: token]
    pub fn add(&self, options: AddModOptions) -> Future<Mod> {
        token_required!(self.modio);
        self.modio.post_form(&self.path(""), options)
    }

    /// Provides a stream over the statistics for all mods of a game.
    ///
    /// See [Filters and sorting](filters/stats/index.html).
    pub fn statistics(&self, filter: &Filter) -> Stream<Statistics> {
        let mut uri = vec![self.path("/stats")];
        let query = filter.to_query_string();
        if !query.is_empty() {
            uri.push(query);
        }
        self.modio.stream(&uri.join("?"))
    }

    /// Provides a stream over the event log for all mods of a game sorted by latest event first.
    ///
    /// See [Filters and sorting](filters/events/index.html).
    pub fn events(&self, filter: &Filter) -> Stream<Event> {
        let mut uri = vec![self.path("/events")];
        let query = filter.to_query_string();
        if !query.is_empty() {
            uri.push(query);
        }
        self.modio.stream(&uri.join("?"))
    }
}

/// Reference interface of a mod.
pub struct ModRef {
    modio: Modio,
    game: u32,
    id: u32,
}

impl ModRef {
    pub(crate) fn new(modio: Modio, game: u32, id: u32) -> Self {
        Self { modio, game, id }
    }

    fn path(&self, more: &str) -> String {
        format!("/games/{}/mods/{}{}", self.game, self.id, more)
    }

    /// Get a reference to the Modio mod object that this `ModRef` refers to.
    pub fn get(&self) -> Future<Mod> {
        self.modio.get(&self.path(""))
    }

    /// Return a reference to an interface that provides access to the files of a mod.
    pub fn files(&self) -> Files {
        Files::new(self.modio.clone(), self.game, self.id)
    }

    /// Return a reference to a file of a mod.
    pub fn file(&self, id: u32) -> FileRef {
        FileRef::new(self.modio.clone(), self.game, self.id, id)
    }

    /// Return a reference to an interface to manage metadata key value pairs of a mod.
    pub fn metadata(&self) -> Metadata {
        Metadata::new(self.modio.clone(), self.game, self.id)
    }

    /// Return a reference to an interface to manage the tags of a mod.
    pub fn tags(&self) -> Endpoint<Tag> {
        Endpoint::new(self.modio.clone(), self.path("/tags"))
    }

    /// Return a reference to an interface that provides access to the comments of a mod.
    pub fn comments(&self) -> Comments {
        Comments::new(self.modio.clone(), self.game, self.id)
    }

    /// Return a reference to an interface to manage the dependencies of a mod.
    pub fn dependencies(&self) -> Endpoint<Dependency> {
        Endpoint::new(self.modio.clone(), self.path("/dependencies"))
    }

    /// Return the statistics for a mod.
    pub fn statistics(&self) -> Future<Statistics> {
        self.modio.get(&self.path("/stats"))
    }

    /// Provides a stream over the event log for a mod sorted by latest event first.
    ///
    /// See [Filters and sorting](filters/events/index.html).
    pub fn events(&self, filter: &Filter) -> Stream<Event> {
        let mut uri = vec![self.path("/events")];
        let query = filter.to_query_string();
        if !query.is_empty() {
            uri.push(query);
        }
        self.modio.stream(&uri.join("?"))
    }

    /// Return a reference to an interface to manage team members of a mod.
    pub fn members(&self) -> Members {
        Members::new(self.modio.clone(), self.game, self.id)
    }

    /// Edit details for a mod. [required: token]
    pub fn edit(&self, options: &EditModOptions) -> Future<ModioResult<Mod>> {
        token_required!(self.modio);
        let params = options.to_query_string();
        self.modio.put(&self.path(""), params)
    }

    /// Add new media to a mod. [required: token]
    pub fn add_media(&self, options: AddMediaOptions) -> Future<()> {
        token_required!(self.modio);
        Box::new(
            self.modio
                .post_form::<ModioMessage, _>(&self.path("/media"), options)
                .map(|_| ()),
        )
    }

    /// Delete media from a mod. [required: token]
    pub fn delete_media(&self, options: &DeleteMediaOptions) -> Future<()> {
        token_required!(self.modio);
        self.modio
            .delete(&self.path("/media"), options.to_query_string())
    }

    /// Submit a positive or negative rating for a mod. [required: token]
    pub fn rate(&self, rating: Rating) -> Future<()> {
        token_required!(self.modio);
        let params = rating.to_query_string();
        Box::new(
            self.modio
                .post::<ModioMessage, _>(&self.path("/ratings"), params)
                .map(|_| ())
                .or_else(|err| match err.kind() {
                    ErrorKind::Fault {
                        code: StatusCode::BAD_REQUEST,
                        ..
                    } => Ok(()),
                    _ => Err(err),
                }),
        )
    }

    /// Subscribe the authenticated user to a mod. [required: token]
    pub fn subscribe(&self) -> Future<()> {
        token_required!(self.modio);
        Box::new(
            self.modio
                .post::<Mod, _>(&self.path("/subscribe"), RequestBody::Empty)
                .map(|_| ())
                .or_else(|err| match err.kind() {
                    ErrorKind::Fault {
                        code: StatusCode::BAD_REQUEST,
                        ..
                    } => Ok(()),
                    _ => Err(err),
                }),
        )
    }

    /// Unsubscribe the authenticated user from a mod. [required: token]
    pub fn unsubscribe(&self) -> Future<()> {
        token_required!(self.modio);
        Box::new(
            self.modio
                .delete(&self.path("/subscribe"), RequestBody::Empty)
                .or_else(|err| match err.kind() {
                    ErrorKind::Fault {
                        code: StatusCode::BAD_REQUEST,
                        ..
                    } => Ok(()),
                    _ => Err(err),
                }),
        )
    }
}

/// Mod filters & sorting
///
/// # Filters
/// - Fulltext
/// - Id
/// - GameId
/// - Status
/// - Visible
/// - SubmittedBy
/// - DateAdded
/// - DateUpdated
/// - DateLive
/// - MaturityOption
/// - Name
/// - NameId
/// - Summary
/// - Description
/// - Homepage
/// - Modfile
/// - MetadataBlob
/// - MetadataKVP
/// - Tags
///
/// # Sorting
/// - Id
/// - Name
/// - Downloads
/// - Popular
/// - Ratings
/// - Subscribers
///
/// See the [modio docs](https://docs.mod.io/#get-all-mods) for more information.
///
/// By default this returns up to `100` items. you can limit the result by using `limit` and
/// `offset`.
///
/// # Example
/// ```
/// use modio::filter::prelude::*;
/// use modio::mods::filters::Id;
/// use modio::mods::filters::GameId;
/// use modio::mods::filters::Tags;
///
/// let filter = Id::_in(vec![1, 2]).order_by(Id::desc());
///
/// let filter = GameId::eq(6).and(Tags::eq("foobar")).limit(10);
/// ```
#[rustfmt::skip]
pub mod filters {
    #[doc(inline)]
    pub use crate::filter::prelude::Fulltext;
    #[doc(inline)]
    pub use crate::filter::prelude::Id;
    #[doc(inline)]
    pub use crate::filter::prelude::Name;
    #[doc(inline)]
    pub use crate::filter::prelude::NameId;
    #[doc(inline)]
    pub use crate::filter::prelude::Status;
    #[doc(inline)]
    pub use crate::filter::prelude::DateAdded;
    #[doc(inline)]
    pub use crate::filter::prelude::DateUpdated;
    #[doc(inline)]
    pub use crate::filter::prelude::DateLive;
    #[doc(inline)]
    pub use crate::filter::prelude::SubmittedBy;

    filter!(GameId, GAME_ID, "game_id", Eq, NotEq, In, Cmp, OrderBy);
    filter!(Visible, VISIBLE, "visible", Eq);
    filter!(MaturityOption, MATURITY_OPTION, "maturity_option", Eq, Cmp, Bit);
    filter!(Summary, SUMMARY, "summary", Like);
    filter!(Description, DESCRIPTION, "description", Like);
    filter!(Homepage, HOMEPAGE, "homepage_url", Eq, NotEq, Like, In);
    filter!(Modfile, MODFILE, "modfile", Eq, NotEq, In, Cmp);
    filter!(MetadataBlob, METADATA_BLOB, "metadata_blob", Eq, NotEq, Like);
    filter!(MetadataKVP, METADATA_KVP, "metadata_kvp", Eq, NotEq, Like);
    filter!(Tags, TAGS, "tags", Eq, NotEq, Like);

    filter!(Downloads, DOWNLOADS, "downloads", OrderBy);
    filter!(Popular, POPULAR, "popular", OrderBy);
    filter!(Ratings, RATINGS, "ratings", OrderBy);
    filter!(Subscribers, SUBSCRIBERS, "subscribers", OrderBy);

    /// Mod event filters and sorting.
    ///
    /// # Filters
    /// - Id
    /// - ModId
    /// - UserId
    /// - DateAdded
    /// - EventType
    ///
    /// # Sorting
    /// - Id
    /// - DateAdded
    ///
    /// See the [modio docs](https://docs.mod.io/#events) for more information.
    ///
    /// By default this returns up to `100` items. You can limit the result by using `limit` and
    /// `offset`.
    ///
    /// # Example
    /// ```
    /// use modio::filter::prelude::*;
    /// use modio::mods::filters::events::EventType as Filter;
    /// use modio::mods::EventType;
    ///
    /// let filter = Id::gt(1024).and(Filter::eq(EventType::ModfileChanged));
    /// ```
    pub mod events {
        #[doc(inline)]
        pub use crate::filter::prelude::Id;
        #[doc(inline)]
        pub use crate::filter::prelude::ModId;
        #[doc(inline)]
        pub use crate::filter::prelude::DateAdded;

        filter!(UserId, USER_ID, "user_id", Eq, NotEq, In, Cmp, OrderBy);
        filter!(EventType, EVENT_TYPE, "event_type", Eq, NotEq, In, OrderBy);
    }

    /// Mod statistics filters & sorting
    ///
    /// # Filters
    /// - ModId
    /// - Popularity
    /// - Downloads
    /// - Subscribers
    /// - RatingsPositive
    /// - RatingsNegative
    ///
    /// # Sorting
    /// - ModId
    /// - Popularity
    /// - Downloads
    /// - Subscribers
    /// - RatingsPositive
    /// - RatingsNegative
    ///
    /// # Example
    /// ```
    /// use modio::filter::prelude::*;
    /// use modio::mods::filters::stats::ModId;
    /// use modio::mods::filters::stats::Popularity;
    ///
    /// let filter = ModId::_in(vec![1, 2]).order_by(Popularity::desc());
    /// ```
    pub mod stats {
        #[doc(inline)]
        pub use crate::filter::prelude::ModId;

        filter!(Popularity, POPULARITY, "popularity_rank_position", Eq, NotEq, In, Cmp, OrderBy);
        filter!(Downloads, DOWNLOADS, "downloads_total", Eq, NotEq, In, Cmp, OrderBy);
        filter!(Subscribers, SUBSCRIBERS, "subscribers_total", Eq, NotEq, In, Cmp, OrderBy);
        filter!(RatingsPositive, RATINGS_POSITIVE, "ratings_positive", Eq, NotEq, In, Cmp, OrderBy);
        filter!(RatingsNegative, RATINGS_NEGATIVE, "ratings_negative", Eq, NotEq, In, Cmp, OrderBy);
    }
}

#[derive(Clone, Copy)]
pub enum Rating {
    Positive,
    Negative,
}

impl QueryString for Rating {
    fn to_query_string(&self) -> String {
        format!(
            "rating={}",
            match *self {
                Rating::Negative => -1,
                Rating::Positive => 1,
            }
        )
    }
}

pub struct AddModOptions {
    visible: Option<Visibility>,
    logo: FileSource,
    name: String,
    name_id: Option<String>,
    summary: String,
    description: Option<String>,
    homepage_url: Option<Url>,
    stock: Option<u32>,
    maturity_option: Option<MaturityOption>,
    metadata_blob: Option<String>,
    tags: Option<Vec<String>>,
}

impl AddModOptions {
    pub fn new<T, P>(name: T, logo: P, summary: T) -> AddModOptions
    where
        T: Into<String>,
        P: AsRef<Path>,
    {
        let filename = logo
            .as_ref()
            .file_name()
            .and_then(OsStr::to_str)
            .map_or_else(String::new, ToString::to_string);

        let logo = FileSource {
            inner: FileStream::open(logo),
            filename,
            mime: IMAGE_STAR,
        };

        AddModOptions {
            name: name.into(),
            logo,
            summary: summary.into(),
            visible: None,
            name_id: None,
            description: None,
            homepage_url: None,
            stock: None,
            maturity_option: None,
            metadata_blob: None,
            tags: None,
        }
    }

    pub fn visible(self, v: bool) -> Self {
        Self {
            visible: if v {
                Some(Visibility::Public)
            } else {
                Some(Visibility::Hidden)
            },
            ..self
        }
    }

    option!(name_id);
    option!(description);
    option!(homepage_url: Url);
    option!(stock: u32);
    option!(maturity_option: MaturityOption);
    option!(metadata_blob);

    pub fn tags(self, tags: &[String]) -> Self {
        Self {
            tags: Some(tags.to_vec()),
            ..self
        }
    }
}

#[doc(hidden)]
impl From<AddModOptions> for Form {
    fn from(opts: AddModOptions) -> Form {
        let mut form = Form::new();

        form = form.text("name", opts.name).text("summary", opts.summary);

        if let Some(visible) = opts.visible {
            form = form.text("visible", visible.to_string());
        }
        if let Some(name_id) = opts.name_id {
            form = form.text("name_id", name_id);
        }
        if let Some(desc) = opts.description {
            form = form.text("description", desc);
        }
        if let Some(url) = opts.homepage_url {
            form = form.text("homepage_url", url.to_string());
        }
        if let Some(stock) = opts.stock {
            form = form.text("stock", stock.to_string());
        }
        if let Some(maturity_option) = opts.maturity_option {
            form = form.text("maturity_option", maturity_option.to_string());
        }
        if let Some(metadata_blob) = opts.metadata_blob {
            form = form.text("metadata_blob", metadata_blob);
        }
        if let Some(tags) = opts.tags {
            for tag in tags {
                form = form.text("tags[]", tag);
            }
        }
        form.part("logo", opts.logo.into())
    }
}

#[derive(Debug, Default)]
pub struct EditModOptions {
    params: std::collections::BTreeMap<&'static str, String>,
}

impl EditModOptions {
    option!(status: Status >> "status");

    pub fn visible(self, v: bool) -> Self {
        let value = if v {
            Visibility::Public
        } else {
            Visibility::Hidden
        };
        let mut params = self.params;
        params.insert("visible", value.to_string());
        Self { params }
    }

    option!(visibility: Visibility >> "visible");
    option!(name >> "name");
    option!(name_id >> "name_id");
    option!(summary >> "summary");
    option!(description >> "description");
    option!(homepage_url: Url >> "homepage_url");
    option!(stock >> "stock");
    option!(maturity_option: MaturityOption >> "maturity_option");
    option!(metadata_blob >> "metadata_blob");
}

impl QueryString for EditModOptions {
    fn to_query_string(&self) -> String {
        form_urlencoded::Serializer::new(String::new())
            .extend_pairs(&self.params)
            .finish()
    }
}

#[doc(hidden)]
#[deprecated(since = "0.4.1", note = "Use `EditDependenciesOptions`")]
pub type EditDepencenciesOptions = EditDependenciesOptions;

pub struct EditDependenciesOptions {
    dependencies: Vec<u32>,
}

impl EditDependenciesOptions {
    pub fn new(dependencies: &[u32]) -> Self {
        Self {
            dependencies: dependencies.to_vec(),
        }
    }

    pub fn one(dependency: u32) -> Self {
        Self {
            dependencies: vec![dependency],
        }
    }
}

impl AddOptions for EditDependenciesOptions {}
impl DeleteOptions for EditDependenciesOptions {}

impl QueryString for EditDependenciesOptions {
    fn to_query_string(&self) -> String {
        form_urlencoded::Serializer::new(String::new())
            .extend_pairs(
                self.dependencies
                    .iter()
                    .map(|d| ("dependencies[]", d.to_string())),
            )
            .finish()
    }
}

pub struct EditTagsOptions {
    tags: Vec<String>,
}

impl EditTagsOptions {
    pub fn new(tags: &[String]) -> Self {
        Self {
            tags: tags.to_vec(),
        }
    }
}

impl AddOptions for EditTagsOptions {}
impl DeleteOptions for EditTagsOptions {}

impl QueryString for EditTagsOptions {
    fn to_query_string(&self) -> String {
        form_urlencoded::Serializer::new(String::new())
            .extend_pairs(self.tags.iter().map(|t| ("tags[]", t)))
            .finish()
    }
}

#[derive(Default)]
pub struct AddMediaOptions {
    logo: Option<FileSource>,
    images_zip: Option<FileSource>,
    images: Option<Vec<FileSource>>,
    youtube: Option<Vec<String>>,
    sketchfab: Option<Vec<String>>,
}

impl AddMediaOptions {
    pub fn logo<P: AsRef<Path>>(self, logo: P) -> Self {
        let logo = logo.as_ref();
        let filename = logo
            .file_name()
            .and_then(OsStr::to_str)
            .map_or_else(String::new, ToString::to_string);

        Self {
            logo: Some(FileSource {
                inner: FileStream::open(logo),
                filename,
                mime: IMAGE_STAR,
            }),
            ..self
        }
    }

    pub fn images_zip<P: AsRef<Path>>(self, images: P) -> Self {
        Self {
            images_zip: Some(FileSource {
                inner: FileStream::open(images),
                filename: "images.zip".into(),
                mime: APPLICATION_OCTET_STREAM,
            }),
            ..self
        }
    }

    pub fn images<P: AsRef<Path>>(self, images: &[P]) -> Self {
        Self {
            images: Some(
                images
                    .iter()
                    .map(|p| {
                        let file = p.as_ref();
                        let filename = file
                            .file_name()
                            .and_then(OsStr::to_str)
                            .map_or_else(String::new, ToString::to_string);

                        FileSource {
                            inner: FileStream::open(file),
                            filename,
                            mime: IMAGE_STAR,
                        }
                    })
                    .collect::<Vec<_>>(),
            ),
            ..self
        }
    }

    pub fn youtube(self, urls: &[String]) -> Self {
        Self {
            youtube: Some(urls.to_vec()),
            ..self
        }
    }

    pub fn sketchfab(self, urls: &[String]) -> Self {
        Self {
            sketchfab: Some(urls.to_vec()),
            ..self
        }
    }
}

#[doc(hidden)]
impl From<AddMediaOptions> for Form {
    fn from(opts: AddMediaOptions) -> Form {
        let mut form = Form::new();
        if let Some(logo) = opts.logo {
            form = form.part("logo", logo.into());
        }
        if let Some(zip) = opts.images_zip {
            form = form.part("images", zip.into());
        }
        if let Some(images) = opts.images {
            for (i, image) in images.into_iter().enumerate() {
                form = form.part(format!("image{}", i), image.into());
            }
        }
        if let Some(youtube) = opts.youtube {
            for url in youtube {
                form = form.text("youtube[]", url);
            }
        }
        if let Some(sketchfab) = opts.sketchfab {
            for url in sketchfab {
                form = form.text("sketchfab[]", url);
            }
        }
        form
    }
}

#[derive(Default)]
pub struct DeleteMediaOptions {
    images: Option<Vec<String>>,
    youtube: Option<Vec<String>>,
    sketchfab: Option<Vec<String>>,
}

impl DeleteMediaOptions {
    pub fn images(self, images: &[String]) -> Self {
        Self {
            images: Some(images.to_vec()),
            ..self
        }
    }

    pub fn youtube(self, urls: &[String]) -> Self {
        Self {
            youtube: Some(urls.to_vec()),
            ..self
        }
    }

    pub fn sketchfab(self, urls: &[String]) -> Self {
        Self {
            sketchfab: Some(urls.to_vec()),
            ..self
        }
    }
}

impl QueryString for DeleteMediaOptions {
    fn to_query_string(&self) -> String {
        let mut ser = form_urlencoded::Serializer::new(String::new());
        if let Some(ref images) = self.images {
            ser.extend_pairs(images.iter().map(|i| ("images[]", i)));
        }
        if let Some(ref urls) = self.youtube {
            ser.extend_pairs(urls.iter().map(|u| ("youtube[]", u)));
        }
        if let Some(ref urls) = self.sketchfab {
            ser.extend_pairs(urls.iter().map(|u| ("sketchfab[]", u)));
        }
        ser.finish()
    }
}
