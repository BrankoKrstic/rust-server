use std::fs;

use crate::{server::Handler, http::{Response, StatusCode, Method}};

pub struct WebsiteHandler {
    public_path: String
}

impl WebsiteHandler {
    pub fn new(public_path: String) -> Self {
        WebsiteHandler { public_path }
    }
    pub fn read_file(&self, file_path: &str) -> Option<String> {
        let fullPath = format!("{}/{}", self.public_path, file_path);
        match fs::canonicalize(&fullPath) {
            Ok(path) if path.starts_with(&self.public_path) => fs::read_to_string(fullPath).ok(),
            _ => None,
        }
    }
}

impl Handler for WebsiteHandler {
    fn handle_request(&mut self, request: &crate::http::Request) -> crate::http::Response {
        match request.method() {
            Method::GET => {
                let contents = self.read_file(request.path());
                if let Some(str_content) = contents {
                    Response::new(StatusCode::Ok, Some(str_content))
                } else {
                    Response::new(StatusCode::NotFound, None)
                }
            },
            _ => Response::new(StatusCode::NotFound, None)
        }

    }
}