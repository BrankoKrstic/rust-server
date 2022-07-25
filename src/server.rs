use std::{net::{TcpListener, TcpStream}, io::{Read}};
use crate::http::{Request, Response, StatusCode, request::ParseError};

pub trait Handler {
    fn handle_request(&mut self, request: &Request) -> Response;

    fn handle_bad_request(&mut self, e: &ParseError) -> Response {
        println!("Failed to pars request: {}", e);
        Response::new(StatusCode::BadRequest, None)
    }
}

pub struct Server {
    address: String,
    handler: Box<dyn Handler>
}

impl Server {
    pub fn new (addr: &str, handler: Box<dyn Handler>) -> Self {
        Server {
            address: String::from(addr),
            handler
        }
    }

    pub fn run (&mut self) {
        let listener = TcpListener::bind(&self.address).unwrap();
        println!("Listening on {}", &self.address);
        for res in listener.incoming() {
            match res {
                Ok(stream) => self.handle_stream(stream),
                Err(_) => println!("Failed to process request"),
            }
        }
    }
    fn handle_stream (&mut self, mut stream: TcpStream) {
        let mut buffer = [0; 1024];
        stream.read(&mut buffer).unwrap();
        let req = Request::try_from(&buffer[..]);

        println!("{:?}", req);
        let res = match req {
            Ok(r) => self.handler.handle_request(&r),
            Err(e) => self.handler.handle_bad_request(&e),
        };

        res.send(&mut stream).unwrap();
    }
}