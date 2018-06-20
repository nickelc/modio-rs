use std::collections::HashMap;

use hyper::client::Connect;
use url::form_urlencoded;

use types::mods::Comment;
use types::ModioListResponse;
use Future;
use Modio;

pub struct Comments<C>
where
    C: Clone + Connect,
{
    modio: Modio<C>,
    game: u32,
    mod_id: u32,
}

impl<C: Clone + Connect> Comments<C> {
    pub fn new(modio: Modio<C>, game: u32, mod_id: u32) -> Self {
        Self {
            modio,
            game,
            mod_id,
        }
    }

    fn path(&self, more: &str) -> String {
        format!("/games/{}/mods/{}/comments{}", self.game, self.mod_id, more)
    }

    pub fn list(&self, options: &CommentsListOptions) -> Future<ModioListResponse<Comment>> {
        let mut uri = vec![self.path("")];
        if let Some(query) = options.serialize() {
            uri.push(query);
        }
        self.modio.get(&uri.join("?"))
    }

    pub fn delete(&self, id: u32) -> Future<()> {
        self.modio
            .delete(&self.path(&format!("/{}", id)), Vec::new())
    }
}

#[derive(Default)]
pub struct CommentsListOptions {
    params: HashMap<&'static str, String>,
}

impl CommentsListOptions {
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
