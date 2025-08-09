mod email_exchange;
mod email_request;
mod external_auth;
mod get_terms;
mod logout;

/// External authentication providers for [`ExternalAuth`].
pub mod external {
    pub use super::external_auth::{
        Discord, EpicGames, Google, MetaQuest, OpenID, Steam, Switch, Xbox, PSN,
    };

    pub(crate) use super::external_auth::Provider;
}

pub use email_exchange::EmailExchange;
pub use email_request::EmailRequest;
pub use external_auth::ExternalAuth;
pub use get_terms::GetTerms;
pub use logout::Logout;
