use std::env;
use std::io::{self, Write};
use std::process;

use futures_util::TryStreamExt;

use modio::{auth::Credentials, Modio};

fn prompt(prompt: &str) -> io::Result<u32> {
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

    let game_id = prompt("Enter game id: ")?;
    let mod_id = prompt("Enter mod id: ")?;

    // Create the call for `/games/{game_id}/mods/{mod_id}` and wait for the result.
    let m = modio.mod_(game_id, mod_id).get().await?;
    if let Some(file) = m.modfile {
        // Download the file and calculate its md5 digest.
        let mut ctx = md5::Context::new();
        let mut size = 0;

        println!("mod: {}", m.name);
        println!("url: {}", file.download.binary_url);
        println!("filename: {}", file.filename);
        println!("filesize: {}", file.filesize);
        println!("reported md5: {}", file.filehash.md5);

        let mut st = Box::pin(modio.download(file).await?.stream());
        while let Some(bytes) = st.try_next().await? {
            size += bytes.len();
            ctx.write_all(&bytes)?;
        }

        println!("computed md5: {:x}", ctx.compute());
        println!("downloaded size: {}", size);
    } else {
        println!("The mod has no files.");
    }
    Ok(())
}
