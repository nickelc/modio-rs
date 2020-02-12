//! Mods Interface
use std::ffi::OsStr;
use std::path::Path;

use mime::{APPLICATION_OCTET_STREAM, IMAGE_STAR};
use url::{form_urlencoded, Url};

use crate::error::Kind;
use crate::files::{FileRef, Files};
use crate::metadata::Metadata;
use crate::multipart::FileSource;
use crate::prelude::*;
use crate::teams::Members;
use crate::Comments;

pub use crate::types::mods::{
    Dependency, Event, EventType, Image, MaturityOption, Media, MetadataMap, Mod, Popularity,
    Ratings, Statistics, Tag, Visibility,
};
pub use crate::types::Logo;
pub use crate::types::Status;

/// Interface for mods of a game.
pub struct Mods {
    modio: Modio,
    game: u32,
}

impl Mods {
    pub(crate) fn new(modio: Modio, game: u32) -> Self {
        Self { modio, game }
    }

    /// Returns a `Query` interface to retrieve mods.
    ///
    /// See [Filters and sorting](filters/index.html).
    pub fn search(&self, filter: Filter) -> Query<Mod> {
        let route = Route::GetMods { game_id: self.game };
        Query::new(self.modio.clone(), route, filter)
    }

    /// Return a reference to a mod.
    pub fn get(&self, id: u32) -> ModRef {
        ModRef::new(self.modio.clone(), self.game, id)
    }

    /// Add a mod and return the newly created Modio mod object. [required: token]
    #[allow(clippy::should_implement_trait)]
    pub async fn add(self, options: AddModOptions) -> Result<Mod> {
        let route = Route::AddMod { game_id: self.game };
        self.modio
            .request(route)
            .body(Form::from(options))
            .send()
            .await
    }

    /// Returns a `Query` interface to retrieve the statistics for all mods of a game.
    ///
    /// See [Filters and sorting](filters/stats/index.html).
    pub fn statistics(self, filter: Filter) -> Query<Statistics> {
        let route = Route::GetAllModStats { game_id: self.game };
        Query::new(self.modio, route, filter)
    }

