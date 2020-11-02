use std::env;
use std::process;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use tokio::time::{self, Instant};

use modio::filter::prelude::*;
use modio::{auth::Credentials, Modio};

const TEN_SECS: Duration = Duration::from_secs(10);

fn current_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt::init();

    // Fetch the access token & api key from the environment of the current process.
    let creds = match (env::var("MODIO_TOKEN"), env::var("MODIO_API_KEY")) {
        (Ok(token), Ok(apikey)) => Credentials::with_token(apikey, token),
        _ => {
            eprintln!("missing MODIO_TOKEN and MODIO_API_KEY environment variable");
            process::exit(1);
        }
    };
    let host = env::var("MODIO_HOST").unwrap_or_else(|_| "https://api.test.mod.io/v1".to_string());

    // Creates a `Modio` endpoint for the test environment.
    let modio = Modio::host(host, creds)?;

    // Creates an `Interval` that yields every 10 seconds starting in 10 seconds.
    let mut interval = time::interval_at(Instant::now() + TEN_SECS, TEN_SECS);

    loop {
        let tstamp = current_timestamp();
        interval.tick().await;

        // Create an event filter for `date_added` > time.
        let filter = DateAdded::gt(tstamp);
        println!("event filter: {}", filter);

        let list: Vec<_> = modio.user().events(filter).collect().await?;

        println!("event count: {}", list.len());
        println!("{:#?}", list);
    }
}
