//! Games interface

use std::path::Path;

use hyper::client::connect::Connect;
use mime::IMAGE_STAR;
use url::form_urlencoded;

use crate::multipart::{FileSource, FileStream, MultipartForm};
use crate::Endpoint;
use crate::Future;
use crate::ModRef;
use crate::Modio;
use crate::ModioListResponse;
use crate::ModioMessage;
use crate::Mods;
use crate::{AddOptions, DeleteOptions, QueryParams};

pub use crate::types::game::{Game, HeaderImage, Icon, TagOption, TagType};
pub use crate::types::Logo;

/// Interface for games the authenticated user added or is team member of.
pub struct MyGames<C>
where
    C: Clone + Connect + 'static,
{
    modio: Modio<C>,
}

impl<C: Clone + Connect + 'static> MyGames<C> {
    pub(crate) fn new(modio: Modio<C>) -> Self {
        Self { modio }
    }

    /// List all games the authenticated user added or is team member of.
    pub fn list(&self, options: &GamesListOptions) -> Future<ModioListResponse<Game>> {
        let mut uri = vec!["/me/games".to_owned()];
        let query = options.to_query_params();
        if !query.is_empty() {
            uri.push(query);
        }
        self.modio.get::<ModioListResponse<Game>>(&uri.join("?"))
    }
}

/// Interface for games.
pub struct Games<C>
where
    C: Clone + Connect + 'static,
{
    modio: Modio<C>,
}

impl<C: Clone + Connect + 'static> Games<C> {
    pub(crate) fn new(modio: Modio<C>) -> Self {
        Self { modio }
    }

    fn path(&self, more: &str) -> String {
        format!("/games{}", more)
    }

    /// List all games.
    pub fn list(&self, options: &GamesListOptions) -> Future<ModioListResponse<Game>> {
        let mut uri = vec![self.path("")];
        let query = options.to_query_params();
        if !query.is_empty() {
            uri.push(query);
        }
        self.modio.get::<ModioListResponse<Game>>(&uri.join("?"))
    }

    /// Return a reference to a game.
    pub fn get(&self, id: u32) -> GameRef<C> {
        GameRef::new(self.modio.clone(), id)
    }
}

/// Reference interface of a game.
pub struct GameRef<C>
where
    C: Clone + Connect + 'static,
{
    modio: Modio<C>,
    id: u32,
}

impl<C: Clone + Connect + 'static> GameRef<C> {
    pub(crate) fn new(modio: Modio<C>, id: u32) -> Self {
        Self { modio, id }
    }

    fn path(&self, more: &str) -> String {
        format!("/games/{}{}", self.id, more)
    }

    /// Get a reference to the Modio game object that this `GameRef` refers to.
    pub fn get(&self) -> Future<Game> {
        self.modio.get::<Game>(&format!("/games/{}", self.id))
    }

    /// Return a reference to a mod of a game.
    pub fn mod_(&self, mod_id: u32) -> ModRef<C> {
        ModRef::new(self.modio.clone(), self.id, mod_id)
    }

    /// Return a reference to an interface that provides access to the mods of a game.
    pub fn mods(&self) -> Mods<C> {
        Mods::new(self.modio.clone(), self.id)
    }

    /// Return a reference to an interface that provides access to the tags of a game.
    pub fn tags(&self) -> Endpoint<C, TagOption> {
        Endpoint::new(self.modio.clone(), self.path("/tags"))
    }

    /// Add or edit new media to a game.
    pub fn add_media(&self, media: GameMediaOptions) -> Future<ModioMessage> {
        self.modio.post_form(&self.path("/media"), media)
    }
}

filter_options! {
    /// Options used to filter game listings.
    ///
    /// # Filter parameters
    /// - _q
    /// - id
    /// - status
    /// - submitted_by
    /// - date_added
    /// - date_updated
    /// - date_live
    /// - name
    /// - name_id
    /// - summary
    /// - instructions_url
    /// - ugc_name
    /// - presentation_option
    /// - submission_option
    /// - curation_option
    /// - community_options
    /// - revenue_options
    /// - api_access_options
    /// - maturity_options
    ///
    /// # Sorting
    /// - id
    /// - status
    /// - name
    /// - name_id
    /// - date_updated
    ///
    /// See [modio docs](https://docs.mod.io/#get-all-games) for more information.
    ///
    /// By default this returns up to `100` items. You can limit the result using `limit` and
    /// `offset`.
    /// # Example
    /// ```
    /// use modio::filter::{Order, Operator};
    /// use modio::games::GamesListOptions;
    ///
    /// let mut opts = GamesListOptions::new();
    /// opts.id(Operator::In, vec![1, 2]);
    /// opts.sort_by(GamesListOptions::ID, Order::Desc);
    /// ```
    #[derive(Debug)]
    pub struct GamesListOptions {
        Filters
        - id = "id";
        - status = "status";
        - submitted_by = "submitted_by";
        - date_added = "date_added";
        - date_updated = "date_updated";
        - date_live = "date_live";
        - name = "name";
        - name_id = "name_id";
        - summary = "summary";
        - instructions_url = "instructions_url";
        - ugc_name = "ugc_name";
        - presentation_option = "presentation_option";
        - submission_option = "submission_option";
        - curation_option = "curation_option";
        - community_options = "community_options";
        - revenue_options = "revenue_options";
        - api_access_options = "api_access_options";
        - maturity_options = "maturity_options";

        Sort
        - ID = "id";
        - STATUS = "status";
        - NAME = "name";
        - NAME_ID = "name_id";
        - DATE_UPDATED = "date_updated";
    }
}

