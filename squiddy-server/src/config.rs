use std::env::{ Args, args };
use std::net::{ IpAddr, Ipv4Addr };

const DEFAULT_LISTEN_PORT: u16 = 7979;
const DEFAULT_LISTEN_ADDRESS: (u8, u8, u8, u8) = (0, 0, 0, 0);

#[derive(Clone)]
pub struct ServerConfig {
    pub listen_address: IpAddr,
    pub listen_port: u16
}

impl ServerConfig {
    pub fn from_args() -> ServerConfig {
        ServerConfig::from(args())
    }
}

impl Default for ServerConfig {
    /// Create a new `ServerConfig` instance with reasonable default values.
    fn default() -> Self {
        let a = DEFAULT_LISTEN_ADDRESS;

        Self {
            listen_address: IpAddr::V4(Ipv4Addr::new(a.0, a.1, a.2, a.3)),
            listen_port:  DEFAULT_LISTEN_PORT
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
