mod server;
mod http;

use server::Server;
use http::Method;
use http::Request;

// Every file in Rust is treated as a module
// So server.rs creates a module called server
//
// mod server {
// }

fn main() {
    println!("Welcome to rust httpserver!");

    let get = Method::GET;
    let post = Method::POST;
    let put = Method::PUT;

    let server = Server::new("127.0.0.1:3001".to_string());
    server.run();
}

/*
GET /user?id=10 HTTP/1.1\r\n
HEADERS \r\n
BODY
 */
