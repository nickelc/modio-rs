//! Mods Interface

use std::path::{Path, PathBuf};

use futures::Future as StdFuture;
use hyper::client::connect::Connect;
use hyper::StatusCode;
use hyper_multipart::client::multipart;
use url::{form_urlencoded, Url};

use error::Error;
use files::{FileRef, Files};
use metadata::Metadata;
use teams::Members;
use types::Event;
use Comments;
use Endpoint;
use EventListOptions;
use Future;
use Modio;
use ModioListResponse;
use ModioMessage;
use MultipartForm;
use {AddOptions, DeleteOptions, QueryParams};

pub use types::mods::{
    Dependency, Image, Media, MetadataMap, Mod, Popularity, Ratings, Statistics, Tag,
};
pub use types::Logo;

/// Interface for mods the authenticated user added or is team member of.
pub struct MyMods<C>
where
    C: Clone + Connect + 'static,
{
    modio: Modio<C>,
}

impl<C: Clone + Connect + 'static> MyMods<C> {
    pub(crate) fn new(modio: Modio<C>) -> Self {
        Self { modio }
    }

    /// List all mods the authenticated user added or is team member of.
    pub fn list(&self, options: &ModsListOptions) -> Future<ModioListResponse<Mod>> {
        let mut uri = vec!["/me/mods".to_owned()];
        let query = options.to_query_params();
        if !query.is_empty() {
            uri.push(query);
        }
        self.modio.get::<ModioListResponse<Mod>>(&uri.join("?"))
    }
}

/// Interface for mods of a game.
pub struct Mods<C>
where
    C: Clone + Connect + 'static,
{
    modio: Modio<C>,
    game: u32,
}

impl<C> Mods<C>
where
    C: Clone + Connect + 'static,
{
    pub(crate) fn new(modio: Modio<C>, game: u32) -> Self {
        Self { modio, game }
    }

    fn path(&self, more: &str) -> String {
        format!("/games/{}/mods{}", self.game, more)
    }

    /// Return a reference to a mod.
    pub fn get(&self, id: u32) -> ModRef<C> {
        ModRef::new(self.modio.clone(), self.game, id)
    }

    /// List all games.
    pub fn list(&self, options: &ModsListOptions) -> Future<ModioListResponse<Mod>> {
        let mut uri = vec![self.path("")];
        let query = options.to_query_params();
        if !query.is_empty() {
            uri.push(query);
        }
        self.modio.get::<ModioListResponse<Mod>>(&uri.join("&"))
    }

    /// Add a mod and return the newly created Modio mod object.
    pub fn add(&self, options: AddModOptions) -> Future<Mod> {
        self.modio.post_form(&self.path(""), options)
    }

    /// Return the statistics for all mods of a game.
    pub fn statistics(&self, options: &StatsListOptions) -> Future<ModioListResponse<Statistics>> {
        let mut uri = vec![self.path("/stats")];
        let query = options.to_query_params();
        if !query.is_empty() {
            uri.push(query);
        }
        self.modio.get(&uri.join("?"))
    }

    /// Return the event log for all mods of a game sorted by latest event first.
    pub fn events(&self, options: &EventListOptions) -> Future<ModioListResponse<Event>> {
        let mut uri = vec![self.path("/events")];
        let query = options.to_query_params();
        if !query.is_empty() {
            uri.push(query);
        }
        self.modio.get(&uri.join("?"))
    }
}

/// Reference interface of a mod.
pub struct ModRef<C>
where
    C: Clone + Connect + 'static,
{
    modio: Modio<C>,
    game: u32,
    id: u32,
}

