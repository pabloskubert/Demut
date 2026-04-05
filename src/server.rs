use crate::backend::DemutBackend;
use crate::enums::Proto;
use std::io;
use std::sync::Arc;
use std::net::TcpListener;
use threadpool::ThreadPool;

pub struct DemutServer {
    backends: Vec<DemutBackend>,
    listen_port: u16,
    listen_addr: String,
    threadpool: ThreadPool,
}



impl DemutServer {
    pub fn new(listen_port: u16, listen_addr: Option<String>) -> Self {
        let addr = listen_addr.unwrap_or_else(|| String::from("0.0.0.0"));
        Self {
            backends: Vec::new(),
            listen_port,
            listen_addr: addr,
            threadpool: ThreadPool::new(20)
        }
    }

    pub fn add_backend(&mut self, backend: DemutBackend) {
        self.backends.push(backend);
    }

    pub fn loop_until_done(self: Arc<Self>) {
        let listen_on = format!("{}:{}", self.listen_addr, self.listen_port);
        let listener = TcpListener::bind(listen_on).unwrap();
        listener.set_nonblocking(true).expect("Cannot set non-blocking");

        // wait for connections
        for stream in listener.incoming() {
            match stream {
                Ok(s) => {
                    let self_clone = Arc::clone(&self);
                    self.threadpool.execute(move || {
                        let mut buf = [0; 28];    
                        s.peek(&mut buf).unwrap();
                        let protocol_string = match &buf {
                            b if b.starts_with(b"HTTP/1.1") => "http",
                            b if b.starts_with(b"HTTP/2.0") => "http2",
                            b if b.starts_with(b"GET ")     => "http",
                            b if b.starts_with(b"POST ")    => "http",
                            b if b.starts_with(b"PUT ")     => "http",
                            b if b.starts_with(b"PATCH ")   => "http",
                            b if b.starts_with(b"DELETE ")  => "http",
                            b if b.starts_with(b"HEAD ")    => "http",
                            b if b.starts_with(b"CONNECT")  => "http",
                            b if b.starts_with(b"TRACE")    => "http",
                            b if b.starts_with(b"OPTIONS ") => "http",
                            b if b.starts_with(&[0x16, 0x03, 0x00]) => "ssl3",
                            b if b.starts_with(&[0x16, 0x03, 0x01]) => "tls1.0",
                            b if b.starts_with(&[0x16, 0x03, 0x02]) => "tls1.1",
                            b if b.starts_with(&[0x16, 0x03, 0x03]) => "tls1.2",
                            b if b.starts_with(&[0x16, 0x03, 0x04]) => "tls1.3",
                            b if b.starts_with(b"SSH-2.0") => "ssh2",
                            b if b.starts_with(b"SSH-1.")  => "ssh1",
                            b if b.starts_with(b"220 ")  => "ftp",   
                            b if b.starts_with(b"USER ") => "ftp",   
                            b if b.starts_with(b"EHLO") => "smtp",
                            b if b.starts_with(b"HELO") => "smtp",
                            b if b.starts_with(b"220 ") => "smtp",   
                            b if b.starts_with(b"+OK")  => "pop3",
                            b if b.starts_with(b"* OK") => "imap",
                            b if b[2] & 0x80 == 0 && b[2] & 0x78 == 0 => "dns",
                            b if b.starts_with(&[0x03, 0x00]) => "rdp",
                            b if b.starts_with(&[0x05]) => "socks5",
                            b if b.starts_with(&[0x04]) => "socks4",
                            b if b.starts_with(&[0x4a, 0x00, 0x00, 0x00]) => "mysql",
                            b if b.starts_with(&[0x00, 0x00, 0x00]) => "postgresql",
                            _ => "unknown",
                        }; 

                        let protocol = protocol_string.parse().unwrap_or(Proto::HTTP);
                        for backend in &self_clone.backends {
                            // match Back<=>Protocol
                            if backend.dst_proto == protocol {
                                backend.forward(&s);
                                break;
                            }
                        }
                    });
                }
                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                    continue;
                }
                Err(e) => panic!("Io Error: {e}"),
            }
        }
    }
}
