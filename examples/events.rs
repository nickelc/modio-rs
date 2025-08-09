use std::env;
use std::process;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use tokio::time::{self, Instant};

use modio::request::filter::prelude::*;
use modio::Client;

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
    let (api_key, token) = match (env::var("MODIO_API_KEY"), env::var("MODIO_TOKEN")) {
        (Ok(api_key), Ok(token)) => (api_key, token),
        _ => {
            eprintln!("missing MODIO_TOKEN and MODIO_API_KEY environment variable");
            process::exit(1);
        }
    };
    let host = env::var("MODIO_HOST").unwrap_or_else(|_| "api.test.mod.io".to_string());

    let client = Client::builder(api_key).token(token).host(host).build()?;

    // Creates an `Interval` that yields every 10 seconds starting in 10 seconds.
    let mut interval = time::interval_at(Instant::now() + TEN_SECS, TEN_SECS);

    loop {
        let tstamp = current_timestamp();
        interval.tick().await;

        // Create an event filter for `date_added` > time.
        let filter = DateAdded::gt(tstamp);
        println!("event filter: {}", filter);

        let list = client
            .get_user_events()
            .filter(filter)
            .await?
            .data()
            .await?;

        println!("event count: {}", list.data.len());
        println!("{:#?}", list);
    }
}
