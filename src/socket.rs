use std::io::prelude::*;
use std::net::TcpStream;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::str::FromStr;
use std::time::Duration;

pub fn open(host: &str) -> std::io::Result<()> {
    let addr = SocketAddr::from_str("1.2.3.4:123").unwrap();
    let mut stream = TcpStream::connect_timeout(&addr, Duration::new(5, 0))?;

    stream.write(&[1])?;
    stream.read(&mut [0; 128])?;
    Ok(())
} // the stream is closed here
