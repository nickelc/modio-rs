use std::env;
use std::process;

use modio::types::id::Id;
use modio::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt::init();

    // Fetch the access token / api key from the environment of the current process.
    let client = match (env::var("MODIO_API_KEY"), env::var("MODIO_TOKEN")) {
        (Ok(api_key), Ok(token)) => Client::builder(api_key).token(token),
        (_, Ok(api_key)) => Client::builder(api_key),
        _ => {
            eprintln!("missing MODIO_TOKEN or MODIO_API_KEY environment variable");
            process::exit(1);
        }
    };
    let host = env::var("MODIO_HOST").unwrap_or_else(|_| "api.test.mod.io".to_string());

    // Creates a `Modio` endpoint for the test environment.
    let client = client.host(host).build()?;

    // OpenXcom: The X-Com Files
    let (game_id, mod_id) = (Id::new(51), Id::new(158));

    // Get mod with its dependencies and all files
    let resp = client.get_mod(game_id, mod_id).await?;
    let m = resp.data().await?;

    let resp = client.get_mod_dependencies(game_id, mod_id).await?;
    let deps = resp.data().await?;

    let resp = client.get_files(game_id, mod_id).await?;
    let files = resp.data().await?;

    println!("{}, {}\n", m.name, m.profile_url);
    println!(
        "deps: {:?}",
        deps.data.into_iter().map(|d| d.mod_id).collect::<Vec<_>>()
    );
    println!(
        "stats: downloads={} subscribers={}\n",
        m.stats.downloads_total, m.stats.subscribers_total,
    );
    let primary = m.modfile.as_ref().map(|f| f.id);
    println!("files:");
    for file in files.data {
        let primary = if primary == Some(file.id) { "*" } else { " " };
        println!("{} id: {} version: {:?}", primary, file.id, file.version);
    }
    Ok(())
}
