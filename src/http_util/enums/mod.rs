use core::fmt;

use strum::EnumProperty;

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

pub mod MIME {
    pub enum Type {
        Text(Text),
        Application,
    }
    pub enum Text {
        Plain,
        Html,
    }
    pub enum Application {
        Json,
    }
}

pub enum Charset {
    UTF8,
}

pub enum Header {
    ContentType { mime: MIME::Type, charset: Charset },
    Other(String),
}
