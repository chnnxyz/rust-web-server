mod server;
mod requests;
fn main() {
    // use let server = server::Server::new(); for custom ip and port
    let server = server::Server::default();
    server.run();
}