impl<C: Clone + Connect + 'static> ModRef<C> {
    pub(crate) fn new(modio: Modio<C>, game: u32, id: u32) -> Self {
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
    pub fn files(&self) -> Files<C> {
        Files::new(self.modio.clone(), self.game, self.id)
    }

    /// Return a reference to a file of a mod.
    pub fn file(&self, id: u32) -> FileRef<C> {
        FileRef::new(self.modio.clone(), self.game, self.id, id)
    }

    /// Return a reference to an interface to manage metadata key value pairs of a mod.
    pub fn metadata(&self) -> Metadata<C> {
        Metadata::new(self.modio.clone(), self.game, self.id)
    }

    /// Return a reference to an interface to manage the tags of a mod.
    pub fn tags(&self) -> Endpoint<C, Tag> {
        Endpoint::new(self.modio.clone(), self.path("/tags"))
    }

    /// Return a reference to an interface that provides access to the comments of a mod.
    pub fn comments(&self) -> Comments<C> {
        Comments::new(self.modio.clone(), self.game, self.id)
    }

    /// Return a reference to an interface to manage the dependencies of a mod.
    pub fn dependencies(&self) -> Endpoint<C, Dependency> {
        Endpoint::new(self.modio.clone(), self.path("/dependencies"))
    }

    /// Return the statistics for a mod.
    pub fn statistics(&self) -> Future<Statistics> {
        self.modio.get(&self.path("/stats"))
    }

    /// Return the event log for a mod sorted by latest event first.
    pub fn events(&self, options: &EventListOptions) -> Future<ModioListResponse<Event>> {
        let mut uri = vec![self.path("/events")];
        let query = options.to_query_params();
        if !query.is_empty() {
            uri.push(query);
        }
        self.modio.get(&uri.join("?"))
    }

    /// Return a reference to an interface to manage team members of a mod.
    pub fn members(&self) -> Members<C> {
        Members::new(self.modio.clone(), self.game, self.id)
    }

    /// Edit details for a mod.
    pub fn edit(&self, options: &EditModOptions) -> Future<Mod> {
        let params = options.to_query_params();
        self.modio.put(&self.path(""), params)
    }

    /// Add new media to a mod.
    pub fn add_media(&self, options: AddMediaOptions) -> Future<ModioMessage> {
        self.modio.post_form(&self.path("/media"), options)
    }

    /// Delete media from a mod.
    pub fn delete_media(&self, options: &DeleteMediaOptions) -> Future<()> {
        self.modio
            .delete(&self.path("/media"), options.to_query_params())
    }

    /// Submit a positive or negative rating for a mod.
    pub fn rate(&self, rating: Rating) -> Future<()> {
        let params = rating.to_query_params();
        Box::new(
            self.modio
                .post::<ModioMessage, _>(&self.path("/ratings"), params)
                .map(|_| ())
                .or_else(|err| match err {
                    Error::Fault {
                        code: StatusCode::BAD_REQUEST,
                        ..
                    } => Ok(()),
                    otherwise => Err(otherwise),
                }),
        )
    }

    /// Subscribe the authenticated user to a mod.
    pub fn subscribe(&self) -> Future<()> {
        Box::new(
            self.modio
                .post::<Mod, _>(&self.path("/subscribe"), Vec::new())
                .map(|_| ())
                .or_else(|err| match err {
                    Error::Fault {
                        code: StatusCode::BAD_REQUEST,
                        ..
                    } => Ok(()),
                    otherwise => Err(otherwise),
                }),
        )
    }

    /// Unsubscribe the authenticated user from a mod.
    pub fn unsubscribe(&self) -> Future<()> {
        Box::new(
            self.modio
                .delete(&self.path("/subscribe"), Vec::new())
                .or_else(|err| match err {
                    Error::Fault {
                        code: StatusCode::BAD_REQUEST,
                        ..
                    } => Ok(()),
                    otherwise => Err(otherwise),
                }),
        )
    }
}

#[derive(Clone, Copy)]
pub enum Rating {
    Positive,
    Negative,
}

impl QueryParams for Rating {
    fn to_query_params(&self) -> String {
        format!(
            "rating={}",
            match *self {
                Rating::Negative => -1,
                Rating::Positive => 1,
            }
        )
    }
}

