use std::future::IntoFuture;

use serde::ser::Serialize;
use serde_derive::Serialize;

use crate::client::Client;
use crate::request::{Output, RequestBuilder, Route};
use crate::response::ResponseFuture;
use crate::types::auth::AccessToken;

/// Request an access token for currently supported providers.
pub struct ExternalAuth<'a, T> {
    http: &'a Client,
    fields: ExternalAuthFields<'a, T>,
}

#[derive(Serialize)]
struct ExternalAuthFields<'a, T> {
    #[serde(skip_serializing_if = "Option::is_none")]
    email: Option<&'a str>,
    #[serde(rename = "date_expires", skip_serializing_if = "Option::is_none")]
    expired_at: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    terms_agreed: Option<bool>,
    #[serde(flatten)]
    others: T,
}

#[derive(Serialize)]
pub struct Discord<'a> {
    #[serde(rename = "discord_token")]
    token: &'a str,
}

impl<'a> Discord<'a> {
    pub const fn new(token: &'a str) -> Self {
        Self { token }
    }
}

#[derive(Serialize)]
pub struct EpicGames<'a> {
    #[serde(rename = "id_token")]
    token: &'a str,
}

impl<'a> EpicGames<'a> {
    pub const fn new(token: &'a str) -> Self {
        Self { token }
    }
}

#[derive(Serialize)]
pub struct Google<'a> {
    #[serde(rename = "id_token")]
    token: &'a str,
}

impl<'a> Google<'a> {
    pub const fn new(token: &'a str) -> Self {
        Self { token }
    }
}

#[derive(Serialize)]
pub struct MetaQuest<'a> {
    device: &'a str,
    nonce: &'a str,
    user_id: u64,
    access_token: &'a str,
}

impl<'a> MetaQuest<'a> {
    pub const fn new(device: &'a str, nonce: &'a str, user_id: u64, access_token: &'a str) -> Self {
        Self {
            device,
            nonce,
            user_id,
            access_token,
        }
    }
}

#[derive(Serialize)]
pub struct OpenID<'a> {
    #[serde(rename = "id_token")]
    token: &'a str,
}

impl<'a> OpenID<'a> {
    pub fn new(token: &'a str) -> Self {
        Self { token }
    }
}

#[derive(Serialize)]
pub struct PSN<'a> {
    auth_code: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    env: Option<u32>,
}

impl<'a> PSN<'a> {
    pub fn new(auth_code: &'a str) -> Self {
        Self {
            auth_code,
            env: None,
        }
    }
}

#[derive(Serialize)]
pub struct Steam<'a> {
    appdata: &'a str,
}

impl<'a> Steam<'a> {
    pub const fn new(appdata: &'a str) -> Self {
        Self { appdata }
    }
}

#[derive(Serialize)]
pub struct Switch<'a> {
    #[serde(rename = "id_token")]
    token: &'a str,
}

impl<'a> Switch<'a> {
    pub const fn new(token: &'a str) -> Self {
        Self { token }
    }
}

#[derive(Serialize)]
pub struct Xbox<'a> {
    #[serde(rename = "xbox_token")]
    token: &'a str,
}

impl<'a> Xbox<'a> {
    pub const fn new(token: &'a str) -> Self {
        Self { token }
    }
}

impl<'a, T> ExternalAuth<'a, T> {
    pub(crate) const fn new(http: &'a Client, others: T) -> Self {
        Self {
            http,
            fields: ExternalAuthFields {
                email: None,
                expired_at: None,
                terms_agreed: None,
                others,
            },
        }
    }

    pub const fn email(mut self, email: &'a str) -> Self {
        self.fields.email = Some(email);
        self
    }

    pub const fn expired_at(mut self, expired_at: u64) -> Self {
        self.fields.expired_at = Some(expired_at);
        self
    }

    pub const fn terms_agreed(mut self, terms_agreed: bool) -> Self {
        self.fields.terms_agreed = Some(terms_agreed);
        self
    }
}

impl<'a> ExternalAuth<'a, PSN<'a>> {
    pub const fn env(mut self, env: u32) -> Self {
        self.fields.others.env = Some(env);
        self
    }
}

pub trait Provider {
    fn route() -> Route;
}

macro_rules! impl_provider {
    ($($Type:ident: $route:expr)*) => {
        $(impl<'a> Provider for $Type<'a> {
            fn route() -> Route {
                $route
            }
        })*
    };
}

impl_provider! {
    Discord: Route::ExternalAuthDiscord
    EpicGames: Route::ExternalAuthEpic
    Google: Route::ExternalAuthGoogle
    MetaQuest: Route::ExternalAuthMeta
    OpenID: Route::ExternalAuthOpenID
    PSN: Route::ExternalAuthPSN
    Steam: Route::ExternalAuthSteam
    Switch: Route::ExternalAuthSwitch
    Xbox: Route::ExternalAuthXbox
}

impl<T: Serialize + Provider> IntoFuture for ExternalAuth<'_, T> {
    type Output = Output<AccessToken>;
    type IntoFuture = ResponseFuture<AccessToken>;

    fn into_future(self) -> Self::IntoFuture {
        let route = T::route();
        match RequestBuilder::from_route(&route).form(&self.fields) {
            Ok(req) => self.http.request(req),
            Err(err) => ResponseFuture::failed(err),
        }
    }
}
