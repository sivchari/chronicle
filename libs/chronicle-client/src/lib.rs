use crate::url::Url;
use chronicle_common::request::{Request, RequestMethod};
use std::io::{Read, Write};
use std::net::TcpStream;
use std::str;

mod url;

pub struct Client {}

impl Client {
    pub fn build() -> Self {
        return Self {};
    }

    pub fn get(url: String) {
        let url = Url::new(&url);

        let request = Request::build(url.pathname, RequestMethod::GET);
        let request = request.print();
        let mut stream = Client::connect(url.host);
        stream.write(request.as_bytes());
        let mut buffer = [0; 2048];
        let nbytes = stream.read(&mut buffer).unwrap();
        let http = str::from_utf8(&buffer).unwrap();
        println!("{}", http);
    }

    fn connect(host: String) -> TcpStream {
        let stream = TcpStream::connect(host).expect("Could not connect.");
        stream
    }
}
