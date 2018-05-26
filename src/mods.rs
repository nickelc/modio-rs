use std::collections::HashMap;
use std::path::PathBuf;

use hyper::client::Connect;
use url::form_urlencoded;

use Comments;
use Files;
use Future;
use Modio;
use types::ModioListResponse;
use types::mods::*;

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

    pub fn tags(&self) -> Tags<C> {
        Tags::new(self.modio.clone(), self.game, self.id)
    }

    pub fn comments(&self) -> Comments<C> {
        Comments::new(self.modio.clone(), self.game, self.id)
    }

    pub fn dependencies(&self) -> Future<ModioListResponse<Dependency>> {
        self.modio.get(&self.path("/dependencies"))
    }
}

pub struct Tags<C>
where
    C: Clone + Connect,
{
    modio: Modio<C>,
    game: u32,
    mod_id: u32,
}

impl<C: Clone + Connect> Tags<C> {
    pub fn new(modio: Modio<C>, game: u32, mod_id: u32) -> Self {
        Self {
            modio,
            game,
            mod_id,
        }
    }

    fn path(&self, more: &str) -> String {
        format!("/games/{}/mods/{}/tags{}", self.game, self.mod_id, more)
    }

    pub fn list(&self) -> Future<ModioListResponse<Tag>> {
        self.modio.get(&self.path(""))
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
