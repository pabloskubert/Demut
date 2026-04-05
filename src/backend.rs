use std::io;
use std::fmt;
use std::net::{TcpStream, Shutdown};
use threadpool::ThreadPool;
use crate::enums::Proto;

#[derive(Clone, PartialEq)]
pub struct DemutBackend {
    pub dst_proto: Proto,
    pub dst_addr: String,
    pub dst_port: u16,
    threadpool: ThreadPool
}


impl DemutBackend {
    pub fn new(proto: String, addr: String, port: u16) -> Self {
        let proto_enum_type: Proto = proto.parse().unwrap(); 

        Self {
            dst_proto: proto_enum_type,
            dst_addr: addr,
            dst_port: port,
            threadpool: ThreadPool::new(2),
        }
    }

    pub fn forward(&self, client: &TcpStream) {
        let backend_addr = format!("{}:{}", self.dst_addr, self.dst_port);

        // connect to the backend
        let backend = match TcpStream::connect(&backend_addr) {
            Ok(s) => s,
            Err(_) => {
                eprintln!("Error forwarding to {backend_addr}, target offline..");
                return;
            }
        };

        // clone streams for bidirectional connection
        let mut client_read = client.try_clone().unwrap();
        let mut client_write = client.try_clone().unwrap();
        let mut backend_read = backend.try_clone().unwrap();
        let mut backend_write = backend.try_clone().unwrap();

        // client -> backend
        self.threadpool.execute(move || {
            io::copy(&mut client_read, &mut backend_write).ok();
            backend_write.shutdown(Shutdown::Write).ok();
        });

        // backend -> client
        self.threadpool.execute(move || { 
            io::copy(&mut backend_read, &mut client_write).ok();
            client_write.shutdown(Shutdown::Write).ok();
        });

        self.threadpool.join();
    }
}

impl fmt::Display for DemutBackend {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}{}:{}",
            self.dst_proto.to_string(),
            self.dst_addr,
            self.dst_port.to_string()
        )
    }
} 

