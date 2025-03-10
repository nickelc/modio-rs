//! Authentication Flow interface
use std::collections::BTreeMap;
use std::fmt;

use crate::routing::Route;
use crate::types::auth::AccessToken;
use crate::types::{Message, Timestamp};
use crate::Modio;
use crate::Result;

pub use crate::types::auth::{Link, Links, Terms};

/// [mod.io](https://mod.io) credentials. API key with optional OAuth2 access token.
#[derive(Clone, Eq, PartialEq)]
pub struct Credentials {
    pub api_key: String,
    pub token: Option<Token>,
}

/// Access token and optional Unix timestamp of the date this token will expire.
#[derive(Clone, Eq, PartialEq)]
pub struct Token {
    pub value: String,
    pub expired_at: Option<Timestamp>,
}

impl fmt::Debug for Credentials {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.token.is_some() {
            f.write_str("Credentials(apikey+token)")
        } else {
            f.write_str("Credentials(apikey)")
        }
    }
}

impl Credentials {
    pub fn new<S: Into<String>>(api_key: S) -> Credentials {
        Credentials {
            api_key: api_key.into(),
            token: None,
        }
    }

    pub fn with_token<S: Into<String>, T: Into<String>>(api_key: S, token: T) -> Credentials {
        Credentials {
            api_key: api_key.into(),
            token: Some(Token {
                value: token.into(),
                expired_at: None,
            }),
        }
    }
}

impl From<&str> for Credentials {
    fn from(api_key: &str) -> Credentials {
        Credentials::new(api_key)
    }
}

impl From<(&str, &str)> for Credentials {
    fn from((api_key, token): (&str, &str)) -> Credentials {
        Credentials::with_token(api_key, token)
    }
}

impl From<String> for Credentials {
    fn from(api_key: String) -> Credentials {
        Credentials::new(api_key)
    }
}

impl From<(String, String)> for Credentials {
    fn from((api_key, token): (String, String)) -> Credentials {
        Credentials::with_token(api_key, token)
    }
}

/// Authentication Flow interface to retrieve access tokens. See the [mod.io Authentication
/// docs](https://docs.mod.io/restapiref/#email-exchange) for more information.
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
///     let modio = Modio::new(Credentials::new("api-key"))?;
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
#[derive(Clone)]
pub struct Auth {
    modio: Modio,
}

impl Auth {
    pub(crate) fn new(modio: Modio) -> Self {
        Self { modio }
    }

    /// Get text and links for user agreement and consent prior to authentication. [required: apikey]
    ///
    /// See the [mod.io docs](https://docs.mod.io/restapiref/#terms) for more information.
    pub async fn terms(self) -> Result<Terms> {
        self.modio.request(Route::Terms).send().await
    }

    /// Request a security code be sent to the email of the user. [required: apikey]
    pub async fn request_code(self, email: &str) -> Result<()> {
        self.modio
            .request(Route::OAuthEmailRequest)
            .form(&[("email", email)])
            .send::<Message>()
            .await?;

        Ok(())
    }

    /// Get the access token for a security code. [required: apikey]
    pub async fn security_code(self, code: &str) -> Result<Credentials> {
        let t = self
            .modio
            .request(Route::OAuthEmailExchange)
            .form(&[("security_code", code)])
            .send::<AccessToken>()
            .await?;

        let token = Token {
            value: t.value,
            expired_at: t.expired_at,
        };
        Ok(Credentials {
            api_key: self.modio.inner.credentials.api_key.clone(),
            token: Some(token),
        })
    }

    /// Authenticate via external services ([Steam], [Switch], [Xbox], [Discord], [Oculus], [Google]).
    ///
    /// See the [mod.io docs](https://docs.mod.io/restapiref/#authentication-2) for more information.
    ///
    /// [Steam]: SteamOptions
    /// [Oculus]: OculusOptions
    /// [Switch]: SwitchOptions
    /// [Xbox]: XboxOptions
    /// [Discord]: DiscordOptions
    /// [Google]: GoogleOptions
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use modio::{Credentials, Modio, Result};
    /// # #[tokio::main]
    /// # async fn run() -> Result<()> {
    /// #   let modio = modio::Modio::new("apikey")?;
    /// use modio::auth::SteamOptions;
    /// let opts = SteamOptions::new("ticket");
    /// modio.auth().external(opts).await?;
    /// #   Ok(())
    /// # }
    /// ```
    pub async fn external<T>(self, auth_options: T) -> Result<Credentials>
    where
        T: Into<AuthOptions>,
    {
        let AuthOptions { route, params } = auth_options.into();

        let t = self
            .modio
            .request(route)
            .form(&params)
            .send::<AccessToken>()
            .await?;

        let token = Token {
            value: t.value,
            expired_at: t.expired_at,
        };
        Ok(Credentials {
            api_key: self.modio.inner.credentials.api_key.clone(),
            token: Some(token),
        })
    }

