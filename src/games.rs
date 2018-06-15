use std::collections::HashMap;

use hyper::client::Connect;
use url::form_urlencoded;

use types::game::*;
use types::ModioListResponse;
use Endpoint;
use Future;
use ModRef;
use Modio;
use Mods;
use {AddOptions, DeleteOptions, QueryParams};

pub struct MyGames<C>
where
    C: Clone + Connect,
{
    modio: Modio<C>,
}

impl<C: Clone + Connect> MyGames<C> {
    pub fn new(modio: Modio<C>) -> Self {
        Self { modio }
    }

    pub fn list(&self, options: &GamesListOptions) -> Future<ModioListResponse<Game>> {
        let mut uri = vec!["/me/games".to_owned()];
        if let Some(query) = options.serialize() {
            uri.push(query);
        }
        self.modio.get::<ModioListResponse<Game>>(&uri.join("?"))
    }
}

pub struct Games<C>
where
    C: Clone + Connect,
{
    modio: Modio<C>,
}

impl<C: Clone + Connect> Games<C> {
    pub fn new(modio: Modio<C>) -> Self {
        Self { modio }
    }

    fn path(&self, more: &str) -> String {
        format!("/games{}", more)
    }

    pub fn list(&self, options: &GamesListOptions) -> Future<ModioListResponse<Game>> {
        let mut uri = vec![self.path("")];
        if let Some(query) = options.serialize() {
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
    C: Clone + Connect,
{
    modio: Modio<C>,
    id: u32,
}

impl<C: Clone + Connect> GameRef<C> {
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
}

#[derive(Default)]
pub struct GamesListOptions {
    params: HashMap<&'static str, String>,
}

impl GamesListOptions {
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
