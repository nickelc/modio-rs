use std::path::{Path, PathBuf};

use hyper::client::connect::Connect;
use hyper_multipart::client::multipart;
use url::form_urlencoded;

use errors::Error;
use filter::{Filter, OneOrMany, Operator, Order, SortField};
use types::game::*;
use types::ModioListResponse;
use Endpoint;
use Future;
use ModRef;
use Modio;
use ModioMessage;
use Mods;
use {AddOptions, DeleteOptions, MultipartForm, QueryParams};

pub struct MyGames<C>
where
    C: Clone + Connect + 'static,
{
    modio: Modio<C>,
}

impl<C: Clone + Connect + 'static> MyGames<C> {
    pub fn new(modio: Modio<C>) -> Self {
        Self { modio }
    }

    pub fn list(&self, options: &GamesListOptions) -> Future<ModioListResponse<Game>> {
        let mut uri = vec!["/me/games".to_owned()];
        let query = options.to_query_params();
        if !query.is_empty() {
            uri.push(query);
        }
        self.modio.get::<ModioListResponse<Game>>(&uri.join("?"))
    }
}

pub struct Games<C>
where
    C: Clone + Connect + 'static,
{
    modio: Modio<C>,
}

impl<C: Clone + Connect + 'static> Games<C> {
    pub fn new(modio: Modio<C>) -> Self {
        Self { modio }
    }

    fn path(&self, more: &str) -> String {
        format!("/games{}", more)
    }

    pub fn list(&self, options: &GamesListOptions) -> Future<ModioListResponse<Game>> {
        let mut uri = vec![self.path("")];
        let query = options.to_query_params();
        if !query.is_empty() {
            uri.push(query);
        }
        self.modio.get::<ModioListResponse<Game>>(&uri.join("?"))
    }

    pub fn get(&self, id: u32) -> GameRef<C> {
        GameRef::new(self.modio.clone(), id)
    }
}

pub struct GameRef<C>
where
    C: Clone + Connect + 'static,
{
    modio: Modio<C>,
    id: u32,
}

impl<C: Clone + Connect + 'static> GameRef<C> {
    pub fn new(modio: Modio<C>, id: u32) -> Self {
        Self { modio, id }
    }

    fn path(&self, more: &str) -> String {
        format!("/games/{}{}", self.id, more)
    }

    pub fn get(&self) -> Future<Game> {
        self.modio.get::<Game>(&format!("/games/{}", self.id))
    }

    pub fn mod_(&self, mod_id: u32) -> ModRef<C> {
        ModRef::new(self.modio.clone(), self.id, mod_id)
    }

    pub fn mods(&self) -> Mods<C> {
        Mods::new(self.modio.clone(), self.id)
    }

    pub fn tags(&self) -> Endpoint<C, TagOption> {
        Endpoint::new(self.modio.clone(), self.path("/tags"))
    }

    pub fn add_media(&self, media: GameMediaOptions) -> Future<ModioMessage> {
        self.modio.post_form(&self.path("/media"), media)
    }
}

filter_options!{
    /// Options used to filter game listings
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
    /// See [modio docs](https://docs.mod.io/#get-all-games) for more informations.
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
    pub fn public<S: Into<String>>(name: S, kind: TagType, tags: Vec<String>) -> Self {
        Self {
            name: name.into(),
            kind,
            hidden: false,
            tags,
        }
    }

    pub fn hidden<S: Into<String>>(name: S, kind: TagType, tags: Vec<String>) -> Self {
        Self {
            name: name.into(),
            kind,
            hidden: true,
            tags,
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

    pub fn some<S: Into<String>>(name: S, tags: Vec<String>) -> Self {
        Self {
            name: name.into(),
            tags: if tags.is_empty() { None } else { Some(tags) },
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

#[derive(Clone, Default)]
pub struct GameMediaOptions {
    logo: Option<PathBuf>,
    icon: Option<PathBuf>,
    header: Option<PathBuf>,
}

impl GameMediaOptions {
    pub fn builder() -> GameMediaOptionsBuilder {
        GameMediaOptionsBuilder::new()
    }
}

impl MultipartForm for GameMediaOptions {
    fn to_form(&self) -> Result<multipart::Form, Error> {
        let mut form = multipart::Form::default();
        if let Some(ref logo) = self.logo {
            if let Err(e) = form.add_file("logo", logo) {
                return Err(e.into());
            }
        }
        if let Some(ref icon) = self.icon {
            if let Err(e) = form.add_file("icon", icon) {
                return Err(e.into());
            };
        }
        if let Some(ref header) = self.header {
            if let Err(e) = form.add_file("header", header) {
                return Err(e.into());
            }
        }
        Ok(form)
    }
}

pub struct GameMediaOptionsBuilder(GameMediaOptions);

impl GameMediaOptionsBuilder {
    pub fn new() -> Self {
        GameMediaOptionsBuilder(Default::default())
    }

    pub fn logo<P: AsRef<Path>>(&mut self, logo: P) -> &mut Self {
        self.0.logo = Some(logo.as_ref().to_path_buf());
        self
    }

    pub fn icon<P: AsRef<Path>>(&mut self, icon: P) -> &mut Self {
        self.0.icon = Some(icon.as_ref().to_path_buf());
        self
    }

    pub fn header<P: AsRef<Path>>(&mut self, header: P) -> &mut Self {
        self.0.header = Some(header.as_ref().to_path_buf());
        self
    }

    pub fn build(&self) -> GameMediaOptions {
        GameMediaOptions {
            logo: self.0.logo.clone(),
            icon: self.0.icon.clone(),
            header: self.0.header.clone(),
        }
    }
}
