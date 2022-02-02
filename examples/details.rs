use std::env;
use std::process;

use modio::filter::Filter;
use modio::{auth::Credentials, Modio};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt::init();

    // Fetch the access token / api key from the environment of the current process.
    let creds = match (env::var("MODIO_TOKEN"), env::var("MODIO_API_KEY")) {
        (Ok(token), Ok(apikey)) => Credentials::with_token(apikey, token),
        (_, Ok(apikey)) => Credentials::new(apikey),
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
    let deps = modref.dependencies().list().await?;
    let files = modref.files().search(Filter::default()).collect().await?;
    let m = modref.get().await?;

    println!("{}, {}\n", m.name, m.profile_url);
    println!(
        "deps: {:?}",
        deps.into_iter().map(|d| d.mod_id).collect::<Vec<_>>()
    );
    println!(
        "stats: downloads={} subscribers={}\n",
        m.stats.downloads_total, m.stats.subscribers_total,
    );
    let primary = m.modfile.as_ref().map(|f| f.id).unwrap_or_default();
    println!("files:");
    for file in files {
        let primary = if primary == file.id { "*" } else { " " };
        println!("{} id: {} version: {:?}", primary, file.id, file.version);
    }
    Ok(())
}
