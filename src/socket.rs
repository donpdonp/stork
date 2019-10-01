use std::io::prelude::*;
use std::net::{TcpStream, SocketAddr};
use std::str::FromStr;
use std::time::Duration;

pub fn open(host: &str) -> std::io::Result<TcpStream> {
    let addr = SocketAddr::from_str(host).unwrap();
    let mut stream = TcpStream::connect_timeout(&addr, Duration::new(5, 0))?;
    Ok(stream)
}
