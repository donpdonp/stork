use std::io::prelude::*;
use std::net::TcpStream;

pub fn open(host: &str) -> std::io::Result<()> {
    let mut stream = TcpStream::connect(host)?;

    stream.write(&[1])?;
    stream.read(&mut [0; 128])?;
    Ok(())
} // the stream is closed here

