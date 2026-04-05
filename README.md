# rustmultiplexer

A fast TCP reverse proxy with automatic protocol detection.

## Usage
```sh
rustmultiplexer.exe [OPTIONS]
```

## Options

| Flag | Short | Description | Default |
|------|-------|-------------|---------|
| `--addr` | `-a` | Listen address | `0.0.0.0` |
| `--port` | `-p` | Listen port | `8080` |
| `--backend` | `-b` | Backend in format `proto:addr:port` (repeatable) | — |
| `--list-protocols` | `-l` | List all supported protocol types | — |
| `--help` | `-h` | Print help | — |

## Supported Protocols

| Protocol | Description |
|----------|-------------|
| `http` | HTTP/1.1 |
| `http2` | HTTP/2.0 |
| `ssl3` | SSL 3.0 |
| `tls1.0` | TLS 1.0 |
| `tls1.1` | TLS 1.1 |
| `tls1.2` | TLS 1.2 |
| `tls1.3` | TLS 1.3 |
| `ssh1` | SSH 1.x |
| `ssh2` | SSH 2.0 |
| `ftp` | FTP |
| `smtp` | SMTP |
| `pop3` | POP3 |
| `imap` | IMAP |
| `dns` | DNS over TCP |
| `rdp` | RDP |
| `socks4` | SOCKS4 |
| `socks5` | SOCKS5 |
| `mysql` | MySQL |
| `postgresql` | PostgreSQL |
| `tcp` | Raw TCP |

## Examples
```sh
# proxy HTTP traffic to a local web server
rustmultiplexer.exe --addr 0.0.0.0 --port 80 --backend http:127.0.0.1:8080

# multiplex multiple protocols on the same port
rustmultiplexer.exe --port 443 \
  --backend tls1.3:127.0.0.1:8443 \
  --backend ssh2:127.0.0.1:22

# list all supported protocols
rustmultiplexer.exe --list-protocols
```