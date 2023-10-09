// SPDX-License-Identifier: Apache-2.0

fn main() {
    let a = 100u16;

    let b = match "100".parse::<u16>() {
        Ok(i) => i,
        Err(_) => unreachable!(),
    };

    assert_eq!(a, b);

    println!("PASS");
}
