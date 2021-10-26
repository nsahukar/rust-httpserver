use crate::http::{ParseError, Response, StatusCode, request::Request};
use std::{convert::TryFrom, io::Read, net::TcpListener};

pub trait Handler {
    fn handle_request(&mut self, request: &Request) -> Response;
    fn handle_bad_request(&mut self, err: &ParseError) -> Response {
        println!("Failed to parse the request: {}", err);
        Response::new(StatusCode::BadRequest, None)
    }
}

pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self { addr }
    }

    pub fn run(self, mut handler: impl Handler) {
        println!("Listening on {}", self.addr);

        let listener = TcpListener::bind(&self.addr).unwrap();

        // loop keyword for infinite loop
        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    let mut buf = [0; 1024];
                    match stream.read(&mut buf) {
                        Ok(_) => {
                            println!("Received a request: {}", String::from_utf8_lossy(&buf));
                            let response = match Request::try_from(&buf[..]) {
                                Ok(request) => handler.handle_request(&request),
                                Err(e) => handler.handle_bad_request(&e),
                            };

                            if let Err(e) = response.send(&mut stream) {
                                println!("Failed to send response: {}", e);
                            }
                        }
                        Err(e) => println!("Failed to read from connection: {}", e),
                    }
                    println!("Ok");
                }
                Err(err) => println!("Failed to establish a connection: {}", err),
            }

            //             let res = listener.accept();
            //             if res.is_err() {
            //                 continue;
            //             }
            //
            //             let (stream, in_addr) = res.unwrap();
        }
    }
}

// creating custom types
// pub type Result<T> = result::Result<T, Error>;
//
// pub struct Error {
//     repr: Repr,
// }
