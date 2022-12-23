mod server;

fn main() {
    let server = server::Server::default();
    println!(
        "Serving on: {}:{}",
        server.address,
        server.port
    );
    server.run();
}

