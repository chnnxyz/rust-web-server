
pub struct Server{
    pub address: String,
    pub port: String,
}

impl Server{
    pub fn new(addr: String, port:String) -> Self{
        Self { 
            address: addr, 
            port: port 
        }
    }

    pub fn default() -> Self {
        Self { 
            address: "127.0.0.1".to_string(), 
            port: "8080".to_string()
        }
    }

    pub fn run(self) {
        println!("Listening on {}:{}", self.address, self.port)
    }
}