// SPDX-License-Identifier: Apache-2.0

fn gen_two_opt() -> (Option<u8>, Option<u8>) {
    (Some(2), None)
}

fn main() {
    match gen_two_opt() {
        (Some(a), None) => {
            println!("Got {}", a);
        }
        _ => unreachable!(),
    }
}
