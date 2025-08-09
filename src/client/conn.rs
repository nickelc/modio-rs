type HttpConnector = hyper_util::client::legacy::connect::HttpConnector;

#[cfg(all(
    feature = "native-tls",
    not(any(feature = "rustls-native-roots", feature = "rustls-webpki-roots"))
))]
type HttpsConnector<T> = hyper_tls::HttpsConnector<T>;

#[cfg(any(feature = "rustls-native-roots", feature = "rustls-webpki-roots"))]
type HttpsConnector<T> = hyper_rustls::HttpsConnector<T>;

#[cfg(any(
    feature = "native-tls",
    feature = "rustls-native-roots",
    feature = "rustls-webpki-roots"
))]
pub type Connector = HttpsConnector<HttpConnector>;

#[cfg(not(any(
    feature = "native-tls",
    feature = "rustls-native-roots",
    feature = "rustls-webpki-roots"
)))]
pub type Connector = HttpConnector;

pub fn create_connector() -> Connector {
    let mut connector = HttpConnector::new();
    connector.enforce_http(false);

    #[cfg(feature = "rustls-native-roots")]
    let connector = hyper_rustls::HttpsConnectorBuilder::new()
        .with_native_roots()
        .expect("failed to load native roots")
        .https_only()
        .enable_http1()
        .wrap_connector(connector);

    #[cfg(all(feature = "rustls-webpki-roots", not(feature = "rustls-native-roots")))]
    let connector = hyper_rustls::HttpsConnectorBuilder::new()
        .with_webpki_roots()
        .https_only()
        .enable_http1()
        .wrap_connector(connector);

    #[cfg(all(
        feature = "native-tls",
        not(any(feature = "rustls-native-roots", feature = "rustls-webpki-roots"))
    ))]
    let connector = hyper_tls::HttpsConnector::new_with_connector(connector);

    connector
}
