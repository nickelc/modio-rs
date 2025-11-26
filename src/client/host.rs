use std::fmt;

use http::uri::Authority;

use crate::types::id::{GameId, UserId};

pub const DEFAULT_HOST: &str = "api.mod.io";
pub const TEST_HOST: &str = "api.test.mod.io";

#[derive(Clone, Default)]
pub enum Host {
    #[default]
    Default,
    Test,
    Dynamic,
    DynamicWithCustom(Authority),
    Game(GameId),
    User(UserId),
    Custom(Authority),
}

pub struct Display<'a> {
    host: &'a Host,
    game_id: Option<GameId>,
}

impl Host {
    pub fn display(&self, game_id: Option<GameId>) -> Display<'_> {
        Display {
            host: self,
            game_id,
        }
    }
}

impl fmt::Display for Display<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.host {
            Host::Default => f.write_str(DEFAULT_HOST),
            Host::Test => f.write_str(TEST_HOST),
            Host::Dynamic => {
                if let Some(game_id) = self.game_id {
                    f.write_fmt(format_args!("g-{game_id}.modapi.io"))
                } else {
                    f.write_str(DEFAULT_HOST)
                }
            }
            Host::DynamicWithCustom(host) => {
                if let Some(game_id) = self.game_id {
                    f.write_fmt(format_args!("g-{game_id}.modapi.io"))
                } else {
                    f.write_str(host.as_str())
                }
            }
            Host::Game(game_id) => f.write_fmt(format_args!("g-{game_id}.modapi.io")),
            Host::User(user_id) => f.write_fmt(format_args!("g-{user_id}.modapi.io")),
            Host::Custom(host) => f.write_str(host.as_str()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default() {
        let host = Host::Default;
        assert_eq!(DEFAULT_HOST, host.display(None).to_string());
        assert_eq!(DEFAULT_HOST, host.display(Some(GameId::new(1))).to_string());
    }

    #[test]
    fn test() {
        let host = Host::Test;
        assert_eq!(TEST_HOST, host.display(None).to_string());
        assert_eq!(TEST_HOST, host.display(Some(GameId::new(1))).to_string());
    }

    #[test]
    fn dynamic() {
        let host = Host::Dynamic;
        assert_eq!(DEFAULT_HOST, host.display(None).to_string());
        assert_eq!(
            "g-1.modapi.io",
            host.display(Some(GameId::new(1))).to_string()
        );
    }

    #[test]
    fn dynamic_with_custom() {
        let host = Host::DynamicWithCustom(Authority::from_static("custom"));
        assert_eq!("custom", host.display(None).to_string());
        assert_eq!(
            "g-1.modapi.io",
            host.display(Some(GameId::new(1))).to_string()
        );
    }

    #[test]
    fn custom() {
        let host = Host::Custom(Authority::from_static("custom"));
        assert_eq!("custom", host.display(None).to_string());
        assert_eq!("custom", host.display(Some(GameId::new(1))).to_string());
    }
}
