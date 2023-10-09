// SPDX-License-Identifier: Apache-2.0

fn count_odd(data: &[u16]) -> usize {
    data.iter().filter(|i| *i % 2 == 1).count()
}

fn main() {
    let a: [u16; 4] = [1u16, 2, 3, 4];

    let count = count_odd(&a);

    println!(
        "Got {} odd number{}",
        count,
        if count > 1 { "s" } else { "" }
    );
}
