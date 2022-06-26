use chronicle_server::server::server::Server;

fn main() {
    let server = Server::build(3000);
    server.serve();
}
