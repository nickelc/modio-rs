use std::env;
use tokio::runtime::Runtime;

use modio::error::Error;
use modio::filter::{Operator, Order};
use modio::mods::ModsListOptions;
use modio::{auth::Credentials, Modio};

fn main() -> Result<(), Error> {
    // Fetch the access token from the environment of the current process.
    match env::var("MODIO_TOKEN").ok() {
        Some(token) => {
            // tokio runtime to execute the modio futures.
            let mut rt = Runtime::new()?;

            // Creates a `Modio` endpoint for the test environment.
            let modio = Modio::host(
                "https://api.test.mod.io/v1",
                concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION")),
                Credentials::Token(token),
            );

            // Create a mod filter for `id` in (1043, 1041), limited to 30 results
            // and ordered by `id` desc.
            let mut opts = ModsListOptions::new();
            opts.id(Operator::In, vec![1043, 1041]);
            opts.limit(30);
            opts.offset(0);
            opts.sort_by(ModsListOptions::ID, Order::Desc);

            // Create the call for `/me/mods` and wait for the `ModioListResponse<Mod>`
            // result.
            for mod_ in rt.block_on(modio.me().mods().list(&opts))? {
                println!("{:#?}", mod_);
            }
            Ok(())
        }
        _ => Err("missing MODIO_TOKEN".into()),
    }
}