    /// Returns a `Query` interface to retrieve the event log of all mods of the game sorted by
    /// latest event first.
    ///
    /// See [Filters and sorting](filters/events/index.html).
    pub fn events(self, filter: Filter) -> Query<Event> {
        let route = Route::GetAllModEvents { game_id: self.game };
        Query::new(self.modio, route, filter)
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

    /// Get a reference to the Modio mod object that this `ModRef` refers to.
    pub async fn get(self) -> Result<Mod> {
        let route = Route::GetMod {
            game_id: self.game,
            mod_id: self.id,
        };
        self.modio.request(route).send().await
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
    pub fn tags(&self) -> Tags {
        Tags::new(self.modio.clone(), self.game, self.id)
    }

    /// Return a reference to an interface that provides access to the comments of a mod.
    pub fn comments(&self) -> Comments {
        Comments::new(self.modio.clone(), self.game, self.id)
    }

    /// Return a reference to an interface to manage the dependencies of a mod.
    pub fn dependencies(&self) -> Dependencies {
        Dependencies::new(self.modio.clone(), self.game, self.id)
    }

    /// Return the statistics for a mod.
    pub async fn statistics(self) -> Result<Statistics> {
        let route = Route::GetModStats {
            game_id: self.game,
            mod_id: self.id,
        };
        self.modio.request(route).send().await
    }

    /// Returns a `Query` interface to retrieve the event log for a mod sorted by latest event first.
    ///
    /// See [Filters and sorting](filters/events/index.html).
    pub fn events(self, filter: Filter) -> Query<Event> {
        let route = Route::GetModEvents {
            game_id: self.game,
            mod_id: self.id,
        };
        Query::new(self.modio, route, filter)
    }

    /// Return a reference to an interface to manage team members of a mod.
    pub fn members(&self) -> Members {
        Members::new(self.modio.clone(), self.game, self.id)
    }

    /// Edit details for a mod. [required: token]
    pub async fn edit(self, options: EditModOptions) -> Result<Editing<Mod>> {
        let route = Route::EditMod {
            game_id: self.game,
            mod_id: self.id,
        };
        self.modio
            .request(route)
            .body(options.to_query_string())
            .send()
            .await
    }

    /// Delete a mod. [required: token]
    pub async fn delete(self) -> Result<()> {
        let route = Route::DeleteMod {
            game_id: self.game,
            mod_id: self.id,
        };
        self.modio.request(route).send().await
    }

    /// Add new media to a mod. [required: token]
    pub async fn add_media(self, options: AddMediaOptions) -> Result<()> {
        let route = Route::AddModMedia {
            game_id: self.game,
            mod_id: self.id,
        };
        self.modio
            .request(route)
            .body(Form::from(options))
            .send::<ModioMessage>()
            .await?;

        Ok(())
    }

    /// Delete media from a mod. [required: token]
    pub async fn delete_media(self, options: DeleteMediaOptions) -> Result<Deletion> {
        let route = Route::DeleteModMedia {
            game_id: self.game,
            mod_id: self.id,
        };
        self.modio
            .request(route)
            .body(options.to_query_string())
            .send()
            .await
    }

    /// Submit a positive or negative rating for a mod. [required: token]
    pub async fn rate(self, rating: Rating) -> Result<()> {
        let route = Route::RateMod {
            game_id: self.game,
            mod_id: self.id,
        };
        self.modio
            .request(route)
            .body(rating.to_query_string())
            .send::<ModioMessage>()
            .await
            .map(|_| ())
            .or_else(|err| match err.kind() {
                Kind::Status(StatusCode::BAD_REQUEST) => Ok(()),
                _ => Err(err),
            })
    }

    /// Subscribe the authenticated user to a mod. [required: token]
    pub async fn subscribe(self) -> Result<()> {
        let route = Route::Subscribe {
            game_id: self.game,
            mod_id: self.id,
        };
        self.modio
            .request(route)
            .send::<Mod>()
            .await
            .map(|_| ())
            .or_else(|err| match err.kind() {
                Kind::Status(StatusCode::BAD_REQUEST) => Ok(()),
                _ => Err(err),
            })
    }

    /// Unsubscribe the authenticated user from a mod. [required: token]
    pub async fn unsubscribe(self) -> Result<()> {
        let route = Route::Unsubscribe {
            game_id: self.game,
            mod_id: self.id,
        };
        self.modio
            .request(route)
            .send()
            .await
            .or_else(|err| match err.kind() {
                Kind::Status(StatusCode::BAD_REQUEST) => Ok(()),
                _ => Err(err),
            })
    }
}

/// Interface for dependencies.
pub struct Dependencies {
    modio: Modio,
    game_id: u32,
    mod_id: u32,
}

impl Dependencies {
    fn new(modio: Modio, game_id: u32, mod_id: u32) -> Self {
        Self {
            modio,
            game_id,
            mod_id,
        }
    }

    /// List mod dependencies.
    pub async fn list(self) -> Result<Vec<Dependency>> {
        let route = Route::GetModDependencies {
            game_id: self.game_id,
            mod_id: self.mod_id,
        };
        Query::new(self.modio, route, Default::default())
            .collect()
            .await
    }

    /// Provides a stream over all mod dependencies.
    pub fn iter(self) -> impl Stream<Item = Result<Dependency>> {
        let route = Route::GetModDependencies {
            game_id: self.game_id,
            mod_id: self.mod_id,
        };
        Query::new(self.modio, route, Default::default()).iter()
    }

    /// Add mod dependencies. [required: token]
    #[allow(clippy::should_implement_trait)]
    pub async fn add(self, options: EditDependenciesOptions) -> Result<()> {
        let route = Route::AddModDepencencies {
            game_id: self.game_id,
            mod_id: self.mod_id,
        };
        self.modio
            .request(route)
            .body(options.to_query_string())
            .send::<ModioMessage>()
            .await?;
        Ok(())
    }

    /// Delete mod dependencies. [required: token]
    pub async fn delete(self, options: EditDependenciesOptions) -> Result<Deletion> {
        let route = Route::DeleteModDependencies {
            game_id: self.game_id,
            mod_id: self.mod_id,
        };
        self.modio
            .request(route)
            .body(options.to_query_string())
            .send()
            .await
    }
}

/// Interface for tags.
pub struct Tags {
    modio: Modio,
    game_id: u32,
    mod_id: u32,
}

impl Tags {
    fn new(modio: Modio, game_id: u32, mod_id: u32) -> Self {
        Self {
            modio,
            game_id,
            mod_id,
        }
    }

    /// List all mod tags.
    pub async fn list(self) -> Result<Vec<Tag>> {
        let route = Route::GetModTags {
            game_id: self.game_id,
            mod_id: self.mod_id,
        };
        Query::new(self.modio, route, Default::default())
            .collect()
            .await
    }

    /// Provides a stream over all mod tags.
    pub fn iter(self) -> impl Stream<Item = Result<Tag>> {
        let route = Route::GetModTags {
            game_id: self.game_id,
            mod_id: self.mod_id,
        };
        Query::new(self.modio, route, Default::default()).iter()
    }

    /// Add mod tags. [required: token]
    #[allow(clippy::should_implement_trait)]
    pub async fn add(self, options: EditTagsOptions) -> Result<()> {
        let route = Route::AddModTags {
            game_id: self.game_id,
            mod_id: self.mod_id,
        };
        self.modio
            .request(route)
            .body(options.to_query_string())
            .send::<ModioMessage>()
            .await?;
        Ok(())
    }

    /// Delete mod tags. [required: token]
    pub async fn delete(self, options: EditTagsOptions) -> Result<Deletion> {
        let route = Route::DeleteModTags {
            game_id: self.game_id,
            mod_id: self.mod_id,
        };
        self.modio
            .request(route)
            .body(options.to_query_string())
            .send()
            .await
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

        let logo = FileSource::new_from_file(logo, filename, IMAGE_STAR);

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
            logo: Some(FileSource::new_from_file(logo, filename, IMAGE_STAR)),
            ..self
        }
    }

    pub fn images_zip<P: AsRef<Path>>(self, images: P) -> Self {
        Self {
            images_zip: Some(FileSource::new_from_file(
                images,
                "images.zip".into(),
                APPLICATION_OCTET_STREAM,
            )),
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

                        FileSource::new_from_file(file, filename, IMAGE_STAR)
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
