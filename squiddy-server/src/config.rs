use std::env::{ Args, args };
use std::net::{ IpAddr, Ipv4Addr, SocketAddr };

const DEFAULT_LISTEN_PORT: u16 = 7979;
const DEFAULT_LISTEN_ADDRESS: (u8, u8, u8, u8) = (0, 0, 0, 0);

pub struct ServerConfig {
    pub listen_address: SocketAddr
}

impl Default for ServerConfig {
    fn default() -> Self {
        let a = DEFAULT_LISTEN_ADDRESS;
        Self {
            listen_address: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(a.0, a.1, a.2, a.3)), DEFAULT_LISTEN_PORT)
        }
    }
}

impl From<Args> for ServerConfig {

    fn from(args: Args) -> Self {
        let default_config = Self::default();

        for arg in args {
            match arg {
                //"-p" => args.next().unwrap_or(format!("{}", DEFAULT_LISTEN_PORT)),
                _ => ()
            }
        }

        default_config
    }
}
