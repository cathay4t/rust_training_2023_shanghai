// SPDX-License-Identifier: Apache-2.0

#[derive(Copy, Clone, Debug)]
struct DataCopy {
    data: u8,
}

impl DataCopy {
    fn new(data: u8) -> Self {
        Self { data }
    }
}

// Copy not allowed to have Drop
// impl std::ops::Drop  for DataCopy {
//    fn drop(&mut self) {
//        println!("Dropped {:?}", self);
//    }
// }

#[derive(Clone, Debug)]
struct DataClone {
    data: u8,
}

impl DataClone {
    fn new(data: u8) -> Self {
        Self { data }
    }
}

impl std::ops::Drop for DataClone {
    fn drop(&mut self) {
        println!("Dropped {:?}", self);
    }
}

fn main() {
    let a = DataCopy::new(9);
    let mut b = a;
    b.data += 1;
    println!("copy b {:?}", b);
    println!("copy a {:?}", a);

    let a = DataClone::new(9);
    // let mut b = a;
    let mut b = a.clone();
    b.data += 1;
    println!("clone b {:?}", b);
    println!("clone a {:?}", a);

    // None Lexical Lifetimes
    let mut x = 9;
    let _p = &mut x;
    println!("{}", x);
}