filter_options!{
    /// Options used to filter mod listings.
    ///
    /// # Filter parameters
    /// - _q
    /// - id
    /// - game_id
    /// - status
    /// - visible
    /// - submitted_by
    /// - date_added
    /// - date_updated
    /// - date_live
    /// - maturity_option
    /// - name
    /// - name_id
    /// - summary
    /// - description
    /// - homepage_url
    /// - modfile
    /// - metadata_blob
    /// - metadata_kvp
    /// - tags
    ///
    /// # Sorting
    /// - id
    /// - name
    /// - downloads
    /// - popular
    /// - ratings
    /// - subscribers
    ///
    /// See the [modio docs](https://docs.mod.io/#get-all-mods) for more informations.
    ///
    /// By default this returns up to `100` items. You can limit the result using `limit` and
    /// `offset`.
    /// # Example
    /// ```
    /// use modio::filter::{Order, Operator};
    /// use modio::mods::ModsListOptions;
    ///
    /// let mut opts = ModsListOptions::new();
    /// opts.id(Operator::In, vec![1, 2]);
    /// opts.sort_by(ModsListOptions::ID, Order::Desc);
    /// ```
    #[derive(Debug)]
    pub struct ModsListOptions {
        Filters
        - id = "id";
        - game_id = "game_id";
        - status = "status";
        - visible = "visible";
        - submitted_by = "submitted_by";
        - date_added = "date_added";
        - date_updated = "date_updated";
        - date_live = "date_live";
        - maturity_option = "maturity_option";
        - name = "name";
        - name_id = "name_id";
        - summary = "summary";
        - description = "description";
        - homepage_url = "homepage_url";
        - modfile = "modfile";
        - metadata_blob = "metadata_blob";
        - metadata_kvp = "metadata_kvp";
        - tags = "tags";

        Sort
        - ID = "id";
        - NAME = "name";
        - DOWNLOADS = "downloads";
        - POPULAR = "popular";
        - RATINGS = "ratings";
        - SUBSCRIBERS = "subscribers";
    }
}

#[derive(Clone, Debug, Default)]
pub struct AddModOptions {
    visible: Option<u32>,
    logo: PathBuf,
    name: String,
    name_id: Option<String>,
    summary: String,
    description: Option<String>,
    homepage_url: Option<Url>,
    stock: Option<u32>,
    maturity_option: Option<u8>,
    metadata_blob: Option<String>,
    tags: Option<Vec<String>>,
}

impl AddModOptions {
    pub fn builder<T, P>(name: T, logo: P, summary: T) -> AddModOptionsBuilder
    where
        T: Into<String>,
        P: AsRef<Path>,
    {
        AddModOptionsBuilder::new(name, logo, summary)
    }
}

impl MultipartForm for AddModOptions {
    fn to_form(&self) -> Result<multipart::Form, Error> {
        let mut form = multipart::Form::default();

        form.add_text("name", self.name.clone());
        form.add_text("summary", self.summary.clone());
        match form.add_file("logo", self.logo.clone()) {
            Ok(_) => {}
            Err(err) => return Err(err.into()),
        };
        if let Some(visible) = self.visible {
            form.add_text("visible", visible.to_string());
        }
        if let Some(ref name_id) = self.name_id {
            form.add_text("name_id", name_id.clone());
        }
        if let Some(ref desc) = self.description {
            form.add_text("description", desc.clone());
        }
        if let Some(ref url) = self.homepage_url {
            form.add_text("homepage_url", url.to_string());
        }
        if let Some(stock) = self.stock {
            form.add_text("stock", stock.to_string());
        }
        if let Some(maturity_option) = self.maturity_option {
            form.add_text("maturity_option", maturity_option.to_string());
        }
        if let Some(ref metadata_blob) = self.metadata_blob {
            form.add_text("metadata_blob", metadata_blob.clone());
        }
        if let Some(ref tags) = self.tags {
            for tag in tags {
                form.add_text("tags[]", tag.clone());
            }
        }
        Ok(form)
    }
}

pub struct AddModOptionsBuilder(AddModOptions);

impl AddModOptionsBuilder {
    pub fn new<T, P>(name: T, logo: P, summary: T) -> Self
    where
        T: Into<String>,
        P: AsRef<Path>,
    {
        AddModOptionsBuilder(AddModOptions {
            name: name.into(),
            logo: logo.as_ref().to_path_buf(),
            summary: summary.into(),
            ..Default::default()
        })
    }

