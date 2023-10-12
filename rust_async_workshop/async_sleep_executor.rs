// SPDX-License-Identifier: Apache-2.0

use std::time::Duration;

use futures::future::join;
use nix::sys::time::TimeSpec;
use nix::sys::timerfd::{
    ClockId::CLOCK_BOOTTIME, Expiration, TimerFd, TimerFlags, TimerSetTimeFlags,
};

#[cfg(feature = "smol")]
use smol::Async as DefaultAsync;
#[cfg(not(feature = "smol"))]
use tokio::io::unix::AsyncFd as DefaultAsync;

struct AsyncSleep;

impl AsyncSleep {
    async fn sleep(dur: Duration) {
        let timer = TimerFd::new(CLOCK_BOOTTIME, TimerFlags::empty()).unwrap();
        timer
            .set(
                Expiration::OneShot(TimeSpec::from_duration(dur)),
                TimerSetTimeFlags::empty(),
            )
            .unwrap();
        let _ = DefaultAsync::new(timer).unwrap().readable().await.unwrap();
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

fn main() {
    #[cfg(feature = "smol")]
    smol::block_on(join(foo(), bar()));
    #[cfg(not(feature = "smol"))]
    tokio::runtime::Builder::new_multi_thread()
        .enable_io()
        .build()
        .unwrap()
        .block_on(join(foo(), bar()));
}
