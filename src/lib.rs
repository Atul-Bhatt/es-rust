use hyper::{self, client::HttpConnector};

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