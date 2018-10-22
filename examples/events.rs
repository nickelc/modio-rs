extern crate modio;
extern crate tokio;

use std::env;
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
    // Fetch the access token from the environment of the current process.
    match env::var("MODIO_TOKEN").ok() {
        Some(token) => {
            // tokio runtime to execute the modio futures.
            let mut rt = Runtime::new()?;

            // Creates a `Modio` endpoint for the test environment.
            let modio = Modio::host(
                "https://api.test.mod.io/v1",
                concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION")),
                Credentials::Token(token),
            );

            // Timestamp for the event filter
            let mut time = current_timestamp();

            // Creates an `Interval` task that yields every 10 seconds starting now.
            let task = Interval::new(Instant::now(), Duration::from_secs(10))
                .for_each(move |_| {
                    // Create an event filter for `date_added` > time.
                    let mut opts = EventListOptions::new();
                    opts.date_added(Operator::GreaterThan, time);

                    // Create the call for `/me/events` and wait for the `ModioListResponse<Event>`
                    // result.
                    let result = rt.block_on(modio.me().events(&opts));

                    match result {
                        Ok(list) => {
                            println!("event filter: {}", opts.to_query_params());
                            println!("event count: {}", list.count);
                            println!("{:#?}", list.data);
                        }
                        Err(e) => println!("modio error: {:?}", e),
                    }

                    // Set a new timestamp for the next run.
                    time = current_timestamp();
                    Ok(())
                }).map_err(|e| panic!("interval errored; err={:?}", e));

            tokio::run(task);
            Ok(())
        }
        _ => Err("missing MODIO_TOKEN".into()),
    }
}
