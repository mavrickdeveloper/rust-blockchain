// main.rs
// Import modules
mod transaction;
mod block;
mod blockchain;
mod consensus;
mod node;

// Import dependencies
use actix_web::{App, HttpServer};
use actix_cors::Cors;

fn main() -> std::io::Result<()> {
    // Create a new node instance
    let node = node::Node::new();
    // Format server address using host and port
    let server_address = format!("{}:{}", node.host, node.port);

    // Start the HTTP server
    HttpServer::new(move || {
        // Set up CORS (Cross-Origin Resource Sharing) for development purposes
        let cors = Cors::permissive();
        // Configure the Actix web application
        App::new()
            .wrap(cors) // Apply the CORS middleware
            .app_data(node.clone()) // Pass the node instance to the application
            // Add your API endpoints here
    })
    .bind(&server_address)? // Bind the server to the address
    .run() // Run the server
}
