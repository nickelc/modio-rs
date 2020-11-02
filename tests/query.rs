use futures_core::Stream;
use futures_util::TryStreamExt;
use httptest::{matchers::*, responders::*};
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

fn create_empty_result() -> Server {
    let server = Server::run();

    expect_requests!(
        server,
        query: any(),
        body: r#"{"data":[],"result_count":0,"result_offset":0,"result_limit":100,"result_total":0}"#
    );

    server
}

fn create_first_page_only() -> Server {
    let server = Server::run();

    expect_requests!(
        server,
        query: not(contains(key("_offset"))),
        body:  include_str!("fixtures/games-page1.json")
    );

    server
}

fn create_games_endpoint() -> Server {
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

    server
}

#[tokio::test]
async fn empty_first() -> Result<()> {
    let server = create_empty_result();

    let modio = Modio::host(server.url_str("/v1"), "foobar")?;
    let first = modio.games().search(Default::default()).first().await?;

    assert!(first.is_none());
    Ok(())
}

#[tokio::test]
async fn first() -> Result<()> {
    let server = create_first_page_only();

    let modio = Modio::host(server.url_str("/v1"), "foobar")?;
    let first = modio.games().search(Default::default()).first().await?;

    assert!(first.is_some());
    assert_eq!(1, first.unwrap().id);
    Ok(())
}

#[tokio::test]
async fn empty_first_page() -> Result<()> {
    let server = create_empty_result();

    let modio = Modio::host(server.url_str("/v1"), "foobar")?;
    let filter = Default::default();
    let list = modio.games().search(filter).first_page().await?;

    assert!(list.is_empty());
    Ok(())
}

#[tokio::test]
async fn first_page() -> Result<()> {
    let server = create_first_page_only();

    let modio = Modio::host(server.url_str("/v1"), "foobar")?;
    let filter = Default::default();
    let list = modio.games().search(filter).first_page().await?;

    assert_eq!(7, list.len());
    assert_eq!(1, list[0].id);
    assert_eq!(51, list[6].id);
    Ok(())
}

#[tokio::test]
async fn empty_collect() -> Result<()> {
    let server = create_empty_result();

    let modio = Modio::host(server.url_str("/v1"), "foobar")?;
    let list = modio.games().search(Default::default()).collect().await?;

    assert!(list.is_empty());
    Ok(())
}

#[tokio::test]
async fn collect() -> Result<()> {
    let server = create_games_endpoint();

    let modio = Modio::host(server.url_str("/v1"), "foobar")?;
    let list = modio.games().search(Default::default()).collect().await?;

    assert_eq!(31, list.len());
    assert_eq!(1, list[0].id);
    assert_eq!(296, list[30].id);
    Ok(())
}

#[tokio::test]
async fn empty_paged() -> Result<()> {
    let server = create_empty_result();

    let modio = Modio::host(server.url_str("/v1"), "foobar")?;
    let mut st = modio.games().search(Default::default()).paged().await?;

    assert_eq!((0, None), st.size_hint());
    assert!(st.try_next().await?.is_none());
    Ok(())
}

#[tokio::test]
async fn paged() -> Result<()> {
    let server = create_games_endpoint();

    let modio = Modio::host(server.url_str("/v1"), "foobar")?;
    let filter = with_limit(7);

    let mut iter = modio.games().search(filter).paged().await?;
    let mut total = 0;
    let mut count = 0;
    // First & Last Game ID's of every loaded page result
    let mut ids = vec![(1, 51), (63, 152), (164, 214), (224, 251), (255, 296)].into_iter();
    let size_hint = iter.size_hint();

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
    assert_eq!((count, None), size_hint);
    Ok(())
}

#[tokio::test]
async fn iter() -> Result<()> {
    let server = create_games_endpoint();

    let modio = Modio::host(server.url_str("/v1"), "foobar")?;
    let filter = with_limit(7);

    let mut iter = modio.games().search(filter).iter().await?;
    let mut count = 0;
    let size_hint = iter.size_hint();

    while let Ok(Some(_)) = iter.try_next().await {
        count += 1;
    }

    assert_eq!(count, 31);
    assert_eq!((count, None), size_hint);
    Ok(())
}
