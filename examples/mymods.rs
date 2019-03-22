use std::env;
use std::process;
use tokio::runtime::Runtime;

use modio::error::Error;
use modio::filter::prelude::*;
use modio::{auth::Credentials, Modio};

fn main() -> Result<(), Error> {
    dotenv::dotenv().ok();
    env_logger::init();

    // Fetch the access token / api key from the environment of the current process.
    let creds = match (env::var("MODIO_TOKEN"), env::var("MODIO_API_KEY")) {
        (Ok(token), _) => Credentials::Token(token),
        (_, Ok(apikey)) => Credentials::ApiKey(apikey),
        _ => {
            eprintln!("missing MODIO_TOKEN or MODIO_API_KEY environment variable");
            process::exit(1);
        }
    };
    let host = env::var("MODIO_HOST").unwrap_or_else(|_| "https://api.test.mod.io/v1".to_string());

    // tokio runtime to execute the modio futures.
    let mut rt = Runtime::new().expect("new rt");

    // Creates a `Modio` endpoint for the test environment.
    let modio = Modio::host(host, creds)?;

    // Create a mod filter for `id` in (1043, 1041), limited to 30 results
    // and ordered by `id` desc.
    let filter = Id::_in(vec![1043, 1041])
        .limit(30)
        .offset(0)
        .order_by(Id::desc());

    // Create the call for `/me/mods` and wait for the `ModioListResponse<Mod>`
    // result.
    for mod_ in rt.block_on(modio.me().mods().list(&filter))? {
        println!("{:#?}", mod_);
    }
    Ok(())
}
