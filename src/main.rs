// use bytes::BufMut;
// use bytes::{Bytes, BytesMut};
// use futures::stream::TryStreamExt;
use hyper::body;
use hyper::service::{make_service_fn, service_fn};
use hyper::HeaderMap;
use hyper::{Body, Request, Response, Server};
use std::{convert::Infallible, net::SocketAddr};

async fn handle(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    // let (_parts, body) = req.into_parts();
    // let entire_body = body
    //     .try_fold(Vec::new(), |mut data, chunk| async move {
    //         data.extend_from_slice(&chunk);
    //         Ok(data)
    //     })
    //     .await
    //     .unwrap();

    // println!("{}", entire_body.len());

    let bytes = body::to_bytes(req.into_body()).await.unwrap();
    println!("{}", bytes.len());

    // let mut builder = Response::builder();
    // builder.header("Foo", "Bar");
    // builder.body(());

    let (mut sender, body) = Body::channel();
    // let resp = Response::new()
    //     .with_header(ContentType("text/event-stream".parse().unwrap()))
    //     .with_header(CacheControl(vec![
    //         CacheDirective::NoStore,
    //         CacheDirective::Private,
    //     ]))
    //     .with_body(body);

    let res = Response::builder().header("Foo", "Bar").body(body).unwrap();
    sender
        .send_data(hyper::body::Bytes::from("test\n"))
        .await
        .unwrap();

    let mut trailers = HeaderMap::new();
    // 'grpc-status': '0',
    //   'grpc-message': 'OK',
    trailers.insert("grpc-status", "0".parse().unwrap());
    trailers.insert("grpc-message", "OK".parse().unwrap());
    sender.send_trailers(trailers).await.unwrap();

    // let res = Response::new("Hello, World!".into());
    Ok(res)
}

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    let make_svc = make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(handle)) });

    let server = Server::bind(&addr).serve(make_svc);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