pub struct AddTagsOptions {
    name: String,
    kind: TagType,
    hidden: bool,
    tags: Vec<String>,
}

impl AddTagsOptions {
    pub fn public<S: Into<String>>(name: S, kind: TagType, tags: &[String]) -> Self {
        Self {
            name: name.into(),
            kind,
            hidden: false,
            tags: tags.to_vec(),
        }
    }

    pub fn hidden<S: Into<String>>(name: S, kind: TagType, tags: &[String]) -> Self {
        Self {
            name: name.into(),
            kind,
            hidden: true,
            tags: tags.to_vec(),
        }
    }
}

impl AddOptions for AddTagsOptions {}

impl QueryParams for AddTagsOptions {
    fn to_query_params(&self) -> String {
        form_urlencoded::Serializer::new(String::new())
            .append_pair("name", &self.name)
            .append_pair("type", &self.kind.to_string())
            .append_pair("hidden", &self.hidden.to_string())
            .extend_pairs(self.tags.iter().map(|t| ("tags[]", t)))
            .finish()
    }
}

pub struct DeleteTagsOptions {
    name: String,
    tags: Option<Vec<String>>,
}

impl DeleteTagsOptions {
    pub fn all<S: Into<String>>(name: S) -> Self {
        Self {
            name: name.into(),
            tags: None,
        }
    }

    pub fn some<S: Into<String>>(name: S, tags: &[String]) -> Self {
        Self {
            name: name.into(),
            tags: if tags.is_empty() {
                None
            } else {
                Some(tags.to_vec())
            },
        }
    }
}

impl DeleteOptions for DeleteTagsOptions {}

impl QueryParams for DeleteTagsOptions {
    fn to_query_params(&self) -> String {
        let mut ser = form_urlencoded::Serializer::new(String::new());
        ser.append_pair("name", &self.name);
        match &self.tags {
            Some(tags) => ser.extend_pairs(tags.iter().map(|t| ("tags[]", t))),
            None => ser.append_pair("tags[]", ""),
        };
        ser.finish()
    }
}

pub struct GameMediaOptions {
    logo: Option<FileSource>,
    icon: Option<FileSource>,
    header: Option<FileSource>,
}

impl GameMediaOptions {
    pub fn builder() -> GameMediaOptionsBuilder {
        GameMediaOptionsBuilder::new()
    }
}

#[doc(hidden)]
impl From<GameMediaOptions> for MultipartForm {
    fn from(opts: GameMediaOptions) -> MultipartForm {
        let mut mpart = MultipartForm::default();
        if let Some(logo) = opts.logo {
            mpart.add_stream("logo", &logo.filename, &logo.mime.to_string(), logo.inner);
        }
        if let Some(icon) = opts.icon {
            mpart.add_stream("icon", &icon.filename, &icon.mime.to_string(), icon.inner);
        }
        if let Some(header) = opts.header {
            mpart.add_stream(
                "header",
                &header.filename,
                &header.mime.to_string(),
                header.inner,
            );
        }
        mpart
    }
}

pub struct GameMediaOptionsBuilder(GameMediaOptions);

impl GameMediaOptionsBuilder {
    fn new() -> Self {
        GameMediaOptionsBuilder(GameMediaOptions {
            logo: None,
            icon: None,
            header: None,
        })
    }

    pub fn logo<P: AsRef<Path>>(&mut self, logo: P) -> &mut Self {
        let logo = logo.as_ref();
        let filename = logo
            .file_name()
            .and_then(|n| n.to_str())
            .map_or_else(String::new, |n| n.to_string());

        self.0.logo = Some(FileSource {
            inner: FileStream::open(logo),
            filename,
            mime: IMAGE_STAR,
        });
        self
    }

    pub fn icon<P: AsRef<Path>>(&mut self, icon: P) -> &mut Self {
        let icon = icon.as_ref();
        let filename = icon
            .file_name()
            .and_then(|n| n.to_str())
            .map_or_else(String::new, |n| n.to_string());

        self.0.icon = Some(FileSource {
            inner: FileStream::open(icon),
            filename,
            mime: IMAGE_STAR,
        });
        self
    }

    pub fn header<P: AsRef<Path>>(&mut self, header: P) -> &mut Self {
        let header = header.as_ref();
        let filename = header
            .file_name()
            .and_then(|n| n.to_str())
            .map_or_else(String::new, |n| n.to_string());

        self.0.header = Some(FileSource {
            inner: FileStream::open(header),
            filename,
            mime: IMAGE_STAR,
        });
        self
    }

    pub fn build(self) -> GameMediaOptions {
        GameMediaOptions {
            logo: self.0.logo,
            icon: self.0.icon,
            header: self.0.header,
        }
    }
}
