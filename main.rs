// main.rs
mod transaction;
mod block;
mod blockchain;
mod consensus;
mod node;

use actix_web::{App, HttpServer};
use actix_cors::Cors;

fn main() -> std::io::Result<()> {
    let node = node::Node::new();
    let server_address = format!("{}:{}", node.host, node.port);

    HttpServer::new(move || {
        let cors = Cors::permissive();
        App::new()
            .wrap(cors)
            .app_data(node.clone())
            // Add your API endpoints here
    })
    .bind(&server_address)?
    .run()
}