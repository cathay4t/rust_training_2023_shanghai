// SPDX-License-Identifier: Apache-2.0

use std::future::Future;

fn foo() -> impl Future<Output = u8> {
    async {
        println!("foo() 1");
        1
    }
}

async fn bar() -> u8 {
    println!("bar() 2");
    2
}

#[tokio::main]
async fn main() {
    foo().await;
    bar().await;
}
