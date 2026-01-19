use serde::{Serialize, Deserialize};
use std::net::SocketAddr;

#[derive(Debug, Serialize, Deserialize)]
pub enum Msg {
    Hello {
        pubkey: [u8; 32],
        nonce: [u8; 32],
        signature: [u8; 64],
        listen: SocketAddr,
    },
    GetPeers,
    Peers(Vec<SocketAddr>),
    Ping(u64),
    Pong(u64),
    Chat {text: String},
}
