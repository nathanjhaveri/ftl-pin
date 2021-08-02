use futures::FutureExt;
use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
    time::Duration,
};
use tokio::{
    fs::File,
    io::{AsyncRead, AsyncReadExt, ReadBuf},
    time::{sleep, Sleep},
};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), tokio::io::Error> {
    let mut buf = vec![0u8; 128 * 1024];
    let mut f = SlowRead::new(File::open("/dev/urandom").await?);
    let mut f = unsafe { Pin::new_unchecked(&mut f) };
    let before = tokio::time::Instant::now();

    f.read_exact(&mut buf).await?;

    println!("Read {} bytes in {:?}", buf.len(), before.elapsed());

    Ok(())

    //let fut = MyFuture::new();
    //println!("Awaiting fut...");
    //fut.await;
    //println!("Awaiting fut... done!");
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

////////

struct SlowRead<R> {
    reader: R,
    sleep: Sleep,
}

impl<R: AsyncRead> SlowRead<R> {
    fn new(reader: R) -> Self {
        Self {
            reader,
            sleep: tokio::time::sleep(Default::default()),
        }
    }
}

impl<R: AsyncRead + Unpin> AsyncRead for SlowRead<R> {
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<std::io::Result<()>> {
        let sleep = unsafe { self.as_mut().map_unchecked_mut(|this| &mut this.sleep) };

        //let sleep = unsafe { Pin::new_unchecked(&mut self.sleep) };

        match sleep.poll(cx) {
            Poll::Ready(_) => {
                let sleep = unsafe { self.as_mut().map_unchecked_mut(|this| &mut this.sleep) };

                sleep.reset(tokio::time::Instant::now() + Duration::from_millis(25));
                let reader = unsafe { self.as_mut().map_unchecked_mut(|this| &mut this.reader) };
                reader.poll_read(cx, buf)
            }
            Poll::Pending => Poll::Pending,
        }
    }
}
