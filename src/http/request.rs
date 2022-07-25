use std::{convert::TryFrom, error::Error, fmt::{Display, Debug}, str::Utf8Error};

use super::{Method, method::MethodError, QueryString};

#[derive(Debug)]
pub struct Request<'buf> {
    path: &'buf str,
    query_string: Option<QueryString<'buf>>,
    method: Method
}

impl <'buf> Request<'buf> {
    pub fn path(&self) -> &str {
        &self.path
    }
    pub fn method(&self) -> &Method {
        &self.method
    }
    pub fn query_string(&self) -> Option<&QueryString> {
        self.query_string.as_ref()
    }
}


impl <'buf> TryFrom<&'buf [u8]> for Request<'buf>
{
    type Error = ParseError;

    fn try_from(buf: &'buf [u8]) -> Result<Request<'buf>, Self::Error> {
        let req_str = std::str::from_utf8(buf)?;
        let mut first_line = req_str
            .split("\r\n")
            .next()
            .ok_or(Self::Error::InvalidRequest)?
            .split(" ");


        let method: Method = first_line
            .next()
            .ok_or(Self::Error::InvalidMethod)?
            .parse()
            .or(Err(Self::Error::InvalidMethod))?;

        let mut full_path = first_line
            .next()
            .ok_or(Self::Error::InvalidRequest)?
            .split("?");

        let path = full_path
            .next()
            .ok_or(Self::Error::InvalidRequest)?;
        
        let mut query_string = None;
        if let Some(q) = full_path.next() {
            query_string = Some(QueryString::from(q))
        }

        match first_line.next() {
            Some(x) if x != "HTTP/1.1"  => {},
            _ => return Err(Self::Error::InvalidProtocol),
        };
        
        Ok(Request {
            method,
            path: path,
            query_string
        })
    }
}

pub enum ParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod
}

impl ParseError {
    fn message(&self) -> &str {
        match self {
            Self::InvalidRequest => "Invalid Request",
            Self::InvalidEncoding => "Invalid Encoding",
            Self::InvalidProtocol => "Invalid Protocol",
            Self::InvalidMethod => "Invalid Method",
        }
    }
}

impl From<Utf8Error> for ParseError
{
    fn from(_: Utf8Error) -> Self {
        Self::InvalidEncoding
    }
}

impl From<MethodError> for ParseError {
    fn from(_: MethodError) -> Self {
        Self::InvalidMethod
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message())
    }
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message())
    }
}

impl Error for ParseError {}