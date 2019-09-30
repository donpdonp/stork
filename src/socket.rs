use std::io::prelude::*;
use std::net::TcpStream;
use std::net::{SocketAddr};
use std::str::FromStr;
use std::time::Duration;

pub fn open(host: &str) -> std::io::Result<()> {
    let addr = SocketAddr::from_str(host).unwrap();
    let mut stream = TcpStream::connect_timeout(&addr, Duration::new(5, 0))?;

    stream.write(&[1])?;
    stream.read(&mut [0; 128])?;
    Ok(())
} // the stream is closed here
