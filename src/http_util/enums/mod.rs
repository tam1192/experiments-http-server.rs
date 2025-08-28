use core::fmt;
use std::str::FromStr;

use strum::EnumProperty;

use crate::http_util::utils::line_parse_http_header;
use anyhow::bail;

#[cfg(test)]
mod tests;

#[derive(Debug, Clone, PartialEq, Eq, strum_macros::Display, strum::EnumString)]
pub enum Method {
    GET,
    POST,
    PUT,
    DELETE,
    #[strum(to_string = "{0}")]
    Other(String),
}

#[derive(Debug, Clone, PartialEq, Eq, strum_macros::Display, strum::EnumString)]
pub enum Version {
    #[strum(serialize = "HTTP/1.0")]
    Http10,
    #[strum(serialize = "HTTP/1.1")]
    Http11,
    #[strum(serialize = "HTTP/2.0")]
    Http20,
    #[strum(serialize = "HTTP/3.0")]
    Http30,
}

#[derive(Debug, Clone, PartialEq, Eq, strum::EnumString, strum::EnumProperty)]
pub enum Status {
    #[strum(
        props(code = 200, msg = "OK"),
        serialize = "200 OK",
        serialize = "200",
        serialize = "OK"
    )]
    Ok,
    #[strum(
        props(code = 400, msg = "Bad Request"),
        serialize = "400 Bad Request",
        serialize = "400",
        serialize = "Bad Request"
    )]
    BadRequest,
    #[strum(
        props(code = 404, msg = "Not Found"),
        serialize = "404 Not Found",
        serialize = "404",
        serialize = "Not Found"
    )]
    NotFound,
    #[strum(
        props(code = 405, msg = "Method Not Allowed"),
        serialize = "405 Method Not Allowed",
        serialize = "405",
        serialize = "Method Not Allowed"
    )]
    MethodNotAllowed,
    Other(u32, String),
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let code = self.get_int("code").unwrap();
        let msg = self.get_str("msg").unwrap();

        write!(f, "{} {}", code, msg)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, strum_macros::Display, strum::EnumString)]
pub enum MIME {
    #[strum(serialize = "application/json")]
    ApplicationJson,
    #[strum(serialize = "text/plain")]
    TextPlain,
    #[strum(serialize = "text/html")]
    TextHtml,
}

#[derive(Debug, Clone, PartialEq, Eq, strum_macros::Display, strum::EnumString)]
pub enum Charset {
    #[strum(serialize = "utf-8")]
    UTF8,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Header {
    ContentType { mime: MIME, charset: Charset },
    Other(String),
}

impl FromStr for Header {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some((k, v)) = line_parse_http_header(s) else {
            bail!("error")
        };

        Ok(match k {
            "Content-Type" | "content-type" => {}
            _ => Self::Other(format!("{}:{}", k, v)),
        })
    }
}
