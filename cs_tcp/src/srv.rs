// SPDX-License-Identifier: Apache-2.0

use chrono::{DateTime, Local, SecondsFormat, Utc};
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn connection_thread_func(
    mut stream: TcpStream,
    sender: std::sync::mpsc::Sender<(i64, u32)>,
) {
    let mut i64_buff = [0u8; 8];
    let mut u32_buff = [0u8; 4];
    if let Err(e) = stream.read_exact(&mut i64_buff) {
        eprintln!("Failed to read i64 from client: {e}");
        return;
    }
    if let Err(e) = stream.read_exact(&mut u32_buff) {
        eprintln!("Failed to read i64 from client: {e}");
        return;
    }
    if let Err(e) = sender
        .send((i64::from_be_bytes(i64_buff), u32::from_be_bytes(u32_buff)))
    {
        eprintln!("Failed to send from thread {e}");
    }
    stream.write(&[0; 1]).ok();
}

fn srv_report_thread(receiver: std::sync::mpsc::Receiver<(i64, u32)>) {
    loop {
        match receiver.recv() {
            Ok((secs, subsec_nanos)) => {
                if let Some(t) =
                    DateTime::<Utc>::from_timestamp(secs, subsec_nanos)
                        .map(|t| t.with_timezone(&Local))
                {
                    println!(
                        "Got client time {}",
                        t.to_rfc3339_opts(SecondsFormat::Millis, false)
                    );
                }
            }
            Err(e) => eprintln!("Failed to recv from thread {e}"),
        }
    }
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8766")?;

    let (sender, receiver) = std::sync::mpsc::channel();

    std::thread::spawn(move || srv_report_thread(receiver));

    loop {
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    println!("Got connection from {:?}", stream);
                    let sender = sender.clone();
                    std::thread::spawn(move || {
                        connection_thread_func(stream, sender)
                    });
                }
                Err(e) => {
                    eprintln!("Cannot connect to TCP client {e:?}");
                }
            }
        }
    }
}
