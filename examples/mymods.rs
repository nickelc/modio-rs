use std::env;
use std::process;

use modio::request::filter::prelude::*;
use modio::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt::init();

    // Fetch the access token / api key from the environment of the current process.
    let (api_key, token) = match (env::var("MODIO_API_KEY"), env::var("MODIO_TOKEN")) {
        (Ok(api_key), Ok(token)) => (api_key, token),
        _ => {
            eprintln!("missing MODIO_TOKEN and MODIO_API_KEY environment variable");
            process::exit(1);
        }
    };
    let host = env::var("MODIO_HOST").unwrap_or_else(|_| "api.test.mod.io".to_string());

    let client = Client::builder(api_key).token(token).host(host).build()?;

    // Create a mod filter for `id` in (1043, 1041), limited to 30 results
    // and ordered by `id` desc.
    let filter = Id::_in(vec![1043, 1041])
        .limit(30)
        .offset(0)
        .order_by(Id::desc());

    // Create the call for `/me/mods` and wait for the result.
    let list = client.get_user_mods().filter(filter).await?.data().await?;
    for mod_ in list.data {
        println!("{:#?}", mod_);
    }
    Ok(())
}
