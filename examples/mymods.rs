extern crate modio;
extern crate tokio;

use std::env;
use tokio::runtime::Runtime;

use modio::errors::Error;
use modio::{Credentials, Modio};
use modio::mods::{ModsListOptions};
use modio::filter::{Operator, Order};

fn main() -> Result<(), Error> {
    match env::var("MODIO_TOKEN").ok() {
        Some(token) => {
            let mut rt = Runtime::new()?;
            let modio = Modio::host(
                "https://api.test.mod.io/v1",
                concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION")),
                Credentials::Token(token),
            );

            let mut opts = ModsListOptions::new();
            opts.id(Operator::In, vec![1043, 1041]);
            // opts.limit(30);
            // opts.offset(20);
            opts.sort_by(ModsListOptions::ID, Order::Desc);

            for mod_ in rt.block_on(modio.me().mods().list(&opts))? {
                println!("{:#?}", mod_);
            }
            Ok(())
        }
        _ => Err("missing MODIO_TOKEN".into()),
    }
}
