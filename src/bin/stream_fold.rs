use tokio_stream::{self as stream, *};

#[tokio::main]
async fn main() {
    let s = stream::iter(vec![1u8, 2, 3]);
    let sum = s.fold(0, |acc, x| acc + x).await;

    assert_eq!(sum, 6);
}
