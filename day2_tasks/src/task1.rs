// SPDX-License-Identifier: Apache-2.0

use std::net::Ipv4Addr;

fn main() {
    let a = ["127.0.0.2", "127.0.0.1", "127.0.0.1"];

    let mut ret: Vec<Ipv4Addr> =
        a.iter().map(|s| s.parse::<Ipv4Addr>().unwrap()).collect();

    ret.sort_unstable();
    ret.dedup();

    println!("HAHA {:?}", ret);
}
