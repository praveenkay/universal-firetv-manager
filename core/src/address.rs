use std::{
    fmt,
    net::{IpAddr, SocketAddr},
};

use crate::{ManagerError, Result};

const DEFAULT_ADB_PORT: u16 = 5555;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct TargetAddress(SocketAddr);

impl TargetAddress {
    /// Parses a numeric IPv4 or IPv6 address and optional port.
    ///
    /// # Errors
    ///
    /// Returns [`ManagerError::InvalidTarget`] for hostnames, surrounding whitespace,
    /// malformed addresses, bracketed IPv6 addresses without a port, and port zero.
    pub fn parse(input: &str) -> Result<Self> {
        if input.is_empty() || input.trim() != input {
            return Err(ManagerError::InvalidTarget(input.to_owned()));
        }

        if let Ok(socket) = input.parse::<SocketAddr>() {
            return Self::from_socket(socket, input);
        }

        let ip = input
            .parse::<IpAddr>()
            .map_err(|_| ManagerError::InvalidTarget(input.to_owned()))?;
        Self::from_socket(SocketAddr::new(ip, DEFAULT_ADB_PORT), input)
    }

    fn from_socket(socket: SocketAddr, input: &str) -> Result<Self> {
        if socket.port() == 0 {
            return Err(ManagerError::InvalidTarget(input.to_owned()));
        }
        Ok(Self(socket))
    }

    #[must_use]
    pub fn ip(self) -> IpAddr {
        self.0.ip()
    }

    #[must_use]
    pub fn port(self) -> u16 {
        self.0.port()
    }

    #[must_use]
    pub fn socket_addr(self) -> SocketAddr {
        self.0
    }
}

impl fmt::Display for TargetAddress {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}
