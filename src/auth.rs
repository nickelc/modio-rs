//! Authentication Flow interface

use futures::Future as StdFuture;
use hyper::client::connect::Connect;
use url::form_urlencoded;

use Future;
use Modio;
use ModioMessage;

/// Various forms of authentication credentials supported by [mod.io](https://mod.io).
#[derive(Clone, Debug, PartialEq)]
pub enum Credentials {
    ApiKey(String),
    Token(String),
}

/// Authentication Flow interface to retrieve access tokens. See the [Modio Authentication
/// docs](https://docs.mod.io/#authentication) for more informations.
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

    /// Request a security code be sent to the email of the user.
    pub fn request_code(&self, email: &str) -> Future<ModioMessage> {
        let data = form_urlencoded::Serializer::new(String::new())
            .append_pair("email", email)
            .finish();
        self.modio.post("/oauth/emailrequest", data)
    }

    /// Get the access token for a security code.
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
