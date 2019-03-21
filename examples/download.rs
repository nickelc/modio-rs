use std::env;
use std::io::{self, Write};
use std::process;

use md5;
use tokio::runtime::Runtime;

use modio::error::Error;
use modio::{auth::Credentials, Modio};

fn prompt(prompt: &str) -> io::Result<u32> {
    print!("{}", prompt);
    io::stdout().flush()?;
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    Ok(buffer.trim().parse().expect("Invalid value"))
}

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

    let game_id = prompt("Enter game id: ").expect("read game id");
    let mod_id = prompt("Enter mod id: ").expect("read mod id");

    // Create the call for `/games/{game_id}/mods/{mod_id}` and wait for the result.
    let m = rt.block_on(modio.mod_(game_id, mod_id).get())?;
    if let Some(file) = m.modfile {
        // Download the file and calculate its md5 digest.
        let ctx = md5::Context::new();

        println!("mod: {}", m.name);
        println!("url: {}", file.download.binary_url);
        println!("filename: {}", file.filename);
        println!("filesize: {}", file.filesize);
        println!("reported md5: {}", file.filehash.md5);

        let (size, ctx) = rt.block_on(modio.download(file, ctx))?;
        println!("computed md5: {:x}", ctx.compute());
        println!("downloaded size: {}", size);
    } else {
        println!("The mod has no files.");
    }
    Ok(())
}
