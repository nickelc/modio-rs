use std::collections::HashMap;

use hyper::client::Connect;
use url::form_urlencoded;

use Future;
use Modio;
use Mods;
use types::ModioListResponse;
use types::game::*;

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

    pub fn get(&self) -> Future<Game> {
        self.modio.get::<Game>(&format!("/games/{}", self.id))
    }

    pub fn mods(&self) -> Mods<C> {
        Mods::new(self.modio.clone(), self.id)
    }

    pub fn tags(&self) -> Tags<C> {
        Tags::new(self.modio.clone(), self.id)
    }
}

pub struct Tags<C>
where
    C: Clone + Connect,
{
    modio: Modio<C>,
    game: u32,
}

impl<C: Clone + Connect> Tags<C> {
    pub fn new(modio: Modio<C>, game: u32) -> Self {
        Self { modio, game }
    }

    pub fn list(&self) -> Future<ModioListResponse<TagOption>> {
        self.modio.get(&format!("/games/{}/tags", self.game))
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
