use chronicle_common::request::Request;
use std::net::TcpListener;

pub struct Server {
    port: i32,
    listener: TcpListener,
}

impl Server {
    pub fn build(port: i32) -> Self {
        let listener = TcpListener::bind(format!("127.0.0.1:{}", port.to_string())).unwrap();
        Self { port, listener }
    }

    pub fn serve(self) {
        for stream in self.listener.incoming() {
            let mut stream = stream.unwrap();
            println!("Connection from {}", stream.peer_addr().unwrap());
            let request = Request::parse_stream_to_request(&mut stream);
        }
    }
}
