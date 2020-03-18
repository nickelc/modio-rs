use futures_util::TryStreamExt;
use httptest::{mappers::*, responders::*};
use httptest::{Expectation, Server};

use modio::filter::prelude::*;
use modio::{Modio, Result};

macro_rules! expect_requests {
    ($server:expr, $(query:$query:expr, body:$body:expr),*) => {
        $(
            $server.expect(
                Expectation::matching(all_of![
                    request::method("GET"),
                    request::path("/v1/games"),
                    request::query(url_decoded($query)),
                ])
                .respond_with(status_code(200).body($body)),
            );
        )*
    };
}

#[tokio::test]
async fn empty() -> Result<()> {
    env_logger::try_init().ok();
    let server = Server::run();

    expect_requests!(
        server,
        query: any(),
        body: r#"{"data":[],"result_count":0,"result_offset":0,"result_limit":100,"result_total":0}"#
    );

    let modio = Modio::host(server.url_str("/v1"), "foobar")?;
    let list = modio.games().search(Default::default()).collect().await?;
    assert!(list.is_empty());

    Ok(())
}

#[tokio::test]
async fn bulk() -> Result<()> {
    env_logger::try_init().ok();
    let server = Server::run();

    expect_requests!(
        server,
        query: not(contains(key("_offset"))),
        body:  include_str!("fixtures/games-page1.json"),

        query: contains(("_offset", "7")),
        body:  include_str!("fixtures/games-page2.json"),

        query: contains(("_offset", "14")),
        body:  include_str!("fixtures/games-page3.json"),

        query: contains(("_offset", "21")),
        body:  include_str!("fixtures/games-page4.json"),

        query: contains(("_offset", "28")),
        body:  include_str!("fixtures/games-page5.json")
    );

    let modio = Modio::host(server.url_str("/v1"), "foobar")?;
    let filter = with_limit(7);

    let mut iter = modio.games().search(filter).bulk();
    let mut total = 0;
    let mut count = 0;
    // First & Last Game ID's of every loaded page result
    let mut ids = vec![(1, 51), (63, 152), (164, 214), (224, 251), (255, 296)].into_iter();

    while let Ok(Some(list)) = iter.try_next().await {
        if let Some((first, last)) = ids.next() {
            assert_eq!(list.first().map(|g| g.id), Some(first));
            assert_eq!(list.last().map(|g| g.id), Some(last));
        }
        count += 1;
        total += list.len();
    }

    assert_eq!(count, 5);
    assert_eq!(total, 31);
    Ok(())
}
