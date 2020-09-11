mod connectioninfo;
mod server;

fn main() {
    server::Server::new()
        .map(|s| s.run())
        .map_err(|e| println!("Failed to initialize server: {:#?}", e))
        .unwrap();
}
