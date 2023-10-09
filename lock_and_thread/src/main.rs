// SPDX-License-Identifier: Apache-2.0

use std::fmt::Write;
use std::sync::{Arc, Mutex};

#[derive(Debug)]
struct ShareState {
    state: String,
}

#[derive(Debug)]
pub struct AbcData {
    share_state: Arc<Mutex<ShareState>>,
}

impl Default for AbcData {
    fn default() -> Self {
        let share_state = Arc::new(Mutex::new(ShareState {
            state: String::new(),
        }));
        Self { share_state }
    }
}

fn append_state(share_state: Arc<Mutex<ShareState>>, postfix: String) {
    match share_state.lock() {
        Ok(mut s) => {
            write!(s.state, "{}", postfix).ok();
        }
        Err(e) => {
            eprintln!("Failed to acquire lock on share_state: {e}");
        }
    }
}

fn main() {
    let data = AbcData::default();
    let mut threads = Vec::new();
    for i in 1..10 {
        let share_state = data.share_state.clone();
        threads.push(std::thread::spawn(move || {
            append_state(share_state, i.to_string())
        }));
    }
    for thread in threads {
        thread.join().expect("Thread failed");
    }
    println!("state {:?}", data.share_state.lock().unwrap().state);
}
