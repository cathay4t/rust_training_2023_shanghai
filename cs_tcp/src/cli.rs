// SPDX-License-Identifier: Apache-2.0

use std::io::{Read, Write};
use std::net::TcpStream;

fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:8766")?;
    let time = chrono::Utc::now();

    stream.write(&time.timestamp().to_be_bytes())?;
    stream.write(&time.timestamp_subsec_nanos().to_be_bytes())?;
    stream.read(&mut [0;1])?;
    Ok(())
}