    /// Logout by revoking the current access token.
    pub async fn logout(self) -> Result<()> {
        self.modio
            .request(Route::OAuthLogout)
            .send::<Message>()
            .await?;

        Ok(())
    }
}

/// Options for external authentication.
pub struct AuthOptions {
    route: Route,
    params: BTreeMap<&'static str, String>,
}

// impl From<*Options> for AuthOptions {{{
impl From<OculusOptions> for AuthOptions {
    fn from(options: OculusOptions) -> AuthOptions {
        AuthOptions {
            route: Route::ExternalAuthMeta,
            params: options.params,
        }
    }
}

impl From<SteamOptions> for AuthOptions {
    fn from(options: SteamOptions) -> AuthOptions {
        AuthOptions {
            route: Route::ExternalAuthSteam,
            params: options.params,
        }
    }
}

impl From<SwitchOptions> for AuthOptions {
    fn from(options: SwitchOptions) -> AuthOptions {
        AuthOptions {
            route: Route::ExternalAuthSwitch,
            params: options.params,
        }
    }
}

impl From<XboxOptions> for AuthOptions {
    fn from(options: XboxOptions) -> AuthOptions {
        AuthOptions {
            route: Route::ExternalAuthXbox,
            params: options.params,
        }
    }
}

impl From<DiscordOptions> for AuthOptions {
    fn from(options: DiscordOptions) -> AuthOptions {
        AuthOptions {
            route: Route::ExternalAuthDiscord,
            params: options.params,
        }
    }
}

impl From<GoogleOptions> for AuthOptions {
    fn from(options: GoogleOptions) -> AuthOptions {
        AuthOptions {
            route: Route::ExternalAuthGoogle,
            params: options.params,
        }
    }
}
// }}}

/// Authentication options for an encrypted gog app ticket.
///
/// See the [mod.io docs](https://docs.mod.io/restapiref/#gog-galaxy) for more information.
pub struct GalaxyOptions {
    params: BTreeMap<&'static str, String>,
}

impl GalaxyOptions {
    pub fn new<T>(ticket: T) -> Self
    where
        T: Into<String>,
    {
        let mut params = BTreeMap::new();
        params.insert("appdata", ticket.into());
        Self { params }
    }

    option!(email >> "email");
    option!(
        /// Unix timestamp of date in which the returned token will expire. Value cannot be higher
        /// than the default value which is a common year.
        expired_at u64 >> "date_expires"
    );
    option!(terms_agreed bool >> "terms_agreed");
}

/// Authentication options for an itch.io JWT token.
///
/// See the [mod.io docs](https://docs.mod.io/restapiref/#itch-io) for more information.
pub struct ItchioOptions {
    params: BTreeMap<&'static str, String>,
}

impl ItchioOptions {
    pub fn new<T>(token: T) -> Self
    where
        T: Into<String>,
    {
        let mut params = BTreeMap::new();
        params.insert("itchio_token", token.into());
        Self { params }
    }

    option!(email >> "email");
    option!(
        /// Unix timestamp of date in which the returned token will expire. Value cannot be higher
        /// than the default value which is a week.
        expired_at u64 >> "date_expires"
    );
    option!(terms_agreed bool >> "terms_agreed");
}

/// Authentication options for an Oculus user.
///
/// See the [mod.io docs](https://docs.mod.io/restapiref/#meta-quest) for more information.
pub struct OculusOptions {
    params: BTreeMap<&'static str, String>,
}

impl OculusOptions {
    pub fn new_for_quest<T>(nonce: T, user_id: u64, auth_token: T) -> Self
    where
        T: Into<String>,
    {
        OculusOptions::new("quest".to_owned(), nonce.into(), user_id, auth_token.into())
    }

