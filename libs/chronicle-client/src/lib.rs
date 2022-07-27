use crate::url::{Protocol, Url};
use chronicle_common::request::{Request, RequestMethod};
use chronicle_common::response::Response;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::str;

mod url;

pub struct Client {}

impl Client {
    pub fn build() -> Self {
        return Self {};
    }

    pub fn get(url: String) -> Response {
        let url = Url::new(&Self::validate_url(url));

        let request = Request::build(url.pathname.clone(), RequestMethod::GET, url.host.clone());
        let host = request.host.clone();
        let request = request.print();
        let mut stream = Client::connect(url);
        let request = request.as_bytes();
        stream.write(request);
        stream.write(&[0]);
        let response = Response::parse_stream_to_response(&mut stream);
        response
    }

    fn connect(url: Url) -> TcpStream {
        let stream = TcpStream::connect(Self::make_connection_port(url));
        let stream = stream.unwrap();
        stream
    }

    fn make_connection_port(url: Url) -> String {
        let port = match url.protocol {
            Protocol::http => "80",
            Protocol::https => "443",
            _ => panic!("unsupported protocol {:?}", url.protocol),
        };

        format!("{}:{}", url.host, port)
    }

    fn validate_url(url: String) -> String {
        if url.ends_with("/") {
            url
        } else {
            let mut url = url.clone();
            url.push('/');
            url
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let response = Client::get("http://example.com/".to_string());
        assert_eq!(response.status_code, 200);
    }

    #[test]
    fn it_works_with_url_not_ending_with_slash() {
        let response = Client::get("http://example.com".to_string());
        assert_eq!(response.status_code, 200);
    }
}