    pub fn visible(&mut self, v: bool) -> &mut Self {
        self.0.visible = if v { Some(1) } else { Some(0) };
        self
    }

    pub fn name_id<S: Into<String>>(&mut self, name_id: S) -> &mut Self {
        self.0.name_id = Some(name_id.into());
        self
    }

    pub fn description<S: Into<String>>(&mut self, description: S) -> &mut Self {
        self.0.description = Some(description.into());
        self
    }

    pub fn homepage_url<U: Into<Url>>(&mut self, url: U) -> &mut Self {
        self.0.homepage_url = Some(url.into());
        self
    }

    pub fn stock(&mut self, stock: u32) -> &mut Self {
        self.0.stock = Some(stock);
        self
    }

    pub fn maturity_option(&mut self, options: u8) -> &mut Self {
        self.0.maturity_option = Some(options);
        self
    }

    pub fn metadata_blob<S: Into<String>>(&mut self, metadata_blob: S) -> &mut Self {
        self.0.metadata_blob = Some(metadata_blob.into());
        self
    }

    pub fn tags(&mut self, tags: Vec<String>) -> &mut Self {
        self.0.tags = Some(tags);
        self
    }

    pub fn build(&self) -> AddModOptions {
        AddModOptions {
            visible: self.0.visible,
            logo: self.0.logo.clone(),
            name: self.0.name.clone(),
            name_id: self.0.name_id.clone(),
            summary: self.0.summary.clone(),
            description: self.0.description.clone(),
            homepage_url: self.0.homepage_url.clone(),
            stock: self.0.stock,
            maturity_option: self.0.maturity_option,
            metadata_blob: self.0.metadata_blob.clone(),
            tags: self.0.tags.clone(),
        }
    }
}

#[derive(Debug, Default)]
pub struct EditModOptions {
    status: Option<u32>,
    visible: Option<u32>,
    name: Option<String>,
    name_id: Option<String>,
    summary: Option<String>,
    description: Option<String>,
    homepage_url: Option<Url>,
    stock: Option<u32>,
    maturity_option: Option<u8>,
    metadata_blob: Option<String>,
}

impl EditModOptions {
    pub fn builder() -> EditModOptionsBuilder {
        EditModOptionsBuilder::new()
    }
}

impl QueryParams for EditModOptions {
    fn to_query_params(&self) -> String {
        form_urlencoded::Serializer::new(String::new())
            .extend_pairs(self.status.iter().map(|s| ("status", s.to_string())))
            .extend_pairs(self.visible.iter().map(|v| ("visible", v.to_string())))
            .extend_pairs(self.name.iter().map(|n| ("name", n)))
            .extend_pairs(self.name_id.iter().map(|n| ("name_id", n)))
            .extend_pairs(self.summary.iter().map(|s| ("summary", s)))
            .extend_pairs(self.description.iter().map(|d| ("description", d)))
            .extend_pairs(self.homepage_url.iter().map(|h| ("homepage_url", h)))
            .extend_pairs(self.stock.iter().map(|s| ("stock", s.to_string())))
            .extend_pairs(
                self.maturity_option
                    .iter()
                    .map(|m| ("maturity_option", m.to_string())),
            )
            .extend_pairs(self.metadata_blob.iter().map(|m| ("metadata_blob", m)))
            .finish()
    }
}

#[derive(Default)]
pub struct EditModOptionsBuilder(EditModOptions);

impl EditModOptionsBuilder {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn status(&mut self, status: u32) -> &mut Self {
        self.0.status = Some(status);
        self
    }

    pub fn visible(&mut self, visible: bool) -> &mut Self {
        self.0.visible = if visible { Some(1) } else { Some(0) };
        self
    }

    pub fn name<T: Into<String>>(&mut self, name: T) -> &mut Self {
        self.0.name = Some(name.into());
        self
    }

    pub fn name_id<T: Into<String>>(&mut self, name_id: T) -> &mut Self {
        self.0.name_id = Some(name_id.into());
        self
    }

