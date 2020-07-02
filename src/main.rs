use hyper::{Uri, Client};
use hyper_tls::HttpsConnector;

#[tokio::main]
async fn main() {
    let https = HttpsConnector::new();
    let client = Client::builder()
        .build::<_, hyper::Body>(https);

    let url = Uri::from_static("https://api.duckduckgo.com/?q=apple&format=json&t=7roxy");
    assert_eq!(url.scheme_str(), Some("https"));

    match client.get(url).await {
        Ok(res) => println!("Response: {:?}", res),
        Err(err) => println!("Error: {}", err),
    }
}
