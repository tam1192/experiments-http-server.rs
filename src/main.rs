use anyhow::{Result, anyhow};
use std::{
    collections::HashMap,
    io::{Read, Write},
    net::TcpListener,
};

mod http_util;

fn main() -> Result<()> {
    let listener = TcpListener::bind("0.0.0.0:4000")?;
    {
        let (mut stream, _) = listener.accept()?;
        let mut buf = [0u8; 512];

        stream.read(&mut buf)?;

        let buf = String::from_utf8_lossy(&buf);

        let req = http_util::HttpRequest::from_str(&buf).ok_or(anyhow!("parse error"))?;
        println!("{:?}", req);

        let mut res_header = HashMap::new();
        res_header.insert("Content-Type", "text/plain");
        let res = http_util::HttpResponse::new(
            req.version,
            (200, "Ok"),
            res_header,
            String::from("hello world"),
        );
        stream.write_all(res.to_string().as_bytes())?;

        stream.flush()?;
    }

    Ok(())
}
