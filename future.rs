use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Duration;
use std::thread;

// 独自のFutureの実装
struct Delay {
    duration: Duration,
}

impl Future for Delay {
    type Output = &'static str;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.duration.as_secs() == 0 {
            Poll::Ready("done")
        } else {
            self.duration = self.duration.checked_sub(Duration::from_secs(1)).unwrap();
            let waker = cx.waker().clone();
            thread::spawn(move || {
                thread::sleep(Duration::from_secs(1));
                waker.wake();
            });
            Poll::Pending
        }
    }
}

// 簡単な非同期関数
async fn my_async_function() -> &'static str {
    let delay = Delay { duration: Duration::from_secs(3) };
    delay.await
}

// main関数
fn main() {
    let future = my_async_function(); // Futureを取得
    let result = futures::executor::block_on(future); // ブロックしてFutureを実行
    println!("Future 完了: {}", result);
}

