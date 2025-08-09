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
//! - [Email Authentication Flow](Client::request_token) to create an OAuth 2 Access Token
//!   (Read + Write)
//! - [External Authentication](Client::external_auth) to create an OAuth 2 Access Token (Read + Write)
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
//! use modio::Client;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let modio = Client::builder(std::env::var("MODIO_API_KEY")?).build()?;
//!
//!     // create some tasks and execute them
//!     // let result = task.await?;
//!     Ok(())
//! }
//! ```
//!
//! For testing purposes use [`Builder::host`] to create a client for the
//! mod.io [test environment](https://docs.mod.io/restapiref/#testing).
//!
//! [`Builder::host`]: crate::client::Builder::host
//!
//! # Example: Chaining api requests
//!
//! ```no_run
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! #    let modio = modio::Client::builder(std::env::var("MODIO_API_KEY")?).build()?;
//! use modio::types::id::Id;
//!
//! // OpenXcom: The X-Com Files
//! let (game_id, mod_id) = (Id::new(51), Id::new(158));
//! let resp = modio.get_mod(game_id, mod_id).await?;
//! let mod_ = resp.data().await?;
//!
//! // Get mod with its dependencies and all files
//! let resp = modio.get_mod_dependencies(game_id, mod_id).await?;
//! let deps = resp.data().await?;
//!
//! let resp = modio.get_files(game_id, mod_id).await?;
//! let files = resp.data().await?;
//!
//! println!("{}", mod_.name);
//! println!(
//!     "deps: {:?}",
//!     deps.data.into_iter().map(|d| d.mod_id).collect::<Vec<_>>()
//! );
//! for file in files.data {
//!     println!("file id: {} version: {:?}", file.id, file.version);
//! }
//! #    Ok(())
//! # }
//! ```
//!
//! # Example: Downloading mods
//!
//! ```no_run
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! #    let modio = modio::Client::builder(std::env::var("MODIO_API_KEY")?).build()?;
//! use modio::client::download::{DownloadAction, ResolvePolicy};
//! use modio::types::id::Id;
//!
//! // Download the primary file of a mod.
//! let action = DownloadAction::Primary {
//!     game_id: Id::new(5),
//!     mod_id: Id::new(19),
//! };
//! modio.download(action).save_to_file("mod.zip").await?;
//!
//! // Download the specific file of a mod.
//! let action = DownloadAction::File {
//!     game_id: Id::new(5),
//!     mod_id: Id::new(19),
//!     file_id: Id::new(101),
//! };
//! modio.download(action).save_to_file("mod.zip").await?;
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
//! modio.download(action).save_to_file("mod.zip").await?;
//! #    Ok(())
//! # }
//! ```
#![doc(html_root_url = "https://docs.rs/modio/0.13.3")]
#![deny(rust_2018_idioms)]
#![deny(rustdoc::broken_intra_doc_links)]
#![allow(clippy::upper_case_acronyms)]
#![allow(
    clippy::module_name_repetitions,
    clippy::must_use_candidate,
    clippy::return_self_not_must_use,
    clippy::too_many_lines
)]

pub mod client;
pub mod request;
pub mod response;
pub mod types;

mod error;

pub use crate::client::Client;
pub use crate::error::{Error, Result};
