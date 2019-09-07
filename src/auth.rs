//! Authentication Flow interface
use std::fmt;

use url::form_urlencoded;

use crate::error::Result;
use crate::Modio;
use crate::ModioMessage;

/// Various forms of authentication credentials supported by [mod.io](https://mod.io).
#[derive(Clone, Debug, PartialEq)]
pub enum Credentials {
    ApiKey(String),
    Token(String),
}

impl fmt::Display for Credentials {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Credentials::ApiKey(key) => f.write_str(&key),
            Credentials::Token(token) => f.write_str(&token),
        }
    }
}

/// Various forms of supported external platforms.
pub enum Service {
    Steam(u64),
    Gog(u64),
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
///     let _modio = modio.with_credentials(token);
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

    /// Request a security code be sent to the email of the user. [required: apikey]
    pub async fn request_code(&self, email: &str) -> Result<()> {
        apikey_required!(self.modio);
        let data = form_urlencoded::Serializer::new(String::new())
            .append_pair("email", email)
            .finish();

        self.modio
            .post::<ModioMessage, _>("/oauth/emailrequest", data)
            .await?;

        Ok(())
    }

    /// Get the access token for a security code. [required: apikey]
    pub async fn security_code(&self, code: &str) -> Result<Credentials> {
        apikey_required!(self.modio);
        let data = form_urlencoded::Serializer::new(String::new())
            .append_pair("security_code", code)
            .finish();

        let token = self
            .modio
            .post::<AccessToken, _>("/oauth/emailexchange", data)
            .await?;

        Ok(Credentials::Token(token.access_token))
    }

    /// Link an external account. Requires an auth token from the external platform.
    ///
    /// See the [mod.io docs](https://docs.mod.io/#link-external-account) for more information.
    pub async fn link(&self, email: &str, service: Service) -> Result<()> {
        token_required!(self.modio);
        let (service, id) = match service {
            Service::Steam(id) => ("steam", id.to_string()),
            Service::Gog(id) => ("gog", id.to_string()),
        };
        let data = form_urlencoded::Serializer::new(String::new())
            .append_pair("email", email)
            .append_pair("service", service)
            .append_pair("service_id", &id)
            .finish();

        self.modio
            .post::<ModioMessage, _>("/external/link", data)
            .await?;

        Ok(())
    }

    /// Get the access token for an encrypted gog app ticket. [required: apikey]
    ///
    /// See the [mod.io docs](https://docs.mod.io/#authenticate-via-gog-galaxy) for more
    /// information.
    pub async fn gog_auth(&self, ticket: &str) -> Result<Credentials> {
        apikey_required!(self.modio);
        let data = form_urlencoded::Serializer::new(String::new())
            .append_pair("appdata", ticket)
            .finish();

        let token = self
            .modio
            .post::<AccessToken, _>("/external/galaxyauth", data)
            .await?;

        Ok(Credentials::Token(token.access_token))
    }

    /// Get the access token for an encrypted steam app ticket. [required: apikey]
    ///
    /// See the [mod.io docs](https://docs.mod.io/#authenticate-via-steam) for more information.
    pub async fn steam_auth(&self, ticket: &str) -> Result<Credentials> {
        apikey_required!(self.modio);
        let data = form_urlencoded::Serializer::new(String::new())
            .append_pair("appdata", ticket)
            .finish();

        let token = self
            .modio
            .post::<AccessToken, _>("/external/steamauth", data)
            .await?;

        Ok(Credentials::Token(token.access_token))
    }
}
