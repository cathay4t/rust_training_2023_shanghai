// SPDX-License-Identifier: Apache-2.0

use once_cell::sync::OnceCell;
use std::sync::Mutex;

static GLOBAL_VEC: OnceCell<Mutex<Vec<String>>> = OnceCell::new();

fn new_thread() {
    GLOBAL_VEC
        .get_or_init(|| Mutex::new(Vec::new()))
        .lock()
        .unwrap()
        .push("ABCD".to_string());
}

fn main() {
    GLOBAL_VEC
        .get_or_init(|| Mutex::new(Vec::new()))
        .lock()
        .unwrap()
        .push("ABC".to_string());
    println!("Before {:?}", GLOBAL_VEC.get().unwrap().lock().unwrap());

    let nt = std::thread::spawn(move || new_thread());
    println!("During {:?}", GLOBAL_VEC.get().unwrap().lock().unwrap());
    nt.join().unwrap();

    println!("After {:?}", GLOBAL_VEC.get().unwrap().lock().unwrap());
}
