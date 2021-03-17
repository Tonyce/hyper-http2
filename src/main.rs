mod proto;
// use bytes::BufMut;
// use bytes::{Bytes, BytesMut};
// use futures::stream::TryStreamExt;
use byteorder::{BigEndian, ByteOrder};
use bytes::BufMut;
use hyper::body;
use hyper::service::{make_service_fn, service_fn};
use hyper::HeaderMap;
use hyper::{Body, Request, Response, Server, Version};
use prost::Message;
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
    let service = make_service_fn(|_conn| async {
        Ok::<_, hyper::Error>(service_fn(|req: Request<Body>| async move {
            if req.version() == Version::HTTP_2 {
                let (_parts, body) = req.into_parts();
                // 获取 body
                let body_buf = body::to_bytes(body).await.unwrap();
                let compressed_flag = &body_buf[0..1];
                let proto_len = &body_buf[1..5];
                let proto_buf = &body_buf[5..];

                println!(
                    "{:?}, {:?}, {:?}",
                    compressed_flag,
                    proto_len,
                    proto_buf.len()
                );

                let flags = BigEndian::read_uint(compressed_flag, 1);
                let len = BigEndian::read_uint(proto_len, 4);
                let body_len = proto_buf.len() as u64;
                // len should be equal body_len
                println!(
                    "flags {}, len {:?}, proto_body_len: {}",
                    flags, len, body_len
                );

                let _pp = proto::helloworld::HelloRequest::decode(proto_buf).unwrap();
                // println!("{:?}", pp);

                // reply body
                let hello_reply = proto::helloworld::HelloReply {
                    message: "haha".to_owned(),
                };
                let mut reply_buf: Vec<u8> = [].to_vec();
                hello_reply.encode(&mut reply_buf).unwrap();
                let reply_len = reply_buf.len() as u32;

                let mut len_buf = [0; 4];
                BigEndian::write_u32(&mut len_buf, reply_len);
                println!("{:?}", reply_len);

                let mut reply_body_buf = vec![0]; // 0 位是 compressed_flag
                reply_body_buf.put(&len_buf[..]);
                reply_body_buf.put(&reply_buf[..]);
                println!("{:?}", reply_body_buf);

                let (mut sender, body) = Body::channel();
                let res = Response::builder().header("Foo", "Bar").body(body).unwrap();
                sender.send_data(reply_body_buf.into()).await.unwrap();

                let mut trailers = HeaderMap::new();
                trailers.insert("grpc-status", "0".parse().unwrap());
                trailers.insert("grpc-message", "OK".parse().unwrap());
                sender.send_trailers(trailers).await.unwrap();

                Ok(res)
            } else {
                // Note: it's usually better to return a Response
                // with an appropriate StatusCode instead of an Err.
                Err("not HTTP/2, abort connection")
            }
        }))
    });

    // let server = Server::bind(&addr).serve(make_svc);
    let server = Server::bind(&addr).serve(service);
    println!("Listening on http://{}", addr);
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
