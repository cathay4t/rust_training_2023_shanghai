// SPDX-License-Identifier: Apache-2.0

use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Waker};
use std::time::Duration;

use futures::join;

struct SharedState {
    completed: bool,
    waker: Option<Waker>,
}

struct AsyncSleep {
    shared_state: Arc<Mutex<SharedState>>,
}

impl AsyncSleep {
    fn sleep(dur: Duration) -> Self {
        let shared_state = Arc::new(Mutex::new(SharedState {
            completed: false,
            waker: None,
        }));
        let thread_state = shared_state.clone();
        std::thread::spawn(move || {
            std::thread::sleep(dur);
            let mut state = thread_state.lock().unwrap();
            state.completed = true;
            if let Some(waker) = state.waker.take() {
                waker.wake()
            }
        });
        Self { shared_state }
    }
}

impl Future for AsyncSleep {
    // TODO: The output should be result so that we may include error details
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // TODO: instead of unwrap, we should return error in Output.
        let mut shared_state = self
            .shared_state
            .lock()
            .expect("Failed to lock the share state");
        if shared_state.completed {
            Poll::Ready(())
        } else {
            shared_state.waker = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}

async fn foo() {
    AsyncSleep::sleep(Duration::from_secs(3)).await;
    println!("foo() slept 3");
}

async fn bar() {
    AsyncSleep::sleep(Duration::from_secs(1)).await;
    println!("bar() slept 1");
}

#[tokio::main]
async fn main() {
    join!(foo(), bar());
}
