#![deny(warnings)]

use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Client, Request, Response, Server};
use std::{convert::Infallible, net::SocketAddr};

async fn hello(mut req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    println!("--- {:?}", req.headers());
    // let body_buf = hyper::body::to_bytes(req.into_body()).await.unwrap();
    let client = Client::builder()
        // .pool_idle_timeout(Duration::from_secs(30))
        .http2_only(true)
        .build_http::<Body>();
    let out_addr: SocketAddr = ([127, 0, 0, 1], 4567).into();
    // let out_addr_clone = out_addr.clone();
    // Ok(Response::new(Body::from("Hello World!")))
    let uri_string = format!(
        "http://{}{}",
        out_addr,
        req.uri()
            .path_and_query()
            .map(|x| x.as_str())
            .unwrap_or("/")
    );
    println!("{}", uri_string);
    let uri = uri_string.parse().unwrap();
    *req.uri_mut() = uri;
    // let header = req.headers();
    // println!("{:?}", header);
    client.request(req).await
}

#[tokio::main]
async fn main() {
    // pretty_env_logger::init();

    let in_addr = ([127, 0, 0, 1], 3001).into();
    let out_addr: SocketAddr = ([127, 0, 0, 1], 50051).into();

    let out_addr_clone = out_addr.clone();
    let client_main = Client::builder()
        // .pool_idle_timeout(Duration::from_secs(30))
        .http2_only(true)
        .build_http::<Body>();
    // The closure inside `make_service_fn` is run for each connection,
    // creating a 'service' to handle requests for that specific connection.
    let make_service = make_service_fn(|_| async move {
        let client = client_main.clone();

        // let remote_addr = socket.remote_addr();
        async move {
            Ok::<_, Infallible>(service_fn(move |_: Request<Body>| async move {
                Ok::<_, Infallible>(Response::new(Body::from(format!(
                    "Hello, {}!",
                    "remote_addr"
                ))))
            }))
        }
    });

    let make_service = make_service_fn(|_conn| {
        // This is the `Service` that will handle the connection.
        // `service_fn` is a helper to convert a function that
        // returns a Response into a `Service`.
        async { Ok::<_, Infallible>(service_fn(hello)) }
    });

    let server = Server::bind(&in_addr).serve(make_service);

    println!("Listening on http://{}", in_addr);
    println!("Proxying on http://{}", out_addr);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
