use std::{fmt, str::FromStr};

use anyhow::{bail, ensure};

#[cfg(test)]
mod tests;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HttpPath(String);

impl FromStr for HttpPath {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        // 0文字を認めない
        ensure!(s.len() > 0, "path parse error: input is 0 characters");

        // 文字単位に分解します
        let mut c = s.chars();

        // 先頭は/になると見込んで
        ensure!(c.next() == Some('/'), "path parse error: Must begin with /");

        // findメソッドで許可されていない文字があるか検索しましょう
        // なかったら成功です。
        if let Some(c) = c.find(|c| {
            !(c.is_ascii_alphanumeric()
                || *c == '/'
                || *c == '-'
                || *c == '_'
                || *c == '.'
                || *c == '='
                || *c == '?'
                || *c == '&'
                || *c == '%'
                || *c == '#')
        }) {
            // 見つかったらエラーとする
            bail!("path parse error: {} is not allowed", c)
        }

        Ok(HttpPath(s.to_string()))
    }
}

// 文字列で取得できるように、Displayを実装しておきましょう
impl<'a> fmt::Display for HttpPath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
