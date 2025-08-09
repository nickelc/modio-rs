use std::env;
use std::io::{self, Write};
use std::process;

use modio::types::id::Id;
use modio::Client;

fn prompt(prompt: &str) -> io::Result<u64> {
    print!("{}", prompt);
    io::stdout().flush()?;
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    Ok(buffer.trim().parse().expect("Invalid value"))
}

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

    let game_id = Id::new(prompt("Enter game id: ")?);
    let mod_id = Id::new(prompt("Enter mod id: ")?);

    // Create the call for `/games/{game_id}/mods/{mod_id}` and wait for the result.
    let resp = client.get_mod(game_id, mod_id).await?;
    let m = resp.data().await?;
    if let Some(file) = m.modfile {
        // Download the file and calculate its md5 digest.
        let mut ctx = md5::Context::new();
        let mut size = 0;

        println!("mod: {}", m.name);
        println!("url: {}", file.download.binary_url);
        println!("filename: {}", file.filename);
        println!("filesize: {}", file.filesize);
        println!("reported md5: {}", file.filehash.md5);

        let mut chunked = client.download(file).chunked().await?;
        while let Some(bytes) = chunked.data().await {
            let bytes = bytes?;
            size += bytes.len();
            ctx.consume(bytes);
        }

        println!("computed md5: {:x}", ctx.finalize());
        println!("downloaded size: {}", size);
    } else {
        println!("The mod has no files.");
    }
    Ok(())
}
