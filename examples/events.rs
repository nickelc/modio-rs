use std::env;
use std::process;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
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

    // Creates an `Interval` task that yields every 10 seconds starting now.
    let task = Interval::new_interval(Duration::from_secs(10))
        .fold(current_timestamp(), move |tstamp, _| {
            // Create an event filter for `date_added` > time.
            let mut opts = EventListOptions::new();
            opts.date_added(Operator::GreaterThan, tstamp);

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

            // timestamp for the next run.
            Ok(current_timestamp())
        })
        .map(|_| ())
        .map_err(|e| panic!("interval errored; err={:?}", e));

    tokio::run(task);
    Ok(())
}
