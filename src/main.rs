use anyhow::{Result, anyhow};
use std::{
    collections::HashMap,
    io::{Read, Write},
    net::TcpListener,
};

use crate::http_util::enums;

mod http_util;

fn main() -> Result<()> {
    let listener = TcpListener::bind("0.0.0.0:4000")?;
    {
        let (mut stream, _) = listener.accept()?;
        let mut buf = [0u8; 512];

        stream.read(&mut buf)?;

        let buf = String::from_utf8_lossy(&buf);

        let req = http_util::HttpRequest::from_str(&buf)?;
        println!("{:?}", req);
    }

    Ok(())
}
