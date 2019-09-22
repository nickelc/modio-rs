//! Authentication Flow interface
use std::fmt;

use url::form_urlencoded;

use crate::error::Result;
use crate::routing::Route;
use crate::Modio;
use crate::ModioMessage;
use crate::QueryString;

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

#[derive(Debug)]
pub struct OculusOptions {
    params: std::collections::BTreeMap<&'static str, String>,
}

impl OculusOptions {
    pub fn new<T>(nonce: T, user_id: u64, auth_token: T) -> Self
    where
        T: Into<String>,
    {
        let mut params = std::collections::BTreeMap::new();
        params.insert("nonce", nonce.into());
        params.insert("user_id", user_id.to_string());
        params.insert("auth_token", auth_token.into());
        Self { params }
    }

    option!(email >> "email");
}

impl QueryString for OculusOptions {
    fn to_query_string(&self) -> String {
        form_urlencoded::Serializer::new(String::new())
            .extend_pairs(&self.params)
            .finish()
    }
}

/// Authentication Flow interface to retrieve access tokens. See the [mod.io Authentication
/// docs](https://docs.mod.io/#email-authentication-flow) for more information.
///
/// # Example
/// ```no_run
/// use std::io::{self, Write};
///
/// use modio::{Credentials, Modio, Result};
///
/// fn prompt(prompt: &str) -> io::Result<String> {
///     print!("{}", prompt);
///     io::stdout().flush()?;
///     let mut buffer = String::new();
///     io::stdin().read_line(&mut buffer)?;
///     Ok(buffer.trim().to_string())
/// }
///
/// #[tokio::main]
/// async fn main() -> Result<()> {
///     let modio = Modio::new(
///         Credentials::ApiKey(String::from("api-key")),
///     )?;
///
///     let email = prompt("Enter email: ").expect("read email");
///     modio.auth().request_code(&email).await?;
///
///     let code = prompt("Enter security code: ").expect("read code");
///     let token = modio.auth().security_code(&code).await?;
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
    pub async fn request_code(self, email: &str) -> Result<()> {
        let data = form_urlencoded::Serializer::new(String::new())
            .append_pair("email", email)
            .finish();

        self.modio
            .request(Route::AuthEmailRequest)
            .body(data)
            .send::<ModioMessage>()
            .await?;

        Ok(())
    }

    /// Get the access token for a security code. [required: apikey]
    pub async fn security_code(self, code: &str) -> Result<Credentials> {
        let data = form_urlencoded::Serializer::new(String::new())
            .append_pair("security_code", code)
            .finish();

        let token = self
            .modio
            .request(Route::AuthEmailExchange)
            .body(data)
            .send::<AccessToken>()
            .await?;

        Ok(Credentials::Token(token.access_token))
    }

    /// Link an external account. Requires an auth token from the external platform.
    ///
    /// See the [mod.io docs](https://docs.mod.io/#link-external-account) for more information.
    pub async fn link(self, email: &str, service: Service) -> Result<()> {
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
            .request(Route::LinkAccount)
            .body(data)
            .send::<ModioMessage>()
            .await?;

        Ok(())
    }

    /// Get the access token for an encrypted gog app ticket. [required: apikey]
    ///
    /// See the [mod.io docs](https://docs.mod.io/#authenticate-via-gog-galaxy) for more
    /// information.
    pub async fn gog_auth(self, ticket: &str) -> Result<Credentials> {
        let data = form_urlencoded::Serializer::new(String::new())
            .append_pair("appdata", ticket)
            .finish();

        let token = self
            .modio
            .request(Route::AuthGog)
            .body(data)
            .send::<AccessToken>()
            .await?;

        Ok(Credentials::Token(token.access_token))
    }

    /// Get the access token for an encrypted steam app ticket. [required: apikey]
    ///
    /// See the [mod.io docs](https://docs.mod.io/#authenticate-via-steam) for more information.
    pub async fn steam_auth(self, ticket: &str) -> Result<Credentials> {
        let data = form_urlencoded::Serializer::new(String::new())
            .append_pair("appdata", ticket)
            .finish();

        let token = self
            .modio
            .request(Route::AuthSteam)
            .body(data)
            .send::<AccessToken>()
            .await?;

        Ok(Credentials::Token(token.access_token))
    }

    /// Get the access token for an Oculus user. [required: apikey]
    ///
    /// See the [mod.io docs](https://docs.mod.io/#authenticate-via-oculus) for more information.
    pub async fn oculus_auth(self, options: OculusOptions) -> Result<Credentials> {
        let token = self
            .modio
            .request(Route::AuthOculus)
            .body(options.to_query_string())
            .send::<AccessToken>()
            .await?;

        Ok(Credentials::Token(token.access_token))
    }
}
