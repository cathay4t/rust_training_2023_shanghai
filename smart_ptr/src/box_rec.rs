// SPDX-License-Identifier: Apache-2.0

#[derive(Debug)]
enum List {
    Cons(u32, Box<List>),
    Nil,
}

fn main() {
    let list = List::Cons(
        1,
        Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))),
    );

    println!("HAHA {:?}", list);
}
