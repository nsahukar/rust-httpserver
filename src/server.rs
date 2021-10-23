use std::net::TcpListener;

pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self { addr }
    }

    pub fn run(self) {
        println!("Listening on {}", self.addr);

        let listener = TcpListener::bind(&self.addr).unwrap();

        // loop keyword for infinite loop
        loop {
            match listener.accept() {
                Ok((stream, _)) => {
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
