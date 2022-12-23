use std::net::{ TcpListener, TcpStream };
use std::io::Read;

pub struct Server{
    pub address: String,
    pub port: String,
}

impl Server{
    pub fn new(addr: String, port:String) -> Self{
        Self { 
            address: addr, 
            port: port,
        }
    }

    pub fn default() -> Self {
        Self { 
            address: "127.0.0.1".to_string(), 
            port: "8080".to_string()

        }
    }

    pub fn run(&self) {
        let addr = format!("{}:{}", &self.address, &self.port);
        let listener = TcpListener::bind(addr).unwrap();
        println!("Listening on {}:{}", &self.address, &self.port);

        // loop is a keyword equivalent to while true
        'main: loop {
            match listener.accept() {
                Ok((mut stream, addr)) => {

                    let mut buffer = [0u8; 1024];
                    //add connection to list
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Received Request: {}",
                                String::from_utf8_lossy(&buffer)
                            )
                        },
                        Err(e) => {
                            println!("Stream could not be read:");
                            println!("{}", e)
                        }
                    }
                },
                Err(e) => {
                    println!("Connection refused");
                    println!("{}", e);
                }
            }
        }
    }
}