use serde::Deserialize;
use url::Url;

/// See the [Access Token Object](https://docs.mod.io/#access-token-object) docs for more
/// information.
#[derive(Deserialize)]
#[non_exhaustive]
pub struct AccessToken {
    #[serde(rename = "access_token")]
    pub value: String,
    #[serde(rename = "date_expires")]
    pub expired_at: Option<u64>,
}

/// See the [Terms Object](https://docs.mod.io/#terms-object) docs for more information.
#[derive(Debug, Deserialize)]
#[non_exhaustive]
pub struct Terms {
    pub plaintext: String,
    pub html: String,
    pub links: Links,
}

/// Part of [`Terms`]
///
/// See the [Terms Object](https://docs.mod.io/#terms-object) docs for more information.
#[derive(Debug, Deserialize)]
#[non_exhaustive]
pub struct Links {
    pub website: Link,
    pub terms: Link,
    pub privacy: Link,
    pub manage: Link,
}

/// Part of [`Terms`]
///
/// See the [Terms Object](https://docs.mod.io/#terms-object) docs for more information.
#[derive(Debug, Deserialize)]
#[non_exhaustive]
pub struct Link {
    pub text: String,
    pub url: Url,
    pub required: bool,
}
