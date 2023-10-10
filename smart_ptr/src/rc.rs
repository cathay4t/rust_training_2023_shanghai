// SPDX-License-Identifier: Apache-2.0

use std::rc::{Rc, Weak};

fn foo(weak: &Weak<String>, strong: Rc<String>) {
    drop(strong);
    println!("Weak {:?}", weak.upgrade());
}

fn could_panic() {
use std::cell::RefCell;

let c = RefCell::new(5);

let m = c.borrow_mut();
let b = c.borrow(); // this causes a panic

}

fn main() {
    // the Interior Mutability Pattern on counter
    let my_rc = Rc::new("TestString".to_string());
    println!("My RC {:?}", my_rc.as_str());
    let my_weak = Rc::downgrade(&my_rc);
    foo(&my_weak, my_rc.clone());
    drop(my_rc);
    println!("Weak {:?}", my_weak.upgrade());
}
