use std::os::unix::net::SocketAddr;

use message_io::{network::Endpoint, node};

pub struct ServerConnection((Endpoint, SocketAddr));

pub struct ClientNetworkManager {

}

impl ClientNetworkManager {
    pub fn new() -> Self {
        let (handler, listener) = node::split::<()>();

        Self {

        }
    }
}