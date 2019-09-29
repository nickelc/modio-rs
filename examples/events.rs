use std::env;
use std::process;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use futures_util::{future, TryFutureExt, TryStreamExt};
use tokio::prelude::*;
use tokio::timer::Interval;

use modio::filter::prelude::*;
use modio::{auth::Credentials, Modio, Result};

fn current_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

#[tokio::main]
async fn main() -> Result<()> {
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

    // Creates an `Interval` task that yields every 10 seconds starting now.
    Interval::new_interval(Duration::from_secs(10))
        .fold(current_timestamp(), move |tstamp, _| {
            // Create an event filter for `date_added` > time.
            let filter = DateAdded::gt(tstamp);
            let qs = format!("{}", filter);

            // Create the call for `/me/events` and wait for the result.
            let print = modio
                .me()
                .events(filter)
                .try_collect()
                .and_then(move |list: Vec<_>| {
                    println!("event filter: {}", qs);
                    println!("event count: {}", list.len());
                    println!("{:#?}", list);
                    future::ok(())
                })
                .map_err(|e| println!("{:?}", e))
                .map(|_| ());

            tokio::spawn(print);

            // timestamp for the next run.
            future::ready(current_timestamp())
        })
        .await;

    Ok(())
}
