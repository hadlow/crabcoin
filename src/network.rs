use libp2p::{Multiaddr, Transport, tcp::TcpConfig};

pub struct Network
{
    config: String,
}

impl Network
{
    pub fn new() -> Self
    {
        let tcp = TcpConfig::new();
        let addr: Multiaddr = "/ip4/98.97.96.95/tcp/20500".parse().expect("invalid multiaddr");
        let _conn = tcp.dial(addr);

        Self
        {
            config: "none".to_string(),
        }
    }
}
