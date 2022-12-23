mod server;
mod requests;
mod utils;

use server::Server;
fn main() {
    // use let server = server::Server::new(); for custom ip and port
    let server = Server::default();
    server.run();
}

