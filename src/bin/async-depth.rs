use tokio::net::TcpStream;
use std::task::{Context, Poll};
use std::future::Future;
use std::pin::Pin;
use std::time::{Duration, Instant};

/*
 * 
 * pub trait Future{
 *     type Output;
 * 
 *     fn poll(self:Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output>;
 * }
 * 
 */

async fn my_async_fn()
{
    println!("hello from async");
    let _socket = TcpStream::connect("127.0.0.1:3000").await.unwrap();
    println!("async TCP operation complete");
}

struct Delay {
    when: Instant,
}

impl Future for Delay{
    type Output = &'static str;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>)
        ->Poll<&'static str>
    {
        if Instant::now() >= self.when{
            println!("Hello World");
            Poll::Ready("done")
        }
        else{
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}

enum MainFuture{
    // Initialized, never polled
    State0,
    // Waiting on `Delay`, i.e. the `future.await`line
    State1(Delay),
    // The future has completed
    Terminated,
}

impl Future for MainFuture{
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>)
        -> Poll<()>
    {
        use MainFuture::*;
        loop {
            match *self {
                State0 => {
                    let when = Instant::now() + Duration::from_secs(3);
                    let fut = Delay{when};
                    *self = State1(fut); // state switch
                }
                State1(ref mut my_future)=> {
                    match Pin::new(my_future).poll(cx){
                        Poll::Ready(out) =>
                        {
                            *self = Terminated; // state switch 
                            return Poll::Ready(())
                        }
                        Poll::Pending =>{
                            return Poll::Pending;
                        }
                    }
                }
                Terminated => {
                    panic!("future polled after completion!!!")
                }
            }
        }
    }
}

async fn async_fn_test()
{
    let fut = my_async_fn();
    fut.await;
}
async fn delay_future_test()
{
    let when = Instant::now() + Duration::from_secs(3);
    let future = Delay{ when };
    future.await;
}

#[tokio::main]
async fn main() {

    async_fn_test().await;

    delay_future_test().await;
}
