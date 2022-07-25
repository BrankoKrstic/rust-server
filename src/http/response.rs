use std::{fmt::Display, io::Write};



#[derive(Debug)]
pub struct Response {
    status: StatusCode,
    headers: Option<String>,
    body: Option<String>
}

impl Response {
    pub fn new(status: StatusCode, body: Option<String>) -> Self {
        let headers = match &body {
            Some(s) => Some(format!("Content-Length: {}", s.len())),
            None => None,
        };
        return Response {
            status,
            headers,
            body
        }
    }
    pub fn send(&self, stream: &mut impl Write) -> std::io::Result<()> {
        let body = match &self.body {
            Some(b) => b,
            None => "",
        };
        let headers = match &self.headers {
            Some(h) => h,
            None => "",
        };
        write!(stream, "HTTP/1.1 {} {}\r\n{}\r\n\r\n{}", self.status, self.status.reason_phrase(), headers, body)
    }
}

#[derive(Copy, Clone, Debug)]
pub enum StatusCode {
    Ok = 200,
    BadRequest = 400,
    NotFound = 404
}

impl StatusCode {
    pub fn reason_phrase (&self) -> &str {
        match self {
            StatusCode::Ok => "Ok",
            StatusCode::BadRequest => "Bad Request",
            StatusCode::NotFound => "Not Found",
        }
    }
}

impl Display for StatusCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", *self as u16)
    }
}