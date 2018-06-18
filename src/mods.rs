use std::collections::HashMap;
use std::path::{Path, PathBuf};

use futures::future;
use hyper::client::Connect;
use hyper_multipart::client::multipart;
use serde_urlencoded;
use url::{form_urlencoded, Url};
use url_serde;

use errors::Error;
use types::mods::*;
use types::Event;
use types::ModioListResponse;
use Comments;
use Endpoint;
use Files;
use Future;
use Modio;
use MultipartForm;
use {AddOptions, DeleteOptions, QueryParams};

pub struct MyMods<C>
where
    C: Clone + Connect,
{
    modio: Modio<C>,
}

impl<C: Clone + Connect> MyMods<C> {
    pub fn new(modio: Modio<C>) -> Self {
        Self { modio }
    }

    pub fn list(&self, options: &ModsListOptions) -> Future<ModioListResponse<Mod>> {
        let mut uri = vec!["/me/mods".to_owned()];
        if let Some(query) = options.serialize() {
            uri.push(query);
        }
        self.modio.get::<ModioListResponse<Mod>>(&uri.join("?"))
    }
}

pub struct Mods<C>
where
    C: Clone + Connect,
{
    modio: Modio<C>,
    game: u32,
}

impl<C> Mods<C>
where
    C: Clone + Connect,
{
    pub fn new(modio: Modio<C>, game: u32) -> Self {
        Self { modio, game }
    }

    fn path(&self, more: &str) -> String {
        format!("/games/{}/mods{}", self.game, more)
    }

    pub fn get(&self, id: u32) -> ModRef<C> {
        ModRef::new(self.modio.clone(), self.game, id)
    }

    pub fn list(&self, options: &ModsListOptions) -> Future<ModioListResponse<Mod>> {
        let mut uri = vec![self.path("")];
        if let Some(query) = options.serialize() {
            uri.push(query);
        }
        self.modio.get::<ModioListResponse<Mod>>(&uri.join("&"))
    }

    pub fn add(&self, options: &'static AddModOptions) -> Future<Mod> {
        self.modio.post_form(&self.path(""), options)
    }

    pub fn events(&self) -> Future<ModioListResponse<Event>> {
        self.modio.get(&self.path("/events"))
    }
}

pub struct ModRef<C>
where
    C: Clone + Connect,
{
    modio: Modio<C>,
    game: u32,
    id: u32,
}

impl<C: Clone + Connect> ModRef<C> {
    pub fn new(modio: Modio<C>, game: u32, id: u32) -> Self {
        Self { modio, game, id }
    }

    fn path(&self, more: &str) -> String {
        format!("/games/{}/mods/{}{}", self.game, self.id, more)
    }

    pub fn get(&self) -> Future<Mod> {
        self.modio.get(&self.path(""))
    }

    pub fn files(&self) -> Files<C> {
        Files::new(self.modio.clone(), self.game, self.id)
    }

    pub fn tags(&self) -> Endpoint<C, Tag> {
        Endpoint::new(self.modio.clone(), self.path("/tags"))
    }

    pub fn comments(&self) -> Comments<C> {
        Comments::new(self.modio.clone(), self.game, self.id)
    }

    pub fn dependencies(&self) -> Endpoint<C, Dependency> {
        Endpoint::new(self.modio.clone(), self.path("/dependencies"))
    }

    pub fn events(&self) -> Future<ModioListResponse<Event>> {
        self.modio.get(&self.path("/events"))
    }

    pub fn edit(&self, options: &EditModOptions) -> Future<Mod> {
        let msg = match serde_urlencoded::to_string(&options) {
            Ok(data) => data,
            Err(err) => return Box::new(future::err(err.into())),
        };

        self.modio.put(&self.path(""), msg.as_bytes().to_vec())
    }
}

#[derive(Default)]
pub struct ModsListOptions {
    params: HashMap<&'static str, String>,
}

impl ModsListOptions {
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

#[derive(Clone, Debug, Default, Serialize)]
pub struct AddModOptions {
    visible: Option<u32>,
    logo: PathBuf,
    name: String,
    name_id: Option<String>,
    summary: String,
    description: Option<String>,
    #[serde(with = "url_serde")]
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

impl<'a> MultipartForm for &'a AddModOptions {
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

#[derive(Debug, Default, Serialize)]
pub struct EditModOptions {
    status: Option<u32>,
    visible: Option<u32>,
    name: Option<String>,
    name_id: Option<String>,
    summary: Option<String>,
    description: Option<String>,
    #[serde(with = "url_serde")]
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

pub struct EditModOptionsBuilder(EditModOptions);

impl EditModOptionsBuilder {
    pub fn new() -> Self {
        EditModOptionsBuilder(Default::default())
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
