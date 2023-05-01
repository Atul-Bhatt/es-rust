use hyper::{self, client::HttpConnector};

use std::error::Error;
use std::io;
use std::io::Read;
use std::fmt;

use rustc_serialize::json;
use rustc_serialize::json::Json;

#[derive(Debug)]
enum EsError{
    EsError(String),
    HttpError(hyper::Error),
    IoError(io::Error),
    JsonError(json::BuilderError)
}

impl From<io::Error> for EsError {
    fn from(err: io::Error) -> EsError {
        EsError::IoError(err)
    }
}

impl From<hyper::Error> for EsError {
    fn from(err: hyper::Error) -> EsError {
        EsError::HttpError(err)
    }
}

impl From<json::BuilderError> for EsError {
    fn from(err: json::BuilderError) -> EsError {
        EsError::JsonError(err)
    }
}

struct Client {
    http_client: hyper::Client<HttpConnector>,
}

impl Client {
    fn new() -> Client {
        Client {
            http_client: hyper::Client::new()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Client;

    #[test]
    fn create_client_working() {
        let mut client = Client::new();
    }
}