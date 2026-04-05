mod backend;
mod tests;
mod enums;
mod server;
use std::sync::Arc;
use clap::Parser;
use backend::DemutBackend;
use server::DemutServer;
extern crate threadpool;

#[derive(Parser, Debug)]
#[command(name = "demut", about = "TCP reverse Proxy")]
struct Args {
    // listen addr
    #[arg(short, long, default_value = "0.0.0.0")]
    addr: String,

    // listen port
    #[arg(short, long, default_value_t = 8080)]
    port: u16,

    // backends
    #[arg(short, long)]
    backend: Vec<String>,

    /// List all supported protocol types
    #[arg(short, long)]
    list_protocols: bool,
}

fn main() {
    let args = Args::parse(); 
    if args.list_protocols {
        println!("Supported protocols:");
        println!("  http       - HTTP/1.1");
        println!("  http2      - HTTP/2.0");
        println!("  ssl3       - SSL 3.0");
        println!("  tls1.0     - TLS 1.0");
        println!("  tls1.1     - TLS 1.1");
        println!("  tls1.2     - TLS 1.2");
        println!("  tls1.3     - TLS 1.3");
        println!("  ssh1       - SSH 1.x");
        println!("  ssh2       - SSH 2.0");
        println!("  ftp        - FTP");
        println!("  smtp       - SMTP");
        println!("  pop3       - POP3");
        println!("  imap       - IMAP");
        println!("  dns        - DNS over TCP");
        println!("  rdp        - RDP");
        println!("  socks4     - SOCKS4");
        println!("  socks5     - SOCKS5");
        println!("  mysql      - MySQL");
        println!("  postgresql - PostgreSQL");
        println!("  tcp        - Raw TCP");
        println!("\nUsage example:");
        println!("  --backend http:127.0.0.1:8080");
        println!("  --backend ssh2:10.0.0.1:22");
        std::process::exit(0);
    }

    
    let mut server = DemutServer::new(args.port, Some(args.addr.clone()));
    if args.backend.len() == 0 {
        eprintln!("Specify at least one backend!!");
        std::process::exit(0);
    }

    for backend_string in &args.backend {
        let backend_parts: Vec<&str> = backend_string.split(":").collect();
        if backend_parts.is_empty() {
            eprintln!("Invalid backend format: {backend_string} Valid format: --backend http:127.0.0.1:8085 --backend proto:addr:port");
            return;
        }
        if backend_parts.len() != 3 {
            eprintln!("Invalid backend format: {backend_string} Valid format: --backend http:127.0.0.1:8085 --backend proto:addr:port");
            return;
        }

        // parse backend
        let proto = backend_parts[0];
        let addr = backend_parts[1];
        let port: u16 = backend_parts[2].parse().expect("port must be a number");

        // add new backend
        server.add_backend(DemutBackend::new(proto.to_string(), addr.to_string(), port));
    }

    let server = Arc::new(server);
    println!("Listenning on: {}:{}", args.addr, args.port);
    println!("Backends: {:?}", args.backend);
    server.loop_until_done();
}