    pub fn summary<T: Into<String>>(&mut self, summary: T) -> &mut Self {
        self.0.summary = Some(summary.into());
        self
    }

    pub fn description<T: Into<String>>(&mut self, description: T) -> &mut Self {
        self.0.description = Some(description.into());
        self
    }

    pub fn homepage_url(&mut self, url: Url) -> &mut Self {
        self.0.homepage_url = Some(url);
        self
    }

    pub fn stock(&mut self, stock: u32) -> &mut Self {
        self.0.stock = Some(stock);
        self
    }

    pub fn maturity_option(&mut self, options: u8) -> &mut Self {
        self.0.maturity_option = Some(options);
        self
    }

    pub fn metadata_blob<T: Into<String>>(&mut self, blob: T) -> &mut Self {
        self.0.metadata_blob = Some(blob.into());
        self
    }

    pub fn build(&self) -> EditModOptions {
        EditModOptions {
            status: self.0.status,
            visible: self.0.visible,
            name: self.0.name.clone(),
            name_id: self.0.name_id.clone(),
            summary: self.0.summary.clone(),
            description: self.0.description.clone(),
            homepage_url: self.0.homepage_url.clone(),
            stock: self.0.stock,
            maturity_option: self.0.maturity_option,
            metadata_blob: self.0.metadata_blob.clone(),
        }
    }
}

pub struct EditDepencenciesOptions {
    dependencies: Vec<u32>,
}

impl EditDepencenciesOptions {
    pub fn new(dependencies: Vec<u32>) -> Self {
        Self { dependencies }
    }

    pub fn one(dependency: u32) -> Self {
        Self {
            dependencies: vec![dependency],
        }
    }
}

impl AddOptions for EditDepencenciesOptions {}
impl DeleteOptions for EditDepencenciesOptions {}

impl QueryParams for EditDepencenciesOptions {
    fn to_query_params(&self) -> String {
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
    pub fn new(tags: Vec<String>) -> Self {
        Self { tags }
    }
}

impl AddOptions for EditTagsOptions {}
impl DeleteOptions for EditTagsOptions {}

impl QueryParams for EditTagsOptions {
    fn to_query_params(&self) -> String {
        form_urlencoded::Serializer::new(String::new())
            .extend_pairs(self.tags.iter().map(|t| ("tags[]", t)))
            .finish()
    }
}

#[derive(Clone, Default)]
pub struct AddMediaOptions {
    logo: Option<PathBuf>,
    images_zip: Option<PathBuf>,
    images: Option<Vec<PathBuf>>,
    youtube: Option<Vec<String>>,
    sketchfab: Option<Vec<String>>,
}

impl AddMediaOptions {
    pub fn builder() -> AddMediaOptionsBuilder {
        AddMediaOptionsBuilder::new()
    }
}

impl MultipartForm for AddMediaOptions {
    fn to_form(&self) -> Result<multipart::Form, Error> {
        let mut form = multipart::Form::default();
        if let Some(ref logo) = self.logo {
            if let Err(e) = form.add_file("logo", logo) {
                return Err(e.into());
            }
        }
        if let Some(ref images) = self.images_zip {
            if let Err(e) = form.add_file("images", images) {
                return Err(e.into());
            }
        }
        if let Some(ref images) = self.images {
            for (i, image) in images.iter().enumerate() {
                if let Err(e) = form.add_file(format!("image{}", i), image) {
                    return Err(e.into());
                }
            }
        }
        if let Some(ref youtube) = self.youtube {
            for url in youtube {
                form.add_text("youtube[]", url.clone());
            }
        }
        if let Some(ref sketchfab) = self.sketchfab {
            for url in sketchfab {
                form.add_text("sketchfab[]", url.clone());
            }
        }
        Ok(form)
    }
}

#[derive(Default)]
pub struct AddMediaOptionsBuilder(AddMediaOptions);

impl AddMediaOptionsBuilder {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn logo<P: AsRef<Path>>(&mut self, logo: P) -> &mut Self {
        self.0.logo = Some(logo.as_ref().to_path_buf());
        self
    }

