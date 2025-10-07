use std::env;
use std::path::PathBuf;
use std::process;
use std::str::FromStr;

use modio::types::files::multipart::UploadSession;
use modio::util::upload::{self, ContentRange};
use modio::util::DataFromRequest;
use modio::Client;
use tokio::fs::File;
use tokio::io::{AsyncReadExt, BufReader};
use tokio_util::io::ReaderStream;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt::init();

    let mut it = env::args().skip(1);

    fn parse_next<T: FromStr, It: Iterator<Item = String>>(it: &mut It) -> Option<T> {
        it.next().and_then(|s| s.parse().ok())
    }

    let game_id = parse_next(&mut it).expect("GameId expected");
    let mod_id = parse_next(&mut it).expect("ModId expected");
    let modfile: PathBuf = parse_next(&mut it).expect("Mod file expected");

    // Fetch the access token / api key from the environment of the current process.
    let client = match (env::var("MODIO_API_KEY"), env::var("MODIO_TOKEN")) {
        (Ok(api_key), Ok(token)) => Client::builder(api_key).token(token),
        (Ok(api_key), _) => Client::builder(api_key),
        _ => {
            eprintln!("missing MODIO_TOKEN or MODIO_API_KEY environment variable");
            process::exit(1);
        }
    };
    let host = env::var("MODIO_HOST").unwrap_or_else(|_| "api.test.mod.io".to_string());

    // Creates a `Modio` endpoint for the test environment.
    let client = client.host(host).build()?;

    let UploadSession { id: upload_id, .. } = client
        .create_multipart_upload_session(game_id, mod_id, "modfile.zip")
        .data()
        .await?;

    let file = File::open(modfile).await?;
    let file_size = file.metadata().await?.len();

    for (start, end) in upload::byte_ranges(file_size) {
        let input = BufReader::new(file.try_clone().await?);
        let part = input.take(upload::MULTIPART_FILE_PART_SIZE);
        let stream = ReaderStream::new(part);

        let range = ContentRange {
            start,
            end,
            total: file_size,
        };

        // Add file part to the upload session.
        client
            .add_multipart_upload_part(game_id, mod_id, upload_id, range, stream)
            .await?;
    }

    // Complete the multipart upload session.
    client
        .complete_multipart_upload_session(game_id, mod_id, upload_id)
        .await?;

    // Finalize upload to the mod with file details.
    let modfile = client
        .add_multipart_upload_file(game_id, mod_id, upload_id)
        .active(true)
        .version("1.0")
        .data()
        .await?;

    dbg!(modfile);

    Ok(())
}
