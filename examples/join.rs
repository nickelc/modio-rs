use std::env;
use std::process;

use futures_util::future::try_join3;
use modio::{auth::Credentials, Modio};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    env_logger::init();

    // Fetch the access token / api key from the environment of the current process.
    let creds = match (env::var("MODIO_TOKEN"), env::var("MODIO_API_KEY")) {
        (Ok(token), _) => Credentials::Token(token, None),
        (_, Ok(apikey)) => Credentials::ApiKey(apikey),
        _ => {
            eprintln!("missing MODIO_TOKEN or MODIO_API_KEY environment variable");
            process::exit(1);
        }
    };
    let host = env::var("MODIO_HOST").unwrap_or_else(|_| "https://api.test.mod.io/v1".to_string());

    // Creates a `Modio` endpoint for the test environment.
    let modio = Modio::host(host, creds)?;

    // OpenXcom: The X-Com Files
    let modref = modio.mod_(51, 158);

    // Get mod with its dependencies and all files
    let deps = modref.dependencies().list();
    let files = modref.files().list(Default::default());
    let mod_ = modref.get();

    let (m, deps, files) = try_join3(mod_, deps, files).await?;

    println!("{}", m.name);
    println!(
        "deps: {:?}",
        deps.into_iter().map(|d| d.mod_id).collect::<Vec<_>>()
    );
    for file in files {
        println!("file id: {} version: {:?}", file.id, file.version);
    }
    Ok(())
}
