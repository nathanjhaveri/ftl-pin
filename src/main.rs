use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
    time::Duration,
};
use tokio::time::{Sleep, sleep};
use futures::FutureExt;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let fut = MyFuture::new();
    println!("Awaiting fut...");
    fut.await;
    println!("Awaiting fut... done!");
}

async fn greet() {
    println!("hello");
    sleep(Duration::from_millis(500)).await;
    println!("goodbye");
}

struct MyFuture {
    sleep: Pin<Box<Sleep>>,
}

impl Future for MyFuture {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        println!("MyFuture::poll()");
        self.sleep.poll_unpin(cx)
    }
}

impl MyFuture {
    fn new() -> Self {
        Self { 
            sleep: Box::pin(tokio::time::sleep(Duration::from_secs(1))),
        }
    }
}
