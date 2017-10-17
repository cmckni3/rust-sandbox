#[macro_use]
extern crate log;
extern crate env_logger;

extern crate simple_server;

use simple_server::Server;

fn main() {
    env_logger::init().unwrap();

    let host = "0.0.0.0";
    let port = "7878";

    let server = Server::new(|request, mut response| {
        info!("Request received. {} {}", request.method(), request.uri());
        Ok(response.body("Hello Rust!".as_bytes())?)
    });

    server.listen(host, port);
}
