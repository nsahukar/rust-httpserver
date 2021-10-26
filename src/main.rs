#![allow(dead_code)]

mod server;
mod http;
mod site_handler;

use std::env;

use server::Server;
use crate::site_handler::SiteHandler;

// Every file in Rust is treated as a module
// So server.rs creates a module called server
//
// mod server {
// }

fn main() {
    println!("Welcome to rust httpserver!");

    let default_path = format!("{}/public", env!("CARGO_MANIFEST_DIR"));
    let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);
    let server = Server::new("127.0.0.1:3001".to_string());
    server.run(SiteHandler::new(public_path));
}
