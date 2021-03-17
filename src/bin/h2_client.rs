use hyper::{body::HttpBody, Body, Client, Uri};
use std::time::Duration;
#[tokio::main]
async fn main() {
    // let client = Client::new();
    let client = Client::builder()
        .pool_idle_timeout(Duration::from_secs(30))
        .http2_only(true)
        .build_http::<Body>();

    // Make a GET /ip to 'http://httpbin.org'
    let res = client
        .get(Uri::from_static("http://127.0.0.1:4567"))
        .await
        .unwrap();

    // And then, if the request gets a response...
    println!("status: {}", res.status());

    // Concatenate the body stream into a single buffer...
    let buf = hyper::body::to_bytes(res).await.unwrap();

    println!("body: {:?}", buf);
}
