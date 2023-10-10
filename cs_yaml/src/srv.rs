// SPDX-License-Identifier: Apache-2.0

use std::ffi::CString;
use std::io::Read;
use std::net::{TcpListener, TcpStream};

mod info;

use crate::info::CliInfo;

fn connection_thread_func(
    mut stream: TcpStream,
    sender: std::sync::mpsc::Sender<CliInfo>,
) {
    let mut size_buff = [0u8; 8];
    if let Err(e) = stream.read_exact(&mut size_buff) {
        eprintln!("Failed to read i64 from client: {e}");
        return;
    }
    let size = usize::from_be_bytes(size_buff);
    let mut data = vec![0u8; size];
    stream.read_exact(data.as_mut_slice()).unwrap();

    match CString::from_vec_with_nul(data) {
        Ok(cli_info_cs) => {
            let cli_info_str = cli_info_cs.to_str().unwrap();

            let cli_info: CliInfo = serde_yaml::from_str(cli_info_str).unwrap();
            if let Err(e) = sender.send(cli_info) {
                eprintln!("Failed to send from thread {e}");
            }
        }
        Err(e) => {
            eprintln!("Failed to retrieve string from client {e}");
        }
    }
}

fn srv_report_thread(receiver: std::sync::mpsc::Receiver<CliInfo>) {
    loop {
        match receiver.recv() {
            Ok(cli_info) => {
                println!(
                    "Got client info\n{}",
                    serde_yaml::to_string(&cli_info).unwrap()
                );
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
