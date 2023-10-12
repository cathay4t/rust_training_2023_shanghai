// SPDX-License-Identifier: Apache-2.0

use std::time::Duration;

use futures::future::join;

async fn foo() {
    tokio::time::sleep(Duration::from_secs(3)).await;
    println!("foo() slept 3");
}

async fn bar() {
    tokio::time::sleep(Duration::from_secs(1)).await;
    println!("bar() slept 1");
}

#[tokio::main]
async fn main() {
    join(foo(), bar()).await;
}
