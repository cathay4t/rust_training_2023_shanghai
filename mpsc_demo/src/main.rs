// SPDX-License-Identifier: Apache-2.0

const THREAD_COUNT: usize = 10;

fn thread_func(id: usize, sender: std::sync::mpsc::Sender<String>) {
    if let Err(e) = sender.send(format!("Thread ID {id}")) {
        eprintln!("Failed to send from thread {e}");
    }
}

fn main() {
    let (sender, reciever) = std::sync::mpsc::channel();
    for i in 1..THREAD_COUNT {
        let sender = sender.clone();
        std::thread::spawn(move || thread_func(i, sender));
    }
    for _ in 1..THREAD_COUNT {
        match reciever.recv() {
            Ok(s) => println!("Got data from thread {s}"),
            Err(e) => eprintln!("Failed to recv from thread {e}"),
        }
    }
}
