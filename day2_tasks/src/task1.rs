// SPDX-License-Identifier: Apache-2.0

use std::net::Ipv4Addr;
use std::str::FromStr;

fn main() {
    let a = ["927.0.0.2", "127.0.0.1", "127.0.0.1"];

    /*
    let mut ret: Vec<Ipv4Addr> =
        a.iter().map(|s| s.parse::<Ipv4Addr>().unwrap()).collect();
    */

    let mut ret: Vec<Ipv4Addr> = Vec::new();

    let a = vec![String::new()];
    for i in a.as_slice() {
        let b: () = i;
    }
    println!("HAHA {:?}", a);

    let i = 8u8;

    match i {
        x if x % 2 == 1 => {
            println!("HAHA {:?}", i)
        }
        x if x % 2 != 1 => {
            println!("HAHA {:?}", i)
        }
        _ => todo!()
    }

    ret.sort_unstable();
    ret.dedup();

    println!("HAHA {:?}", ret);
}
