use std::fmt;
use std::str::FromStr;


#[derive(Clone, PartialEq)]
pub enum Proto {
    HTTP,
    HTTP2,
    SSL3,
    TLS10,
    TLS11,
    TLS12,
    TLS13,
    SSH1,
    SSH2,
    FTP,
    SMTP,
    POP3,
    IMAP,
    DNS,
    RDP,
    SOCKS4,
    SOCKS5,
    MYSQL,
    POSTGRESQL,
    TCP,
    UNKNOWN,
}

impl fmt::Display for Proto {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Proto::HTTP       => write!(f, "http"),
            Proto::HTTP2      => write!(f, "http2"),
            Proto::SSL3       => write!(f, "ssl3"),
            Proto::TLS10      => write!(f, "tls1.0"),
            Proto::TLS11      => write!(f, "tls1.1"),
            Proto::TLS12      => write!(f, "tls1.2"),
            Proto::TLS13      => write!(f, "tls1.3"),
            Proto::SSH1       => write!(f, "ssh1"),
            Proto::SSH2       => write!(f, "ssh2"),
            Proto::FTP        => write!(f, "ftp"),
            Proto::SMTP       => write!(f, "smtp"),
            Proto::POP3       => write!(f, "pop3"),
            Proto::IMAP       => write!(f, "imap"),
            Proto::DNS        => write!(f, "dns"),
            Proto::RDP        => write!(f, "rdp"),
            Proto::SOCKS4     => write!(f, "socks4"),
            Proto::SOCKS5     => write!(f, "socks5"),
            Proto::MYSQL      => write!(f, "mysql"),
            Proto::POSTGRESQL => write!(f, "postgresql"),
            Proto::TCP        => write!(f, "tcp"),
            Proto::UNKNOWN    => write!(f, "unknown"),
        }
    }
}

impl FromStr for Proto {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "http"       => Ok(Proto::HTTP),
            "http2"      => Ok(Proto::HTTP2),
            "ssl3"       => Ok(Proto::SSL3),
            "tls1.0"     => Ok(Proto::TLS10),
            "tls1.1"     => Ok(Proto::TLS11),
            "tls1.2"     => Ok(Proto::TLS12),
            "tls1.3"     => Ok(Proto::TLS13),
            "ssh1"       => Ok(Proto::SSH1),
            "ssh2"       => Ok(Proto::SSH2),
            "ftp"        => Ok(Proto::FTP),
            "smtp"       => Ok(Proto::SMTP),
            "pop3"       => Ok(Proto::POP3),
            "imap"       => Ok(Proto::IMAP),
            "dns"        => Ok(Proto::DNS),
            "rdp"        => Ok(Proto::RDP),
            "socks4"     => Ok(Proto::SOCKS4),
            "socks5"     => Ok(Proto::SOCKS5),
            "mysql"      => Ok(Proto::MYSQL),
            "postgresql" => Ok(Proto::POSTGRESQL),
            "tcp"        => Ok(Proto::TCP),
            "unknown"    => Ok(Proto::UNKNOWN),
            _            => Err(format!("Unknown protocol: {s}")),
        }
    }
}