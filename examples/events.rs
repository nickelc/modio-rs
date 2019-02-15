use std::env;
use std::process;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use tokio::prelude::*;
use tokio::runtime::Runtime;
use tokio::timer::Interval;

use modio::error::Error;
use modio::filter::Operator;
use modio::EventListOptions;
use modio::QueryParams;
use modio::{auth::Credentials, Modio};

fn current_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

fn main() -> Result<(), Error> {
    dotenv::dotenv().ok();

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
    let mut rt = Runtime::new()?;

    // Creates a `Modio` endpoint for the test environment.
    let modio = Modio::host(
        host,
        concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION")),
        creds,
    );

    // Timestamp for the event filter
    let mut time = current_timestamp();

    // Creates an `Interval` task that yields every 10 seconds starting now.
    let task = Interval::new(Instant::now(), Duration::from_secs(10))
        .for_each(move |_| {
            // Create an event filter for `date_added` > time.
            let mut opts = EventListOptions::new();
            opts.date_added(Operator::GreaterThan, time);

            // Create the call for `/me/events` and wait for the result.
            let print = modio
                .me()
                .events(&opts)
                .collect()
                .and_then(move |list| {
                    println!("event filter: {}", opts.to_query_params());
                    println!("event count: {}", list.len());
                    println!("{:#?}", list);
                    Ok(())
                })
                .map_err(|e| println!("{:?}", e));

            rt.spawn(print);

            // Set a new timestamp for the next run.
            time = current_timestamp();
            Ok(())
        })
        .map_err(|e| panic!("interval errored; err={:?}", e));

    tokio::run(task);
    Ok(())
}
