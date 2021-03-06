use chronicle_common::request::{Request, RequestMethod};
use chronicle_common::response::Response;
use chronicle_common::stream::ApplicationStream;
use chronicle_common::tls::TlsConnector;
use chronicle_common::url::Url;

pub struct Client {
    tls_connector: TlsConnector,
}

impl Client {
    pub fn build() -> Self {
        return Self {
            tls_connector: TlsConnector::new(),
        };
    }

    pub fn get(url: String) -> Response {
        let url = Url::new(&url);

        let request = Request::build(url.pathname.clone(), RequestMethod::GET, url.host.clone());
        let request = request.print();
        let client = Client::build();
        let mut stream = client.connect(url);
        let request = request.as_bytes();
        stream.write(request).unwrap();
        stream.write(&[0]).unwrap();
        let response = Response::parse_stream_to_response(&mut stream);
        response
    }

    fn connect(&self, url: Url) -> ApplicationStream {
        ApplicationStream::new(&url, &self.tls_connector)
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
    fn it_works_with_https() {
        let response = Client::get("https://github.com/".to_string());
        assert_eq!(response.status_code, 200);
    }

    #[test]
    fn it_works_with_url_not_ending_with_slash() {
        let response = Client::get("http://example.com".to_string());
        assert_eq!(response.status_code, 200);
    }
}