    pub fn images_zip<P: AsRef<Path>>(&mut self, images: P) -> &mut Self {
        self.0.images_zip = Some(images.as_ref().to_path_buf());
        self
    }

    pub fn images<P: AsRef<Path>>(&mut self, images: &[P]) -> &mut Self {
        self.0.images = Some(
            images
                .iter()
                .map(|p| p.as_ref().to_path_buf())
                .collect::<Vec<_>>(),
        );
        self
    }

    pub fn youtube(&mut self, urls: Vec<String>) -> &mut Self {
        self.0.youtube = Some(urls);
        self
    }

    pub fn sketchfab(&mut self, urls: Vec<String>) -> &mut Self {
        self.0.sketchfab = Some(urls);
        self
    }

    pub fn build(&self) -> AddMediaOptions {
        AddMediaOptions {
            logo: self.0.logo.clone(),
            images_zip: self.0.images_zip.clone(),
            images: self.0.images.clone(),
            youtube: self.0.youtube.clone(),
            sketchfab: self.0.sketchfab.clone(),
        }
    }
}

#[derive(Default)]
pub struct DeleteMediaOptions {
    images: Option<Vec<String>>,
    youtube: Option<Vec<String>>,
    sketchfab: Option<Vec<String>>,
}

impl DeleteMediaOptions {
    pub fn builder() -> DeleteMediaOptionsBuilder {
        DeleteMediaOptionsBuilder::new()
    }
}

impl QueryParams for DeleteMediaOptions {
    fn to_query_params(&self) -> String {
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

#[derive(Default)]
pub struct DeleteMediaOptionsBuilder(DeleteMediaOptions);

impl DeleteMediaOptionsBuilder {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn images(&mut self, images: Vec<String>) -> &mut Self {
        self.0.images = Some(images);
        self
    }

    pub fn youtube(&mut self, urls: Vec<String>) -> &mut Self {
        self.0.youtube = Some(urls);
        self
    }

    pub fn sketchfab(&mut self, urls: Vec<String>) -> &mut Self {
        self.0.sketchfab = Some(urls);
        self
    }

    pub fn build(&self) -> DeleteMediaOptions {
        DeleteMediaOptions {
            images: self.0.images.clone(),
            youtube: self.0.youtube.clone(),
            sketchfab: self.0.sketchfab.clone(),
        }
    }
}

filter_options!{
    /// Options used to filter mod statistics.
    ///
    /// # Filter parameters
    /// - mod_id
    /// - popularity_rank_position
    /// - downloads_total
    /// - subscribers_total
    /// - ratings_positive
    /// - ratings_negative
    ///
    /// # Sorting
    /// - mod_id
    /// - popularity_rank_position
    /// - downloads_total
    /// - subscribers_total
    /// - ratings_positive
    /// - ratings_negative
    ///
    /// See the [mod.io docs](https://docs.mod.io/#get-all-mod-stats) for more informations.
    ///
    /// By default this returns up to `100` items. You can limit the result using `limit` and
    /// `offset`.
    /// # Example
    /// ```
    /// use modio::filter::{Order, Operator};
    /// use modio::mods::StatsListOptions;
    ///
    /// let mut opts = StatsListOptions::new();
    /// opts.mod_id(Operator::In, vec![1, 2]);
    /// opts.sort_by(StatsListOptions::MOD_ID, Order::Desc);
    /// ```
    #[derive(Debug)]
    pub struct StatsListOptions {
        Filters
        - mod_id = "mod_id";
        - downloads = "downloads_total";
        - subscribers = "subscribers_total";
        - rank_position = "popularity_rank_position";
        - ratings_positive = "ratings_positive";
        - ratings_negative = "ratings_negative";

        Sort
        - MOD_ID = "mod_id";
        - DOWNLOADS = "downloads_total";
        - SUBSCRIBERS = "subscribers_total";
        - RANK_POSITION = "popularity_rank_position";
        - RATINGS_POSITIVE = "ratings_positive";
        - RATINGS_NEGATIVE = "ratings_negative";
    }
}