    pub fn new_for_rift<T>(nonce: T, user_id: u64, auth_token: T) -> Self
    where
        T: Into<String>,
    {
        OculusOptions::new("rift".to_owned(), nonce.into(), user_id, auth_token.into())
    }

    fn new(device: String, nonce: String, user_id: u64, auth_token: String) -> Self {
        let mut params = BTreeMap::new();
        params.insert("device", device);
        params.insert("nonce", nonce);
        params.insert("user_id", user_id.to_string());
        params.insert("auth_token", auth_token);
        Self { params }
    }

    option!(email >> "email");
    option!(
        /// Unix timestamp of date in which the returned token will expire. Value cannot be higher
        /// than the default value which is a common year.
        expired_at u64 >> "date_expires"
    );
    option!(terms_agreed bool >> "terms_agreed");
}

/// Authentication options for an encrypted steam app ticket.
///
/// See the [mod.io docs](https://docs.mod.io/restapiref/#steam) for more information.
pub struct SteamOptions {
    params: BTreeMap<&'static str, String>,
}

impl SteamOptions {
    pub fn new<T>(ticket: T) -> Self
    where
        T: Into<String>,
    {
        let mut params = BTreeMap::new();
        params.insert("appdata", ticket.into());
        Self { params }
    }

    option!(email >> "email");
    option!(
        /// Unix timestamp of date in which the returned token will expire. Value cannot be higher
        /// than the default value which is a common year.
        expired_at u64 >> "date_expires"
    );
    option!(terms_agreed bool >> "terms_agreed");
}

/// Authentication options for the NSA ID token.
///
/// See the [mod.io docs](https://docs.mod.io/restapiref/#nintendo-switch) for more information.
pub struct SwitchOptions {
    params: BTreeMap<&'static str, String>,
}

impl SwitchOptions {
    pub fn new<T>(id_token: T) -> Self
    where
        T: Into<String>,
    {
        let mut params = BTreeMap::new();
        params.insert("id_token", id_token.into());
        Self { params }
    }

    option!(email >> "email");
    option!(
        /// Unix timestamp of date in which the returned token will expire. Value cannot be higher
        /// than the default value which is a common year.
        expired_at u64 >> "date_expires"
    );
    option!(terms_agreed bool >> "terms_agreed");
}

/// Authentication options for an Xbox Live token.
///
/// See the [mod.io docs](https://docs.mod.io/restapiref/#xbox-live) for more information.
pub struct XboxOptions {
    params: BTreeMap<&'static str, String>,
}

impl XboxOptions {
    pub fn new<T>(token: T) -> Self
    where
        T: Into<String>,
    {
        let mut params = BTreeMap::new();
        params.insert("xbox_token", token.into());
        Self { params }
    }

    option!(email >> "email");
    option!(
        /// Unix timestamp of date in which the returned token will expire. Value cannot be higher
        /// than the default value which is a common year.
        expired_at u64 >> "date_expires"
    );
    option!(terms_agreed bool >> "terms_agreed");
}

/// Authentication options for an Discord token.
///
/// See the [mod.io docs](https://docs.mod.io/restapiref/#discord) for more information.
pub struct DiscordOptions {
    params: BTreeMap<&'static str, String>,
}

impl DiscordOptions {
    pub fn new<T>(token: T) -> Self
    where
        T: Into<String>,
    {
        let mut params = BTreeMap::new();
        params.insert("discord_token", token.into());
        Self { params }
    }

    option!(email >> "email");
    option!(
        /// Unix timestamp of date in which the returned token will expire. Value cannot be higher
        /// than the default value which is a week.
        expired_at u64 >> "date_expires"
    );
    option!(terms_agreed bool >> "terms_agreed");
}

/// Authentication options for an Google token.
///
/// See the [mod.io docs](https://docs.mod.io/restapiref/#google) for more information.
pub struct GoogleOptions {
    params: BTreeMap<&'static str, String>,
}

impl GoogleOptions {
    pub fn new<T>(token: T) -> Self
    where
        T: Into<String>,
    {
        let mut params = BTreeMap::new();
        params.insert("id_token", token.into());
        Self { params }
    }

    option!(email >> "email");
    option!(
        /// Unix timestamp of date in which the returned token will expire. Value cannot be higher
        /// than the default value which is a week.
        expired_at u64 >> "date_expires"
    );
    option!(terms_agreed bool >> "terms_agreed");
}

// vim: fdm=marker
