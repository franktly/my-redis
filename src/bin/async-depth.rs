use tokio::net::TcpStream;
use std::pin::Pin;
use std::task::{Context, Poll};


pub trait Future{
    type Output;

    fn poll(self:Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output>;
}

async fn my_async_fn()
{
    println!("hello from async");
    let _socket = TcpStream::connect("127.0.0.1:3000").await.unwrap();
    println!("async TCP operation complete");
}

#[tokio::main]
async fn main() {
    let fut = my_async_fn();
    fut.await;
}
