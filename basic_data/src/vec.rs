// SPDX-License-Identifier: Apache-2.0

use std::ops::Deref;

fn gen_vec() -> Vec<u8> {
    let mut ret = Vec::new();
    for i in 0..100 {
        ret.push(i);
    }
    ret
}

fn main() {
    let data = gen_vec();
    // let a: () = data.deref();
    println!("First one {:?}", data.get(0));
    println!("First one {:?}", data.iter().next());
    println!("First 5 {:?}", &data[..5]);
    println!("Last 5 {:?}", &data[data.len() - 5..]);

    let simple_data = [1u8, 2, 3];
    let opt_data = Some(simple_data.to_vec());
    let opt_data2: Vec<u16> = simple_data.iter().map(|i| (*i).into()).collect::<T>();
    // let expected = Some(simple_data);
    let expected = Some(simple_data.as_slice());
    assert_eq!(opt_data.as_deref(), expected);
    assert_eq!(opt_data2.deref(), [1u16, 2, 3].as_slice());
}
