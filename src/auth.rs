//! Authentication Flow interface
use futures::Future as StdFuture;
use url::form_urlencoded;

use crate::Future;
use crate::Modio;
use crate::ModioMessage;

/// Various forms of authentication credentials supported by [mod.io](https://mod.io).
#[derive(Clone, Debug, PartialEq)]
pub enum Credentials {
    ApiKey(String),
    Token(String),
}

/// Authentication Flow interface to retrieve access tokens. See the [mod.io Authentication
/// docs](https://docs.mod.io/#email-authentication-flow) for more information.
///
/// # Example
/// ```no_run
/// use std::io::{self, Write};
/// use tokio::runtime::Runtime;
///
/// use modio::error::Error;
/// use modio::{Credentials, Modio};
///
/// fn prompt(prompt: &str) -> io::Result<String> {
///     print!("{}", prompt);
///     io::stdout().flush()?;
///     let mut buffer = String::new();
///     io::stdin().read_line(&mut buffer)?;
///     Ok(buffer.trim().to_string())
/// }
///
/// fn main() -> Result<(), Error> {
///     let mut rt = Runtime::new().expect("new rt");
///     let modio = Modio::new(
///         Credentials::ApiKey(String::from("api-key")),
///     )?;
///
///     let email = prompt("Enter email: ").expect("read email");
///     rt.block_on(modio.auth().request_code(&email))?;
///
///     let code = prompt("Enter security code: ").expect("read code");
///     let token = rt.block_on(modio.auth().security_code(&code))?;
///
///     // Consume the endpoint and create an endpoint with new credentials.
///     let _modio = modio.with_credentials(Credentials::Token(token));
///
///     Ok(())
/// }
/// ```
pub struct Auth {
    modio: Modio,
}

#[derive(Deserialize)]
struct AccessToken {
    access_token: String,
}

impl Auth {
    pub(crate) fn new(modio: Modio) -> Self {
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
        let data = form_urlencoded::Serializer::new(String::new())
            .append_pair("security_code", code)
            .finish();

        Box::new(
            self.modio
                .post::<AccessToken, _>("/oauth/emailexchange", data)
                .map(|token| token.access_token),
        )
    }

    /// Get the access token for an encrypted steam user auth ticket. See the [modio
    /// docs](https://docs.mod.io/#authenticate-via-steam) for more information.
    pub fn steam_auth(&self, ticket: &str) -> Future<String> {
        let data = form_urlencoded::Serializer::new(String::new())
            .append_pair("appdata", ticket)
            .finish();

        Box::new(
            self.modio
                .post::<AccessToken, _>("/external/steamauth", data)
                .map(|token| token.access_token),
        )
    }
}
