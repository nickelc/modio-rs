use std::collections::HashMap;

use hyper::client::Connect;
use url::form_urlencoded;

use Future;
use Modio;
use ModioListResponse;
use User;

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

    pub fn list(&self, options: &CommentsListOptions) -> Future<ModioListResponse<Comment>> {
        let mut uri = vec![
            format!("/games/{}/mods/{}/comments", self.game, self.mod_id),
        ];
        if let Some(query) = options.serialize() {
            uri.push(query);
        }
        self.modio.get(&uri.join("?"))
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

#[derive(Debug, Deserialize)]
pub struct Comment {
    id: u32,
    mod_id: u32,
    #[serde(rename = "user")]
    submitted_by: User,
    date_added: u64,
    reply_id: u32,
    thread_position: String,
    karma: u32,
    karma_guest: u32,
    content: String,
}
