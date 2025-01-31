//! Modio provides a set of building blocks for interacting with the [mod.io](https://mod.io) API.
//!
//! The client uses asynchronous I/O, backed by the `futures` and `tokio` crates, and requires both
//! to be used alongside.
//!
//! # Authentication
//!
//! To access the API authentication is required and can be done via several ways:
//!
//! - Request an [API key (Read-only)](https://mod.io/me/access)
//! - Manually create an [OAuth 2 Access Token (Read + Write)](https://mod.io/me/access#oauth)
//! - [Email Authentication Flow](auth::Auth#example) to create an OAuth 2 Access Token (Read + Write)
//! - [External Authentication](auth::Auth::external) to create an OAuth 2 Access Token (Read + Write)
//!   automatically on platforms such as Steam, GOG, itch.io, Switch, Xbox, Discord and Oculus.
//!
//! # Rate Limiting
//!
//! - API keys linked to a game have **unlimited requests**.
//! - API keys linked to a user have **60 requests per minute**.
//! - OAuth2 user tokens are limited to **120 requests per minute**.
//!
//! [`Error::is_ratelimited`] will return true
//! if the rate limit associated with credentials has been exhausted.
//!
//! # Example: Basic setup
//!
//! ```no_run
//! use modio::{Credentials, Modio};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let modio = Modio::new(Credentials::new("user-or-game-api-key"))?;
//!
//!     // create some tasks and execute them
//!     // let result = task.await?;
//!     Ok(())
//! }
//! ```
//!
//! For testing purposes use [`Modio::host`] to create a client for the
//! mod.io [test environment](https://docs.mod.io/restapiref/#testing).
//!
//! # Example: Chaining api requests
//!
//! ```no_run
//! use futures_util::future::try_join3;
//! use modio::filter::Filter;
//! use modio::types::id::Id;
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! #    let modio = modio::Modio::new("user-or-game-api-key")?;
//!
//! // OpenXcom: The X-Com Files
//! let modref = modio.mod_(Id::new(51), Id::new(158));
//!
//! // Get mod with its dependencies and all files
//! let deps = modref.dependencies().list();
//! let files = modref.files().search(Filter::default()).collect();
//! let mod_ = modref.get();
//!
//! let (m, deps, files) = try_join3(mod_, deps, files).await?;
//!
//! println!("{}", m.name);
//! println!(
//!     "deps: {:?}",
//!     deps.into_iter().map(|d| d.mod_id).collect::<Vec<_>>()
//! );
//! for file in files {
//!     println!("file id: {} version: {:?}", file.id, file.version);
//! }
//! #    Ok(())
//! # }
//! ```
//!
//! # Example: Downloading mods
//!
//! ```no_run
//! use modio::download::{DownloadAction, ResolvePolicy};
//! use modio::types::id::Id;
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! #    let modio = modio::Modio::new("user-or-game-api-key")?;
//!
//! // Download the primary file of a mod.
//! let action = DownloadAction::Primary {
//!     game_id: Id::new(5),
//!     mod_id: Id::new(19),
//! };
//! modio
//!     .download(action)
//!     .await?
//!     .save_to_file("mod.zip")
//!     .await?;
//!
//! // Download the specific file of a mod.
//! let action = DownloadAction::File {
//!     game_id: Id::new(5),
//!     mod_id: Id::new(19),
//!     file_id: Id::new(101),
//! };
//! modio
//!     .download(action)
//!     .await?
//!     .save_to_file("mod.zip")
//!     .await?;
//!
//! // Download the specific version of a mod.
//! // if multiple files are found then the latest file is downloaded.
//! // Set policy to `ResolvePolicy::Fail` to return with
//! // `modio::download::Error::MultipleFilesFound` as source error.
//! let action = DownloadAction::Version {
//!     game_id: Id::new(5),
//!     mod_id: Id::new(19),
//!     version: "0.1".to_string(),
//!     policy: ResolvePolicy::Latest,
//! };
//! modio
//!     .download(action)
//!     .await?
//!     .save_to_file("mod.zip")
//!     .await?;
//! #    Ok(())
//! # }
//! ```
#![doc(html_root_url = "https://docs.rs/modio/0.12.0")]
#![deny(rust_2018_idioms)]
#![deny(rustdoc::broken_intra_doc_links)]
#![allow(clippy::upper_case_acronyms)]

#[macro_use]
mod macros;

pub mod auth;
#[macro_use]
pub mod filter;
pub mod comments;
pub mod download;
pub mod files;
pub mod games;
pub mod metadata;
pub mod mods;
pub mod reports;
pub mod teams;
pub mod types;
pub mod user;

mod client;
mod error;
mod file_source;
mod loader;
mod request;
mod routing;

pub use crate::auth::Credentials;
pub use crate::client::{Builder, Modio};
pub use crate::download::DownloadAction;
pub use crate::error::{Error, Result};
pub use crate::loader::{Page, Query};
pub use crate::types::{Deletion, Editing, TargetPlatform, TargetPortal};

mod prelude {
    pub use futures_util::Stream;
    pub use reqwest::multipart::Form;
    pub use reqwest::StatusCode;

    pub use crate::filter::Filter;
    pub use crate::loader::Query;
    pub use crate::routing::Route;
    pub use crate::types::Message;
    pub use crate::{Deletion, Editing, Modio, Result};
}

/// Re-exports of the used reqwest types.
#[doc(hidden)]
pub mod lib {
    pub use reqwest::header;
    pub use reqwest::redirect::Policy;
    pub use reqwest::ClientBuilder;
    #[cfg(feature = "__tls")]
    pub use reqwest::{Certificate, Identity};
    pub use reqwest::{Proxy, Url};
}
