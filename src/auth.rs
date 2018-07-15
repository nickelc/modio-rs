use futures::Future as StdFuture;
use hyper::client::connect::Connect;
use url::form_urlencoded;

use Future;
use Modio;
use ModioMessage;

pub struct Auth<C>
where
    C: Clone + Connect + 'static,
{
    modio: Modio<C>,
}

impl<C: Clone + Connect> Auth<C> {
    pub(crate) fn new(modio: Modio<C>) -> Self {
        Self { modio }
    }

    pub fn request_code(&self, email: &str) -> Future<ModioMessage> {
        let data = form_urlencoded::Serializer::new(String::new())
            .append_pair("email", email)
            .finish();
        self.modio.post("/oauth/emailrequest", data)
    }

    pub fn security_code(&self, code: &str) -> Future<String> {
        #[derive(Deserialize)]
        struct AccessToken {
            access_token: String,
        }

        let data = form_urlencoded::Serializer::new(String::new())
            .append_pair("security_code", code)
            .finish();

        Box::new(
            self.modio
                .post::<AccessToken, _>("/oauth/emailexchange", data)
                .map(|token| token.access_token),
        )
    }
}
