use anyhow::{Context, anyhow, bail, ensure};

use super::{utils::*, *};
use std::fmt::format;
use std::str::FromStr;
use std::{collections::HashMap, fmt};

#[cfg(test)]
mod tests;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HttpRequest<'a> {
    pub method: enums::Method,
    pub path: HttpPath,
    pub version: enums::Version,
    pub header: HashMap<&'a str, &'a str>,
    pub body: String,
}
impl<'a> HttpRequest<'a> {
    pub fn new(
        method: enums::Method,
        path: HttpPath,
        version: enums::Version,
        header: HashMap<&'a str, &'a str>,
        body: String,
    ) -> Self {
        Self {
            method,
            path,
            version,
            header,
            body,
        }
    }
    pub fn from_str(s: &'a str) -> anyhow::Result<Self> {
        // 行取得で行う
        let mut lines = s.lines();

        // 1行目を取得する
        let mut parts = {
            let line = lines.next().unwrap_or("");
            line.split_whitespace() // スペース単位で分割させる
        };

        let method = enums::Method::from_str(parts.next().unwrap_or(""))?;
        let path = HttpPath::from_str(parts.next().unwrap_or(""))?;
        let version = enums::Version::from_str(parts.next().unwrap_or(""))?;
        // 余分にあったら無効とする
        ensure!(parts.next().is_none(), "error");

        // 2行目(以降)を処理する
        let mut header: HashMap<&str, &str> = HashMap::new();
        loop {
            let line = lines.next().unwrap_or("");
            match line_parse_http_header(line) {
                Some((k, v)) => {
                    _ = header.insert(k, v);
                }
                None => break,
            }
        }

        // headerの処理をする
        let body = lines.collect::<String>();

        Ok(Self {
            method,
            path,
            version,
            header,
            body,
        })
    }
}

// 文字列で取得できるように、Displayを実装しておきましょう
impl<'a> fmt::Display for HttpRequest<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}\r\n", self.method, self.path, self.version)?;
        for (k, v) in &self.header {
            write!(f, "{}: {}\r\n", k, v)?;
        }
        write!(f, "\r\n{}", self.body)
    }
}
