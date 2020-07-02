use hyper::{Client, Uri};

// https://stackoverflow.com/a/14189088/3818139
#[tokio::main]
async fn main() {
    let client = Client::new();

    let url: Uri = "http://httpbin.org/response-headers?foo=bar"
        .parse()
        .unwrap();
    assert_eq!(url.query(), Some("foo=bar"));

    match client.get(url).await {
        Ok(res) => println!("Response: {}", res.status()),
        Err(err) => println!("Error: {}", err),
    }
}
