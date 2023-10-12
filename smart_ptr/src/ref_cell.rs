// SPDX-License-Identifier: Apache-2.0

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn could_panic() {
    let c = RefCell::new(5);

    let m = c.borrow_mut();
    let b = c.borrow(); // this causes a panic
}

fn main() {
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    // The runtime checks on borrowing race problem
    *value.borrow_mut() += 10;

    println!("a: {:?}", a);
    println!("b: {:?}", b);
    println!("c: {:?}", c);

    could_panic();
}
